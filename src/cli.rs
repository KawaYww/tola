use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Output directory path
    #[arg(short, long, default_value = "public")]
    pub output_dir: PathBuf,

    /// Content directory path
    #[arg(short, long, default_value = "content")]
    pub content_dir: PathBuf,

    /// Assets directory path
    #[arg(short, long, default_value = "assets")]
    pub assets_dir: PathBuf,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Serve the site. Rebuild and reload on change automatically
    Serve {
        /// Interface to bind on
        #[arg(short, long, default_value = "127.0.0.1")]
        interface: String,

        /// The port you should provide
        #[arg(short, long, default_value_t = 8282)]
        port: u16,

        /// enable watch
        #[arg(short, long, default_value_t = true)]
        watch: bool,
    },

    /// Deletes the output directory if there is one and rebuilds the site
    Built {
        /// The port you should provide
        #[arg(short, long)]
        minify: bool,
    },
}
