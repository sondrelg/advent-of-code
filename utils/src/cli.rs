use clap::{Parser, Subcommand};

use crate::data::Data;

#[derive(Clone, Parser)]
pub struct Wrapper {
    pub day: u8,

    #[clap(default_value = "full")]
    pub data: Data,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(name = "run")]
    Run(Wrapper),

    #[clap(name = "download")]
    Download(Wrapper),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
