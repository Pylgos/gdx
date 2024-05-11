use std::{path::Path, process::Command};


pub fn compile(src_path: &Path, dst_path: &Path) -> Result<(), std::io::Error> {
    Command::new("cc")
        .arg("-o")
        .arg(dst_path)
        .arg(src_path)
        .output()?;
    Ok(())
}

