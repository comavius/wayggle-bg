mod app_state;
mod graphics;
use wayland_client::Connection;
use wayland_protocols_wlr::layer_shell::v1::client::{zwlr_layer_shell_v1, zwlr_layer_surface_v1};
fn main() {
    let conn = Connection::connect_to_env().unwrap();
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let display = conn.display();
    display.get_registry(&qh, ());

    let mut app_state = app_state::AppState::new(
        display.clone(),
        include_str!("../shader/example-vert.glsl").to_string(),
        include_str!("../shader/example-frag.glsl").to_string(),
    );

    println!("Starting initial roundtrip to get globals...");
    event_queue.roundtrip(&mut app_state).unwrap();
    println!("Globals received.");

    let compositor = app_state.compositor.as_ref().expect("Compositor not found");
    let surface = compositor.create_surface(&qh, ());
    app_state.surface = Some(surface.clone());

    let layer_shell = app_state
        .layer_shell
        .as_ref()
        .expect("Layer shell not found");
    let layer_surface = layer_shell.get_layer_surface(
        &surface,
        None,
        zwlr_layer_shell_v1::Layer::Bottom,
        "egl_background".to_string(),
        &qh,
        (),
    );
    layer_surface.set_exclusive_zone(-1);
    layer_surface.set_anchor(
        zwlr_layer_surface_v1::Anchor::Top
            | zwlr_layer_surface_v1::Anchor::Bottom
            | zwlr_layer_surface_v1::Anchor::Left
            | zwlr_layer_surface_v1::Anchor::Right,
    );
    layer_surface.set_size(0, 0);
    app_state.layer_surface = Some(layer_surface);

    surface.commit();
    println!("Initial commit done. Waiting for configure event...");

    while app_state.is_running() {
        event_queue.blocking_dispatch(&mut app_state).unwrap();
    }

    println!("Exiting.");
}
