use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct PngmeArgs {
    #[command[subcommand]]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
    pub data: String,
}

#[derive(Args)]
pub struct DecodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    pub path: PathBuf,
}
