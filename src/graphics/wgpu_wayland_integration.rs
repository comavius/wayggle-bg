use raw_window_handle::{
    WaylandDisplayHandle,
    WaylandWindowHandle,
};
use std::{
    ffi::c_void,
    ptr::NonNull,
};
use wayland_client::{
    Proxy,
    protocol::{
        wl_display,
        wl_surface,
    },
};
use wgpu::{
    CreateSurfaceError,
    Instance,
    Surface,
};

/// Helper to abstract surface creation. This leads to backend-agnostic
/// rendering pipelines.
pub fn create_surface_generator<'a>(
    display: &'a wl_display::WlDisplay,
    surface: &'a wl_surface::WlSurface,
) -> Result<
    impl Fn(Instance) -> Result<Surface<'a>, CreateSurfaceError> + 'a,
    wgpu::Error,
>
{
    Ok(move |instance: Instance| {
        let raw_display_ptr: NonNull<c_void> = NonNull::new(
            display
                .id()
                .as_ptr() as *const _ as *mut c_void,
        )
        .unwrap();
        let raw_window_ptr = NonNull::new(
            surface
                .id()
                .as_ptr() as *const _ as *mut c_void,
        )
        .unwrap();
        // Recreate the raw handles each call so we don't move a captured
        // non-Copy value.
        let wayland_display_handle = WaylandDisplayHandle::new(raw_display_ptr);
        let wayland_window_handle = WaylandWindowHandle::new(raw_window_ptr);
        let raw_display_handle = raw_window_handle::RawDisplayHandle::Wayland(
            wayland_display_handle,
        );
        let raw_window_handle =
            raw_window_handle::RawWindowHandle::Wayland(wayland_window_handle);

        let unsafe_surface = wgpu::SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: raw_display_handle,
            raw_window_handle: raw_window_handle,
        };
        unsafe { instance.create_surface_unsafe(unsafe_surface) }
    })
}
