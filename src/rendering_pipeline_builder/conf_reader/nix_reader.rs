use super::*;

#[derive(Debug, Error)]
pub enum RenderingPipelineNixConfigurationReadError
{
    #[error("Invalid nix file path: {0}")]
    InvalidNixFilePath(PathBuf),
    #[error("Internal nix library not found: {0}")]
    InternalNixLibraryNotFound(PathBuf),
    #[error("Failed to evaluate nix expression: {0:?}")]
    NixEvaluationError(Vec<snix_eval::Error>),
    #[error("Failed to deserialize rendering pipeline configuration: {0}")]
    SerdeError(serde_json::Error),
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Reader, evaluator and deserializer for rendering pipeline
/// configuration written in Nix.
///
/// Encapsulating `tvix-eval` interfaces.
pub fn read_rendering_pipeline_configuration_from_nix_file(
    nix_file_path: &Path,
    nix_lib_dir: &Path,
    default_resolution: ConfResolution,
) -> Result<ConfRenderPass, RenderingPipelineNixConfigurationReadError>
{
    // Both nix_file_path and nix_lib_dir must be canonicalized
    // to evaluate the nix expression without file path.
    let canonical_nix_file_path = nix_file_path
        .canonicalize()
        .map_err(|_| {
            RenderingPipelineNixConfigurationReadError::InvalidNixFilePath(
                nix_file_path.to_path_buf(),
            )
        })?;
    let canonical_nix_lib_dir = nix_lib_dir
        .canonicalize()
        .map_err(|_| {
            RenderingPipelineNixConfigurationReadError::InternalNixLibraryNotFound(
                nix_lib_dir.to_path_buf(),
            )
        })?;
    // Passing height and width of monitor resolution because it is
    // machine dependent.
    //
    // Intentionally using `builtins.toJSON` instead of `snix-eval`
    // to avoid troubles with purity.
    // `snix_eval::from_str_with_config` won't work!
    let nix_expression = format!(
        r"
        builtins.toJSON
        (
            import {}
            (
            import {}
            {{ 
            defaultResolution = {{
                height = {};
                width = {};
                }};
                }}
            )
        )
        ",
        canonical_nix_file_path.to_string_lossy(),
        canonical_nix_lib_dir.to_string_lossy(),
        default_resolution.height,
        default_resolution.width
    );
    tracing::debug!("Nix expression: {}", nix_expression);
    let evaluation = snix_eval::Evaluation::builder_impure()
        .mode(snix_eval::EvalMode::Lazy)
        .build();
    let result = evaluation.evaluate(&nix_expression, None);
    if !result
        .errors
        .is_empty()
    {
        return Err(
            RenderingPipelineNixConfigurationReadError::NixEvaluationError(
                result.errors,
            ),
        );
    }
    match result.value {
        Some(value) => {
            let json_string_escaped = value.to_string();
            let json_string = json_string_escaped
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .ok_or_else(|| {
                    RenderingPipelineNixConfigurationReadError::InternalError(
                        "Failed to strip quotes from JSON string".to_string(),
                    )
                })?
                .replace("\\n", "\n")
                .replace("\\\"", "\"")
                .replace("\\\\", "\\");
            let render_pass: ConfRenderPass = serde_json::from_str(
                &json_string,
            )
            .map_err(RenderingPipelineNixConfigurationReadError::SerdeError)?;
            Ok(render_pass)
        }
        None => Err(RenderingPipelineNixConfigurationReadError::InternalError(
            "Nix evaluation returned neither value nor error".to_string(),
        )),
    }
}
