use anyhow::{anyhow, Context, Result};
use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::tempdir;

#[cfg(target_os = "windows")]
fn read() -> Result<DynamicImage> {
    let dir = tempdir()?;
    let file_path = dir.path().join("test.png");
    File::create(&file_path)?;

    let file_path_str = file_path.to_str().context("path not found")?;
    let cmd = format!("System.Windows.Forms;$clip=[Windows.Forms.Clipboard]::GetImage();if ($clip -ne $null) {{ $clip.Save('{}')  }};", file_path_str);
    Command::new("Powershell")
        .args(&["-Command", "Add-Type", "-AssemblyName", &cmd])
        .output()
        .context("failed to read image from clipboard")?;

    let file = ImageReader::open(file_path)?;
    let img = file.decode()?;
    Ok(img)
}

#[cfg(target_os = "windows")]
fn write(file_path: &Path) -> Result<()> {
    let file = file_path.to_str().context("file path is wrong")?;
    let cmd = format!("System.Windows.Forms;[System.Windows.Forms.Clipboard]::SetImage([System.Drawing.Image]::FromFile('{}'));", file);
    let output = Command::new("Powershell")
        .args(&["-Command", "Add-Type", "-AssemblyName", &cmd])
        .output()
        .context("failed to write image to clipboard")?;

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "failed to run {}",
            str::from_utf8(output.stderr.as_ref())?
        ))
    }
}

#[cfg(target_os = "macos")]
fn read() -> Result<DynamicImage> {
    let dir = tempdir()?;
    let file_path = dir.path().join("test.png");
    File::create(&file_path)?;

    let file_path_str = file_path.to_str().context("path not found")?;
    let cmd = format!(
        "write (the clipboard as «class PNGf») to (open for access \"{}\" with write permission)",
        file_path_str
    );
    Command::new("osascript")
        .args(&["-e", &cmd])
        .output()
        .context("failed to read image from clipboard")?;

    let file = ImageReader::open(file_path)?;
    let img = file.decode()?;
    Ok(img)
}

#[cfg(target_os = "macos")]
fn write(file_path: &Path) -> Result<()> {
    let file = file_path.to_str().context("file path is wrong")?;
    let cmd = format!("set the clipboard to (read \"{}\" as «class PNGf»)", file);
    let output = Command::new("osascript")
        .args(&["-e", &cmd])
        .output()
        .context("failed to write image to clipboard")?;

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "failed to run {}",
            str::from_utf8(output.stderr.as_ref())?
        ))
    }
}

#[derive(Debug)]
pub struct ImageClipboard {}

impl ImageClipboard {
    pub fn new() -> Self {
        ImageClipboard {}
    }

    pub fn write(&self, data: &DynamicImage) -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.png");
        data.save_with_format(&file_path, image::ImageFormat::Png)?;

        self.write_from_file(&file_path)
    }

    pub fn write_from_file<P>(&self, file_path: P) -> Result<()> where P: AsRef<Path> {
        write(file_path.as_ref())
    }

    pub fn read(&self) -> Result<DynamicImage> {
        read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn write_to_clipboard() -> Result<()> {
        let clipboard = ImageClipboard::new();
        let img = ImageReader::open("./assets/test.png")?.decode()?;
        clipboard.write(&img)?;
        let result = clipboard.read()?;
        assert!(result.as_bytes() == img.as_bytes(), "image was wrong");

        Ok(())
    }

    #[test]
    fn write_from_file() -> Result<()> {
        let clipboard = ImageClipboard::new();
        clipboard.write_from_file(Path::new("./assets/test.png"))
    }

    #[test]
    fn write_jpeg() -> Result<()> {
        let clipboard = ImageClipboard::new();
        clipboard.write_from_file(Path::new("./assets/test.jpeg"))
    }
}
