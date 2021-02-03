use anyhow::{anyhow, Result};
use clap::{App, Arg};

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
        Ok(())
    } else {
        Err(anyhow!("file name is necessary"))
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("[error] {}", err);
    }
}
