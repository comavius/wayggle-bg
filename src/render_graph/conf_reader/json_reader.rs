use super::*;

#[derive(Debug, Error)]
pub enum RenderGraphJsonConfigurationReadError {
    #[error("JSON file I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to deserialize render graph configuration: {0}")]
    SerdeError(#[from] serde_json::Error),
}

/// Deserializer for render graph configuration written in JSON.
///
/// Design of public render graph configuration interface is optimized for Nix
/// and I DO NOT recommend to use JSON format.
///
/// Configuring render graph in JSON may cause readability, maintainability,
/// reusability and runtime cache efficiency issues.
pub fn read_render_graph_configuration_from_json_file(
    json_file_path: &Path,
) -> Result<ConfRenderPass, RenderGraphJsonConfigurationReadError> {
    let json = std::fs::read_to_string(json_file_path)?;
    let render_pass: ConfRenderPass = serde_json::from_str(&json)?;
    Ok(render_pass)
}
