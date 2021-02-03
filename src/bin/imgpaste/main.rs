use anyhow::{anyhow, Result};
use clap::{App, Arg};
use image::ImageOutputFormat;
use std::fs::File;
use std::path::Path;

fn decide_image_format<P>(file_path: P) -> Result<image::ImageOutputFormat>
where
    P: AsRef<Path>,
{
    match file_path.as_ref().extension() {
        Some(os_str) => {
            match os_str.to_str() {
                Some("png") => Ok(ImageOutputFormat::Png),
                // I don't know which value is good for jpeg quality
                Some("jpeg") | Some("jpg") => Ok(ImageOutputFormat::Jpeg(85)),
                Some("gif") => Ok(ImageOutputFormat::Gif),
                _ => Err(anyhow!("unknown file format")),
            }
        },
        _ => Err(anyhow!("file extension was unspecified")),
    }
}

fn run() -> Result<()> {
    let app = App::new("imgpaste")
        .version("0.0.1")
        .author("Ryo Kitagawa<kitadrum50@gmail.com>")
        .about("imgpaste paste image from clipboard")
        .arg(Arg::with_name("file").help("File to paste.").index(1));
    let matches = app.get_matches();

    let clipboard = clipimg::ImageClipboard::new();
    if let Some(file) = matches.value_of("file") {
        let img = clipboard.read()?;
        let mut f = File::create(file)?;
        let format = decide_image_format(file)?;
        img.write_to(&mut f, format)?;
        Ok(())
    } else {
        Err(anyhow!("file should be specified."))
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("[error] {}", err);
    }
}
