use std::path::PathBuf;

use clap::Parser;

pub mod phash;
pub mod hash;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(short, long, default_value = "c")]
    lang: String,

    #[arg(long, default_value = "info")]
    log_level: String,

    #[arg(short, long, default_value = "pho_output.c")]
    output: String,
}

// https://cmph.sourceforge.net/papers/esa09.pdf

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Args::parse();

    if !args.file.exists() {
        return Err(format!("Cannot find file {}", args.file.display()).into());
    }

    let log_level = args.log_level.trim().parse()?;

    spdlog::default_logger().set_level_filter(spdlog::LevelFilter::Equal(log_level));

    let phash = phash::PHash::from_file(&args.file)?;

    return Ok(());
}
