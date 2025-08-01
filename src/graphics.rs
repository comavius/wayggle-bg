use khronos_egl as egl;
use std::ffi::CString;

use glow::HasContext;

use wayland_client::protocol::wl_surface;
use wayland_client::{Proxy, protocol::wl_display};
use wayland_egl as wegl;

use wayland_client::protocol::wl_shm_pool;
use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::{wl_buffer, wl_callback, wl_compositor, wl_registry, wl_shm},
};
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{self, ZwlrLayerShellV1},
    zwlr_layer_surface_v1::{self, ZwlrLayerSurfaceV1},
};

/// Struct to manage EGL/OpenGL ES initialization and rendering using `glow`
pub struct Graphics {
    egl_instance: egl::Instance<egl::Static>,
    egl_display: egl::Display,
    egl_context: egl::Context,
    egl_surface: egl::Surface,
    wl_egl_surface: wegl::WlEglSurface,
    width: i32,
    height: i32,

    gl: glow::Context,

    shader_program: glow::Program,
    vbo: glow::Buffer,
    time_uniform_location: glow::UniformLocation,
}

impl Graphics {
    pub fn render(&self, elapsed: f32) {
        self.egl_instance
            .make_current(
                self.egl_display,
                Some(self.egl_surface),
                Some(self.egl_surface),
                Some(self.egl_context),
            )
            .unwrap();

        // glow functions must be called inside an unsafe block
        unsafe {
            self.gl.viewport(0, 0, self.width, self.height);
            // Pass the program handle wrapped in Option
            self.gl.use_program(Some(self.shader_program));

            // Pass a reference to the UniformLocation
            self.gl
                .uniform_1_f32(Some(&self.time_uniform_location), elapsed);

            // Draw the rectangle
            self.gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
        }

        self.egl_instance
            .swap_buffers(self.egl_display, self.egl_surface)
            .unwrap();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width as i32;
        self.height = height as i32;
        self.wl_egl_surface
            .resize(width as i32, height as i32, 0, 0);
        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn new(
        display: &wl_display::WlDisplay,
        surface: &wl_surface::WlSurface,
        width: u32,
        height: u32,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Self {
        let egl_instance = egl::Instance::<egl::Static>::new(egl::Static);

        let egl_display = unsafe {
            egl_instance
                .get_display(display.id().as_ptr() as egl::NativeDisplayType)
                .unwrap()
        };

        egl_instance.initialize(egl_display).unwrap();
        egl_instance.bind_api(egl::OPENGL_ES_API).unwrap();

        let attributes = [
            egl::RED_SIZE,
            8,
            egl::GREEN_SIZE,
            8,
            egl::BLUE_SIZE,
            8,
            egl::SURFACE_TYPE,
            egl::WINDOW_BIT,
            egl::RENDERABLE_TYPE,
            egl::OPENGL_ES2_BIT,
            egl::NONE,
        ];
        let config = egl_instance
            .choose_first_config(egl_display, &attributes)
            .unwrap()
            .unwrap();

        let context_attributes = [egl::CONTEXT_CLIENT_VERSION, 2, egl::NONE];
        let egl_context = egl_instance
            .create_context(egl_display, config, None, &context_attributes)
            .unwrap();

        let wl_egl_surface =
            wegl::WlEglSurface::new(surface.id(), width as i32, height as i32).unwrap();

        let egl_surface = unsafe {
            egl_instance
                .create_window_surface(
                    egl_display,
                    config,
                    wl_egl_surface.ptr() as egl::NativeWindowType,
                    None,
                )
                .unwrap()
        };

        egl_instance
            .make_current(
                egl_display,
                Some(egl_surface),
                Some(egl_surface),
                Some(egl_context),
            )
            .unwrap();

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                egl_instance.get_proc_address(s).unwrap() as *const _
            })
        };

        let shader_program = unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let vs = gl
                .create_shader(glow::VERTEX_SHADER)
                .expect("Cannot create vertex shader");
            gl.shader_source(vs, &vertex_shader);
            gl.compile_shader(vs);
            if !gl.get_shader_compile_status(vs) {
                panic!("{}", gl.get_shader_info_log(vs));
            }
            gl.attach_shader(program, vs);

            let fs = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .expect("Cannot create fragment shader");
            gl.shader_source(fs, &fragment_shader);
            gl.compile_shader(fs);
            if !gl.get_shader_compile_status(fs) {
                panic!("{}", gl.get_shader_info_log(fs));
            }
            gl.attach_shader(program, fs);

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            gl.detach_shader(program, fs);
            gl.delete_shader(fs);
            gl.detach_shader(program, vs);
            gl.delete_shader(vs);

            gl.use_program(Some(program));
            program
        };

        let time_uniform_location = unsafe {
            gl.get_uniform_location(shader_program, "u_time")
                .expect("u_time uniform not found")
        };

        let vbo = unsafe {
            let vertices: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
            let vertices_u8: &[u8] = core::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * std::mem::size_of::<f32>(),
            );

            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);

            let pos_attr_loc = gl
                .get_attrib_location(shader_program, "a_position")
                .unwrap();
            gl.enable_vertex_attrib_array(pos_attr_loc);
            gl.vertex_attrib_pointer_f32(pos_attr_loc, 2, glow::FLOAT, false, 0, 0);

            vbo
        };

        Graphics {
            egl_instance,
            egl_display,
            egl_context,
            egl_surface,
            wl_egl_surface,
            width: width as i32,
            height: height as i32,
            gl,
            shader_program,
            vbo,
            time_uniform_location,
        }
    }
}
impl Drop for Graphics {
    fn drop(&mut self) {
        unsafe {
            // 1. Unbind EGL context
            self.egl_instance
                .make_current(self.egl_display, None, None, None)
                .unwrap();

            // 2. glow (OpenGL) resources cleanup
            self.gl.delete_program(self.shader_program);
            self.gl.delete_buffer(self.vbo);

            // 3. Destroy EGL surface
            self.egl_instance
                .destroy_surface(self.egl_display, self.egl_surface)
                .unwrap();

            // 4. Destroy EGL context
            self.egl_instance
                .destroy_context(self.egl_display, self.egl_context)
                .unwrap();

            // 5. Terminate connection to EGL display
            self.egl_instance.terminate(self.egl_display).unwrap();
        }
    }
}
