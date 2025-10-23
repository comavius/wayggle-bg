use super::{
    conf_reader::*,
    *,
};
use std::{
    collections::HashMap,
    path::PathBuf,
};
use uuid::Uuid;

/// Transforming recursive reference style DAG into (V, E) style.
///
/// Approx. O(|V|^3 + |E|) time complexity
/// for easy and safe render pass caching.
/// This can be improved to O(|V| log |V| + |E|) with hashes,
/// but I don't like manual hash implementation.
pub fn flatten_rendering_pipeline(
    conf_render_pass: &ConfRenderPass,
) -> RenderingPipeline
{
    let mut passes = vec![];
    let mut dependencies = vec![];
    flatten_rendering_pipeline_inner(
        conf_render_pass,
        &mut passes,
        &mut dependencies,
    );
    RenderingPipeline {
        passes: passes
            .into_iter()
            .map(|(id, pass)| {
                let flat_pass = match pass {
                    ConfRenderPass::TexturePass(pass) => {
                        RenderingPass::TexturePass(TexturePass {
                            name: pass.name,
                            resolution: Resolution {
                                height: pass
                                    .resolution
                                    .height,
                                width: pass
                                    .resolution
                                    .width,
                            },
                            path: pass.path,
                            format: match pass.format {
                                ConfTextureFormat::Png => TextureFormat::Png,
                                ConfTextureFormat::Jpeg => TextureFormat::Jpeg,
                                ConfTextureFormat::Auto => TextureFormat::Auto,
                            },
                        })
                    }
                    ConfRenderPass::TransformPass(pass) => {
                        RenderingPass::TransformPass(TransformPass {
                            name: pass.name,
                            resolution: Resolution {
                                height: pass
                                    .resolution
                                    .height,
                                width: pass
                                    .resolution
                                    .width,
                            },
                            vertex_shader: pass.vertex_shader,
                            fragment_shader: pass.fragment_shader,
                            builtin_names: BuiltinsNames {
                                time: pass
                                    .builtin_names
                                    .time,
                                resolution: pass
                                    .builtin_names
                                    .resolution,
                                mouse: pass
                                    .builtin_names
                                    .mouse,
                                position: pass
                                    .builtin_names
                                    .position,
                            },
                            frame_rate: pass.frame_rate,
                        })
                    }
                };
                (id, flat_pass)
            })
            .collect(),
        dependencies,
    }
}

fn flatten_rendering_pipeline_inner(
    conf_render_pass: &ConfRenderPass,
    passes: &mut Vec<(Uuid, ConfRenderPass)>,
    dependencies: &mut Vec<Dependency>,
) -> Uuid
{
    let existing_id = passes
        .iter()
        .find(|(_, pass)| pass == conf_render_pass)
        .map(|(id, _)| *id);
    let id = if let Some(id) = existing_id {
        id
    } else {
        let id = Uuid::new_v4();
        passes.push((id, conf_render_pass.clone()));
        if let ConfRenderPass::TransformPass(transform_pass) = conf_render_pass
        {
            for (input_name, input_pass) in &transform_pass.inputs {
                let input_id = flatten_rendering_pipeline_inner(
                    input_pass,
                    passes,
                    dependencies,
                );
                dependencies.push(Dependency {
                    from: input_id,
                    to: id,
                    name: input_name.clone(),
                });
            }
        }
        id
    };
    id
}
