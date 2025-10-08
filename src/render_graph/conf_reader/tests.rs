use super::*;

#[test]
#[tracing_test::traced_test]
fn test_read_render_graph_configuration_from_nix_file() {
    let here = file!();
    let here_path = PathBuf::from(here).parent().unwrap().to_path_buf();
    let test_nix_file_path = here_path.join("test_nix").join("renderGraph.nix");
    let nix_lib_dir = here_path.join("nix");
    let default_resolution = Resolution {
        height: 1080,
        width: 1920,
    };
    let render_pass = read_render_graph_configuration_from_nix_file(
        &test_nix_file_path,
        &nix_lib_dir,
        default_resolution,
    );
    assert!(render_pass.is_ok());
}
