use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pngme")]
#[command(author = "chkhong <chuhennkhong@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "PNG secret encoder", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs)
}

#[derive(Args)]
pub struct EncodeArgs {
    
    /// image file path
    #[arg(short, long)]
    pub file_path: PathBuf,

    #[arg(short, long)]
    pub chunk_type: String,

    /// message to encode into png image
    #[arg(short, long)]
    pub message: String,

    /// optional image output path
    #[arg(short, long)]
    pub out_path: Option<PathBuf>

}

#[derive(Args)]
pub struct DecodeArgs {

    /// image file path
    #[arg(short, long)]
    pub file_path: PathBuf,
    
    #[arg(short, long)]
    pub chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    
    /// image file path
    #[arg(short, long)]
    pub file_path: PathBuf,
    
    #[arg(short, long)]
    pub chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {

    /// image file path
    #[arg(short, long)]
    pub file_path: PathBuf
}