use crate::constants::PROMPT_OVERRIDE_MSG;
use std::{io::Write, path::Path};

pub fn prompt_override(path: &Path) -> bool {
    print!("{}{}", path.display(), PROMPT_OVERRIDE_MSG);
    std::io::stdout().flush().unwrap_or_default();
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .unwrap_or_default();
    response.trim().is_empty() || response.trim().to_lowercase() == "y"
}

pub fn open_output(path: &Path) {
    if cfg!(target_os = "linux") {
        std::process::Command::new("xdg-open")
            .arg(path)
            .spawn()
            .ok();
    }
}
