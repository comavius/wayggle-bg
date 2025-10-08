mod json_reader;
mod nix_reader;

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tvix_eval::EvalMode;

pub use nix_reader::{
    RenderGraphNixConfigurationReadError, read_render_graph_configuration_from_nix_file,
};

pub use json_reader::{
    RenderGraphJsonConfigurationReadError, read_render_graph_configuration_from_json_file,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ConfRenderPass {
    TexturePass(ConfTexturePass),
    TransformPass(ConfTransformPass),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConfTexturePass {
    pub name: String,
    pub resolution: ConfResolution,
    pub path: PathBuf,
    pub format: ConfTextureFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ConfTextureFormat {
    Png,
    Jpeg,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConfTransformPass {
    pub name: String,
    pub resolution: ConfResolution,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub inputs: HashMap<String, ConfRenderPass>,
    pub builtin_names: ConfBuiltinsNames,
    pub outputs: Vec<String>,
    pub frame_rate: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConfResolution {
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConfBuiltinsNames {
    pub time: String,
    pub resolution: String,
    pub mouse: String,
    pub position: String,
}
