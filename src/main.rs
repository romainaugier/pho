use clap::Parser;
use phash::PHash;
use std::path::PathBuf;
use std::time::Instant;

pub mod generate;
pub mod hash;
pub mod lang;
pub mod phash;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(short, long, default_value = "string")]
    key_type: String,

    #[arg(short, long, default_value = "pho_output.c")]
    output: PathBuf,

    #[arg(short, long)]
    name: String,

    #[arg(long, default_value = "pho")]
    namespace: String,

    #[arg(long, default_value = "murmur3")]
    first_order_hash: String,

    #[arg(long, default_value = "xorshift")]
    second_order_hash: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !args.file.exists() {
        return Err(format!("Cannot find file {}", args.file.display()).into());
    }

    let start = Instant::now();

    let phash = PHash::from_file(&args.file, &args.first_order_hash, &args.second_order_hash)?;

    let elapsed = start.elapsed();
    let ms = elapsed.as_millis();

    println!("Perfect Hash found in {} ms", ms);

    return generate::gen_code(
        args.output,
        &phash,
        args.name.as_str(),
        args.namespace.as_str(),
    );
}
