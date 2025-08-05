#[derive(clap::Parser)]
#[clap(
    name = "wayggle-bg",
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    #[clap(name = "shadertoy")]
    ShaderToy {
        #[clap(short, long, value_name = "FILE")]
        fragment_shader: Option<String>,
    },
    #[clap(name = "the_book_of_shaders")]
    TheBookOfShaders {
        #[clap(short, long, value_name = "FILE")]
        fragment_shader: Option<String>,
        #[clap(short, long, value_name = "FILE")]
        vertex_shader: Option<String>,
    },
}
