use super::*;

#[derive(Debug, Error)]
pub enum RenderingPipelineJsonConfigurationReadError {
    #[error("JSON file I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to deserialize rendering pipeline configuration: {0}")]
    SerdeError(#[from] serde_json::Error),
}

/// Deserializer for rendering pipeline configuration written in JSON.
///
/// Design of public rendering pipeline configuration interface is optimized for Nix
/// and I DO NOT recommend to use JSON format.
///
/// Configuring rendering pipeline in JSON may cause readability, maintainability,
/// reusability and runtime cache efficiency issues.
pub fn read_rendering_pipeline_configuration_from_json_file(
    json_file_path: &Path,
) -> Result<ConfRenderPass, RenderingPipelineJsonConfigurationReadError> {
    let json = std::fs::read_to_string(json_file_path)?;
    let render_pass: ConfRenderPass = serde_json::from_str(&json)?;
    Ok(render_pass)
}
