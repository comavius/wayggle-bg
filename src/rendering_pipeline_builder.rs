use std::path::PathBuf;
use std::{collections::HashMap, path::Path};
use uuid::Uuid;

mod conf_reader;
mod flatten_graph;
mod misc_to_mermaid;

pub use misc_to_mermaid::to_mermaid;

pub fn build_rendering_pipeline(
    pipeline_configuration: &PipelineConfiguration,
) -> Result<RenderingPipeline, String> {
    let conf_render_pass = match &pipeline_configuration.setting_files {
        RenderingPipelineSettingFiles::FromJson(json_file) => {
            conf_reader::read_rendering_pipeline_configuration_from_json_file(&json_file.json_path)
                .map_err(|e| format!("Failed to read JSON configuration: {}", e))?
        }
        RenderingPipelineSettingFiles::FromNix(nix_files) => {
            conf_reader::read_rendering_pipeline_configuration_from_nix_file(
                &nix_files.nix_path,
                &nix_files.nix_lib_dir,
                conf_reader::ConfResolution {
                    height: pipeline_configuration.monitor_resolution.height,
                    width: pipeline_configuration.monitor_resolution.width,
                },
            )
            .map_err(|e| format!("Failed to read Nix configuration: {}", e))?
        }
    };
    let rendering_pipeline = flatten_graph::flatten_rendering_pipeline(&conf_render_pass);
    Ok(rendering_pipeline)
}

// Input Types
#[derive(Debug, Clone)]
pub enum RenderingPipelineSettingFiles {
    FromJson(PipelineJsonSettingFile),
    FromNix(PipelineNixSettingFiles),
}

#[derive(Debug, Clone)]
pub struct PipelineJsonSettingFile {
    pub json_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct PipelineNixSettingFiles {
    pub nix_path: PathBuf,
    pub nix_lib_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct PipelineConfiguration {
    pub setting_files: RenderingPipelineSettingFiles,
    pub monitor_resolution: Resolution,
}

// Output Types
#[derive(Debug, Clone)]
pub struct RenderingPipeline {
    pub passes: HashMap<Uuid, RenderingPass>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone)]
pub enum RenderingPass {
    TexturePass(TexturePass),
    TransformPass(TransformPass),
}

#[derive(Debug, Clone)]
pub struct TexturePass {
    pub name: String,
    pub resolution: Resolution,
    pub path: PathBuf,
    pub format: TextureFormat,
}

#[derive(Debug, Clone)]
pub enum TextureFormat {
    Png,
    Jpeg,
    Auto,
}

#[derive(Debug, Clone)]
pub struct TransformPass {
    pub name: String,
    pub resolution: Resolution,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub builtin_names: BuiltinsNames,
    pub frame_rate: u32,
}

#[derive(Debug, Clone)]
pub struct Resolution {
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Clone)]
pub struct BuiltinsNames {
    pub time: String,
    pub resolution: String,
    pub mouse: String,
    pub position: String,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub from: Uuid,
    pub to: Uuid,
    pub name: String,
}
