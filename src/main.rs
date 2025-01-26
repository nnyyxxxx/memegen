use anyhow::Result;
use clap::Parser;

mod cli;
mod constants;
mod image_processor;
mod utils;

use cli::Args;
use image_processor::process_image;

fn main() -> Result<()> {
    let args = Args::parse();
    process_image(args)
}
