use std::time::Instant;

use crate::graphics::Graphics;
use wayland_client::protocol::wl_display;
use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::{wl_callback, wl_compositor, wl_registry, wl_surface},
};
use wayland_protocols_wlr::layer_shell::v1::client::{zwlr_layer_shell_v1, zwlr_layer_surface_v1};

pub struct AppState {
    pub graphics: Option<Graphics>,
    pub start_time: Instant,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub closed: bool,
    // Wayland objects
    pub display: wl_display::WlDisplay,
    pub compositor: Option<(wl_compositor::WlCompositor, u32)>,
    pub layer_shell: Option<(zwlr_layer_shell_v1::ZwlrLayerShellV1, u32)>,
    pub surface: Option<wl_surface::WlSurface>,
    pub layer_surface: Option<zwlr_layer_surface_v1::ZwlrLayerSurfaceV1>,
}

impl AppState {
    pub fn new(
        display: wl_display::WlDisplay,
        vertex_shader: String,
        fragment_shader: String,
    ) -> Self {
        AppState {
            graphics: None,
            start_time: Instant::now(),
            vertex_shader,
            fragment_shader,
            closed: false,
            display,
            compositor: None,
            layer_shell: None,
            surface: None,
            layer_surface: None,
        }
    }

    pub fn is_running(&self) -> bool {
        !self.closed
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_registry::Event::Global {
                name,
                interface,
                version,
            } => {
                let _span_guard =
                    tracing::trace_span!("wl_registry::Event::Global", name, interface, version)
                        .entered();
                match interface.as_str() {
                    "wl_compositor" => {
                        tracing::info!("Compositor found: {} (version {})", name, version);
                        state.compositor = Some((registry.bind(name, version, qh, ()), name));
                    }
                    "zwlr_layer_shell_v1" => {
                        tracing::info!("LayerShell found: {} (version {})", name, version);
                        state.layer_shell = Some((registry.bind(name, version, qh, ()), name));
                    }
                    _ => {}
                }
            }
            wl_registry::Event::GlobalRemove { name } => {
                let _span_guard =
                    tracing::trace_span!("wl_registry::Event::GlobalRemove", name).entered();
                if let Some((_, compositor_name)) = &state.compositor {
                    if *compositor_name == name {
                        tracing::warn!("Compositor {} removed", name);
                        state.compositor = None;
                    }
                }
                if let Some((_, layer_shell_name)) = &state.layer_shell {
                    if *layer_shell_name == name {
                        tracing::warn!("LayerShell {} removed", name);
                        state.layer_shell = None;
                    }
                }
            }
            _ => {}
        };
        return;
    }
}

impl Dispatch<zwlr_layer_shell_v1::ZwlrLayerShellV1, ()> for AppState {
    fn event(
        _state: &mut Self,
        _layer_shell: &zwlr_layer_shell_v1::ZwlrLayerShellV1,
        _event: zwlr_layer_shell_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        // Do nothing: LayerShell never dispatches events.
    }
}

impl Dispatch<zwlr_layer_surface_v1::ZwlrLayerSurfaceV1, ()> for AppState {
    fn event(
        state: &mut Self,
        surface: &zwlr_layer_surface_v1::ZwlrLayerSurfaceV1,
        event: zwlr_layer_surface_v1::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            zwlr_layer_surface_v1::Event::Configure {
                serial,
                width,
                height,
            } => {
                let _span_guard = tracing::trace_span!(
                    "zwlr_layer_surface_v1::Event::Configure",
                    serial,
                    width,
                    height
                );
                tracing::info!(
                    "Layer surface configured: serial={}, width={}, height={}",
                    serial,
                    width,
                    height
                );
                surface.ack_configure(serial);
                if let Some(surface) = state.surface.as_ref()
                    && state.graphics.is_none()
                {
                    let graphics = Graphics::new(
                        &state.display,
                        &surface,
                        width,
                        height,
                        state.vertex_shader.as_str(),
                        state.fragment_shader.as_str(),
                    );
                    let elapsed = state.start_time.elapsed().as_secs_f32();
                    graphics.render(elapsed);
                    tracing::info!("Rendering initial frame");
                    let _callback = surface.frame(qh, ());
                    surface.commit();
                    state.graphics = Some(graphics);
                } else if let Some(graphics) = state.graphics.as_mut() {
                    graphics.resize(width, height);
                }
            }
            zwlr_layer_surface_v1::Event::Closed => {
                let _span_guard =
                    tracing::trace_span!("zwlr_layer_surface_v1::Event::Closed").entered();
                tracing::info!("Layer surface closed");
                state.closed = true;
            }
            _ => (),
        }
    }
}

impl Dispatch<wl_callback::WlCallback, ()> for AppState {
    fn event(
        state: &mut Self,
        _callback: &wl_callback::WlCallback,
        event: wl_callback::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_callback::Event::Done { .. } => {
                let _span_guard = tracing::trace_span!("wl_callback::Event::Done").entered();
                // Frame callback done, can be used to trigger next render
                if let (Some(graphics), Some(surface)) =
                    (state.graphics.as_ref(), state.surface.as_ref())
                {
                    let elapsed = state.start_time.elapsed().as_secs_f32();
                    tracing::trace!("Rendering frame at elapsed time: {}", elapsed);
                    graphics.render(elapsed);
                    let _callback = surface.frame(qh, ());
                    surface.commit();
                } else {
                    tracing::trace!("No graphics or surface available for rendering.");
                }
            }
            _ => {
                // Do nothing
            }
        }
    }
}

impl Dispatch<wl_surface::WlSurface, ()> for AppState {
    fn event(
        _state: &mut Self,
        _surface: &wl_surface::WlSurface,
        event: wl_surface::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_surface::Event::Enter { .. } => {
                // Do nothing: Cursor enter event is not needed for background.
            }
            wl_surface::Event::Leave { .. } => {
                // Do nothing: Cursor leave event is not needed for background.
            }
            wl_surface::Event::PreferredBufferScale { factor } => {
                // todo: HiDPI support
                tracing::debug!("TODO: Handle preferred buffer scale factor: {}", factor);
            }
            wl_surface::Event::PreferredBufferTransform { transform } => {
                // todo: Device rotation support
                tracing::debug!("TODO: Handle preferred buffer transform: {:?}", transform);
            }
            _ => {
                // Do nothing
            }
        }
    }
}

impl Dispatch<wl_compositor::WlCompositor, ()> for AppState {
    fn event(
        _state: &mut Self,
        _compositor: &wl_compositor::WlCompositor,
        _event: wl_compositor::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        // Do nothing: Compositor never dispatches events.
    }
}
