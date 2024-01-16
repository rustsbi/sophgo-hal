use clap::Parser;
use std::fs;

/// Generate a ROM image for Sophgo chips.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input filename.
    input: String,
    /// Output ROM image filename.
    #[arg(short, long, default_value_t = String::from("fip.bin"))]
    output: String,
}

fn main() {
    let args = Args::parse();
    let data = fs::read(&args.input).expect("Unable to read file");

    // TODO handle error
    let ops = sophgo_rom_tool::check(&data).unwrap();

    let mut image = vec![0u8; ops.resize_image_full_length];
    // TODO handle error
    sophgo_rom_tool::process(&mut image, &ops).unwrap();

    fs::write(&args.output, image).expect("Unable to write file");
}
