#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tvix_eval::EvalMode;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RenderPass {
    TexturePass(TexturePass),
    TransformPass(TransformPass),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TexturePass {
    pub name: String,
    pub resolution: Resolution,
    pub path: PathBuf,
    pub format: TextureFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TextureFormat {
    Png,
    Jpeg,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TransformPass {
    pub name: String,
    pub resolution: Resolution,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub inputs: HashMap<String, RenderPass>,
    pub builtin_names: BuiltinsNames,
    pub outputs: Vec<String>,
    pub frame_rate: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BuiltinsNames {
    pub time: String,
    pub resolution: String,
    pub mouse: String,
    pub position: String,
}

#[derive(Debug, Error)]
pub enum RenderGraphConfigurationReadError {
    #[error("Invalid nix file path: {0}")]
    InvalidNixFilePath(PathBuf),
    #[error("Internal nix library not found: {0}")]
    InternalNixLibraryNotFound(PathBuf),
    #[error("Failed to evaluate nix expression: {0:?}")]
    NixEvaluationError(Vec<tvix_eval::Error>),
    #[error("Failed to deserialize render graph configuration: {0}")]
    SerdeError(serde_json::Error),
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub fn read_render_graph_configuration_from_nix_file(
    nix_file_path: &Path,
    nix_lib_dir: &Path,
    default_resolution: Resolution,
) -> Result<RenderPass, RenderGraphConfigurationReadError> {
    // Both nix_file_path and nix_lib_dir must be canonicalized
    // to evaluate the nix expression without file path.
    let canonical_nix_file_path = nix_file_path.canonicalize().map_err(|_| {
        RenderGraphConfigurationReadError::InvalidNixFilePath(nix_file_path.to_path_buf())
    })?;
    let canonical_nix_lib_dir = nix_lib_dir.canonicalize().map_err(|_| {
        RenderGraphConfigurationReadError::InternalNixLibraryNotFound(nix_lib_dir.to_path_buf())
    })?;
    // Passing height and width of monitor resolution because it is machine dependent.
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
        /*
         */
        canonical_nix_file_path.to_string_lossy(),
        canonical_nix_lib_dir.to_string_lossy(),
        default_resolution.height,
        default_resolution.width
    );
    tracing::debug!("Nix expression: {}", nix_expression);
    let evaluation = tvix_eval::Evaluation::builder_impure()
        .mode(EvalMode::Lazy)
        .build();
    let result = evaluation.evaluate(&nix_expression, None);
    if !result.errors.is_empty() {
        return Err(RenderGraphConfigurationReadError::NixEvaluationError(
            result.errors,
        ));
    }
    match result.value {
        Some(value) => {
            let json_string_escaped = value.to_string();
            let json_string = json_string_escaped
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .ok_or_else(|| {
                    RenderGraphConfigurationReadError::InternalError(
                        "Failed to strip quotes from JSON string".to_string(),
                    )
                })?
                .replace("\\n", "\n")
                .replace("\\\"", "\"")
                .replace("\\\\", "\\");
            let render_pass: RenderPass = serde_json::from_str(&json_string)
                .map_err(RenderGraphConfigurationReadError::SerdeError)?;
            Ok(render_pass)
        }
        None => Err(RenderGraphConfigurationReadError::InternalError(
            "Nix evaluation returned neither value nor error".to_string(),
        )),
    }
}
