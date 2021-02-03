use anyhow::Result;
use clap::{App, Arg};
use image::io::Reader as ImageReader;
use std::io::{self, Cursor, Read};

fn run() -> Result<()> {
    let app = App::new("imgcopy")
        .version("0.0.1")
        .author("Ryo Kitagawa<kitadrum50@gmail.com>")
        .about("imgcopy copy image to clipboard")
        .arg(Arg::with_name("file").help("Sets target file").index(1));
    let matches = app.get_matches();

    let clipboard = clipimg::ImageClipboard::new();
    if let Some(file) = matches.value_of("file") {
        clipboard.write_from_file(file)?;
    } else {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        let reader = ImageReader::new(Cursor::new(buf))
            .with_guessed_format()?
            .decode()?;
        clipboard.write(&reader)?;
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("[error] {}", err);
    }
}
