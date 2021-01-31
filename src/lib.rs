use anyhow::{Result, Context};
use image::DynamicImage;
use image::io::Reader as ImageReader;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::path::Path;
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
    Command::new("Powershell")
        .args(&["-Command", "Add-Type", "-AssemblyName", &cmd])
        .output()
        .context("failed to write image to clipboard")?;
    Ok(())
}

#[derive(Debug)]
pub struct ImageClipboard {}

impl ImageClipboard {
    pub fn new() -> Self {
        ImageClipboard {}
    }

    pub fn write(&self, data: &[u8]) -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.png");
        let mut f = File::create(&file_path)?;
        f.write(data)?;

        self.write_from_file(&file_path)
    }

    pub fn write_from_file(&self, file_path: &Path) -> Result<()> {
        write(&file_path)
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
        clipboard.write(img.as_bytes())?;
        let result = clipboard.read()?;
        assert!(result.as_bytes() == img.as_bytes(), "image was wrong");

        Ok(())
    }

    #[test]
    fn write_from_file() -> Result<()> {
        let clipboard = ImageClipboard::new();
        clipboard.write_from_file(Path::new("./assets/test.png"))
    }
}
