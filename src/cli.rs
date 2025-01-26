use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    pub input: String,
    pub output: PathBuf,
    pub top_text: Option<String>,
    pub bottom_text: Option<String>,
}
