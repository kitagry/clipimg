use image::DynamicImage;
use image::io::Reader as ImageReader;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::path::Path;
use tempfile::tempdir;

#[cfg(target_os = "windows")]
fn read() -> Result<DynamicImage, String> {
    let dir = tempdir().map_err(|e| e.to_string())?;
    let file_path = dir.path().join("test.png");
    File::create(&file_path).map_err(|e| e.to_string())?;

    let file_path_str = file_path.to_str().ok_or("path not found".to_string())?;
    let cmd = format!("System.Windows.Forms;$clip=[Windows.Forms.Clipboard]::GetImage();if ($clip -ne $null) {{ $clip.Save('{}')  }};", file_path_str);
    Command::new("Powershell")
        .args(&["-Command", "Add-Type", "-AssemblyName", &cmd])
        .output()
        .map_err(|e| e.to_string())?;

    let file = ImageReader::open(file_path).map_err(|e| e.to_string())?;
    let img = file.decode().map_err(|e| e.to_string())?;
    Ok(img)
}

#[cfg(target_os = "windows")]
fn write(file_path: &Path) -> Result<(), String> {
    let file = file_path.to_str().ok_or("path is wrong")?;
    let cmd = format!("System.Windows.Forms;[System.Windows.Forms.Clipboard]::SetImage([System.Drawing.Image]::FromFile('{}'));", file);
    Command::new("Powershell")
        .args(&["-Command", "Add-Type", "-AssemblyName", &cmd])
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug)]
pub struct ImageClipboard {}

impl ImageClipboard {
    pub fn new() -> Self {
        ImageClipboard {}
    }

    pub fn write(&self, data: &[u8]) -> Result<(), String> {
        let dir = tempdir().map_err(|e| e.to_string())?;
        let file_path = dir.path().join("test.png");
        let mut f = File::create(&file_path).map_err(|e| e.to_string())?;
        f.write(data).map_err(|e| e.to_string())?;

        write(&file_path)?;
        Ok(())
    }

    pub fn write_from_file(&self, file_path: &Path) -> Result<(), String> {
        write(&file_path)
    }

    pub fn read(&self) -> Result<DynamicImage, String> {
        read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn write_to_clipboard() -> Result<(), String> {
        let clipboard = ImageClipboard::new();
        let img = ImageReader::open("./assets/test.png")
            .map_err(|e| e.to_string())?
            .decode()
            .map_err(|e| e.to_string())?;
        clipboard.write(img.as_bytes())?;
        let result = clipboard.read()?;
        assert!(result.as_bytes() == img.as_bytes(), "image was wrong");

        Ok(())
    }
}
