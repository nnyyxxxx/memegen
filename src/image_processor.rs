use crate::{cli::Args, constants::*, utils};
use anyhow::{Context, Result};
use std::{fs, process::Command};

pub fn process_image(args: Args) -> Result<()> {
    if args.output.exists() && !utils::prompt_override(&args.output) {
        return Ok(());
    }

    let img_data = if args.input.starts_with("http") {
        reqwest::blocking::get(&args.input)?.bytes()?
    } else {
        fs::read(&args.input)?.into()
    };

    let img = image::load_from_memory(&img_data)?;
    let width = img.width();
    let caption_height = width / 6;
    let stroke_width = (width / 400).max(1);

    let temp_path = std::env::temp_dir().join(TEMP_FILE);
    img.save(&temp_path)?;

    let mut cmd = Command::new("magick");
    cmd.args([
        temp_path.to_str().unwrap(),
        "-background",
        BACKGROUND,
        "-font",
        FONT,
        "-fill",
        FILL_COLOR,
        "-stroke",
        STROKE_COLOR,
        "-strokewidth",
        &stroke_width.to_string(),
        "-size",
        &format!("{}x{}", width, caption_height),
    ]);

    if let Some(text) = args.top_text {
        let caption = format!("caption:{}", text.to_uppercase());
        cmd.args(["-gravity", "north", &caption, "-composite"]);
    }

    if let Some(text) = args.bottom_text {
        let caption = format!("caption:{}", text.to_uppercase());
        cmd.args(["-gravity", "south", &caption, "-composite"]);
    }

    cmd.arg(&args.output);

    let status = cmd.output().context(FAILED_MAGICK_ERR)?;

    fs::remove_file(temp_path)?;

    if !status.status.success() {
        let error = String::from_utf8_lossy(&status.stderr);
        anyhow::bail!("{}{}", MAGICK_FAILED_ERR, error);
    }

    if !args.output.exists() {
        anyhow::bail!(OUTPUT_NOT_CREATED_ERR);
    }

    println!("{}{}", DONE_MSG, args.output.display());
    utils::open_output(&args.output);

    Ok(())
}
