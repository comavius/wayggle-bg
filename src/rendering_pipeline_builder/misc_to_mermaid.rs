use super::*;

pub fn to_mermaid(rendering_pipeline: &RenderingPipeline) -> String
{
    let mut mermaid = String::from("graph TD\n");
    for (id, pass) in &rendering_pipeline.passes {
        let pass_label = match pass {
            RenderingPass::TexturePass(texture_pass) => {
                format!(
                    "{}[\"Texture Pass: {}\\n{}x{}\"]",
                    id,
                    texture_pass.name,
                    texture_pass
                        .resolution
                        .width,
                    texture_pass
                        .resolution
                        .height
                )
            }
            RenderingPass::TransformPass(transform_pass) => format!(
                "{}[\"Transform Pass: {}\\n{}x{}\"]",
                id,
                transform_pass.name,
                transform_pass
                    .resolution
                    .width,
                transform_pass
                    .resolution
                    .height
            ),
        };
        mermaid.push_str(&format!("    {}\n", pass_label));
    }
    for dependency in &rendering_pipeline.dependencies {
        mermaid.push_str(&format!(
            "    {} -->|{}| {}\n",
            dependency.from, dependency.name, dependency.to
        ));
    }
    mermaid
}
