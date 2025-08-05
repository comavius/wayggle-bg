pub fn hyprland_get_cursor() -> (f32, f32) {
    // Get the cursor position from the Hyprland API
    let output = std::process::Command::new("hyprctl")
        .arg("cursorpos")
        .output()
        .expect("Failed to execute hyprctl command");
    if !output.status.success() {
        tracing::error!(
            "Failed to get cursor position: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }

    // format is: "x, y"
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = stdout.trim().split(", ").collect();
    if parts.len() != 2 {
        tracing::error!("Unexpected output format from hyprctl: {}", stdout);
        std::process::exit(1);
    }
    let x: f32 = parts[0].parse().expect("Failed to parse x coordinate");
    let y: f32 = parts[1].parse().expect("Failed to parse y coordinate");
    (x, y)
}
