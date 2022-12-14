use std::str::FromStr;
use std::fs;

use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use png::Png;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    match &cli.command {
        args::Commands::Encode(args) => {
            let chunk_type = ChunkType::from_str(&args.chunk_type)?;
            let message = &args.message;
            let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

            let mut png = Png::try_from(fs::read(args.file_path.clone()).unwrap().as_slice())?;

            png.append_chunk(chunk);

            if let Some(out_path) = &args.out_path {
                fs::write(out_path, png.as_bytes())?;
            }
        },
        args::Commands::Decode(args) => {
            let png = Png::try_from(fs::read(args.file_path.clone()).unwrap().as_slice())?;
            
            match png.chunk_by_type(&args.chunk_type) {
                Some(chunk) => println!("decoded message: {:?}", chunk.data_as_string()?),
                None => println!("No message decoded")
            }
        },
        args::Commands::Remove(args) => {
            let mut png = Png::try_from(fs::read(args.file_path.clone()).unwrap().as_slice())?;
            
            match png.remove_chunk(&args.chunk_type) {
                Ok(..) => {
                    fs::write(&args.file_path, png.as_bytes())?;
                    println!("Chunk deleted successfully, chunk updated");
                },
                Err(e) => eprintln!("{:?}", e)
            }
        },
        args::Commands::Print(args) => {
            let png = Png::try_from(fs::read(args.file_path.clone()).unwrap().as_slice())?;
            
            println!("{:?}", png);
        }
    }
    Ok(())
}
