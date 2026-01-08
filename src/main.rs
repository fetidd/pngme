mod args;
mod chunk;
mod chunk_type;
mod error;
mod png;

use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::{
    args::{Commands::*, DecodeArgs, EncodeArgs, PngmeArgs, PrintArgs, RemoveArgs},
    chunk::Chunk,
    chunk_type::ChunkType,
    error::Result,
    png::Png,
};

fn encode(args: EncodeArgs) -> Result<()> {
    let EncodeArgs {
        path,
        chunk_type,
        data,
    } = args;
    let mut png = create_png(&path)?;
    let chunk_type = ChunkType::from_str(chunk_type.as_str())?;
    let new_chunk = Chunk::new(chunk_type, data.as_bytes().to_vec());
    png.append_chunk(new_chunk);
    std::fs::write(path, png.as_bytes())?;
    Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
    let DecodeArgs { path, chunk_type } = args;
    let png = create_png(&path)?;
    let chunk = png.chunk_by_type(&chunk_type);
    if let Some(chunk) = chunk {
        println!("{chunk}");
    }
    Ok(())
}

fn remove(args: RemoveArgs) -> Result<()> {
    let RemoveArgs { path, chunk_type } = args;
    let mut png = create_png(&path)?;
    png.remove_first_chunk(&chunk_type)?;
    std::fs::write(path, png.as_bytes())?;
    Ok(())
}

fn print(args: PrintArgs) -> Result<()> {
    let PrintArgs { path } = args;
    let png = create_png(&path)?;
    println!("{png}");
    Ok(())
}

fn create_png(path: &PathBuf) -> Result<Png> {
    let png = std::fs::read(path).map_err(Box::new)?;
    let png = Png::try_from(png.as_slice())?;
    Ok(png)
}

fn main() -> Result<()> {
    let args = PngmeArgs::parse();
    match args.command {
        Encode(encode_args) => encode(encode_args),
        Decode(decode_args) => decode(decode_args),
        Remove(remove_args) => remove(remove_args),
        Print(print_args) => print(print_args),
    }
}
