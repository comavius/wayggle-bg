#[derive(clap::Parser)]
#[clap(
    name = "wayggle-bg",
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
    #[clap(short = 'c', long = "enable-cursor-support", value_name = "COMPOSITOR", value_enum, default_value_t = CursorSupportKind::Disabled)]
    pub cursor_support: CursorSupportKind,
}

#[derive(clap::Subcommand)]
pub enum Command {
    #[clap(name = "shadertoy")]
    ShaderToy {
        #[clap(short, long, value_name = "FILE")]
        fragment_shader: String,
    },
    #[clap(name = "the_book_of_shaders")]
    TheBookOfShaders {
        #[clap(short, long, value_name = "FILE")]
        fragment_shader: String,
        #[clap(short, long, value_name = "FILE")]
        vertex_shader: Option<String>,
    },
    #[clap(
        name = "default",
        about = "Run the default shader with a given name<'box'>."
    )]
    Default {
        #[clap(short, long, value_name = "NAME")]
        name: String,
    },
}

#[derive(Clone, clap::ValueEnum)]
pub enum CursorSupportKind {
    Hyprland,
    Disabled,
}
