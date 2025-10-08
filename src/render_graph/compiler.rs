use thiserror::Error;

pub enum ShaderKind {
    Vertex,
    Fragment,
}

#[derive(Debug, Error)]
pub enum ShaderCompilaionError {
    #[error("Inner compiler error: {0}")]
    InnerCompilerError(#[from] shaderc::Error),
}

pub struct ShaderCompilationParams {
    pub source: String,
    pub kind: ShaderKind,
    pub entry_point: String,
    pub tag: String,
}

/// Compile GLSL source code into SPIR-V binary.
///
/// Encapsulating `shaderc` interfaces.
pub fn compile_to_spirv(
    source: &ShaderCompilationParams,
) -> Result<Vec<u32>, ShaderCompilaionError> {
    let shader_kind = match source.kind {
        ShaderKind::Vertex => shaderc::ShaderKind::Vertex,
        ShaderKind::Fragment => shaderc::ShaderKind::Fragment,
    };
    let compiler = shaderc::Compiler::new().map_err(ShaderCompilaionError::InnerCompilerError)?;
    let compile_options =
        shaderc::CompileOptions::new().map_err(ShaderCompilaionError::InnerCompilerError)?;
    let binary_result = compiler
        .compile_into_spirv(
            &source.source,
            shader_kind,
            &source.tag,
            &source.entry_point,
            Some(&compile_options),
        )?
        .as_binary()
        .to_vec();
    Ok(binary_result)
}
