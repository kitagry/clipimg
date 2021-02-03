use anyhow::{anyhow, Result};
use std::fs::File;
use clap::{App, Arg};

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
        img.write_to(&mut f, image::ImageOutputFormat::Png)?;
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
