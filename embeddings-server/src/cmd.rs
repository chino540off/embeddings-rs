#[derive(clap::Subcommand)]
pub enum Commands {
    /// Launch HTTP server
    Serve {
        /// listening addr of the HTTP server
        #[arg(long, default_value = "0.0.0.0:80")]
        listening_addr: String,
    },
    /// Only prefetch Model
    Prefetch {},
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Model identifier
    #[arg(long)]
    pub model_id: String,

    /// Model revision
    #[arg(long, default_value = "main")]
    pub model_rev: String,

    #[command(subcommand)]
    pub command: Commands,
}
