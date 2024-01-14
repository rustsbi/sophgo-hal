use clap::Parser;
use crc::{Crc, CRC_16_XMODEM};
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

const SIGNATURE: [u8; 32] = [
    0x6F, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const MAGIC: [u8; 12] = [
    0x43, 0x56, 0x42, 0x4C, 0x30, 0x31, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00,
]; // CVBL01

fn main() {
    let args = Args::parse();
    let data = fs::read(&args.input).expect("Unable to read file");
    let data_len = data.len();
    if data_len < SIGNATURE.len() || data[..SIGNATURE.len()] != SIGNATURE {
        panic!("Invalid input file");
    }

    let mut image = vec![0; 0x1000];
    image.extend(data);
    let padding_len = 512 - (data_len % 512);
    image.extend(core::iter::repeat(0).take(padding_len));

    let crc = Crc::<u16>::new(&CRC_16_XMODEM);
    let bl2_checksum = crc.checksum(&image[0x1000..]);

    image[..MAGIC.len()].copy_from_slice(&MAGIC);
    image[0xBC..0xC0].copy_from_slice(&0x2F8u32.to_le_bytes()); // chip_conf_size
    image[0xC0..0xC4].copy_from_slice(&0xCAFE0000u32.to_le_bytes()); // blcp_img_cksum
    image[0xD4..0xD8].copy_from_slice(&(0xCAFE0000u32 + bl2_checksum as u32).to_le_bytes()); // bl2_img_cksum
    image[0xD8..0xDC].copy_from_slice(&((data_len + padding_len) as u32).to_le_bytes()); // bl2_img_size

    let param_checksum = crc.checksum(&image[0x10..0x800]);
    image[0xC..0x10].copy_from_slice(
        (0xCAFE0000u32 + param_checksum as u32)
            .to_le_bytes()
            .as_ref(),
    ); // param_cksum

    fs::write(&args.output, image).expect("Unable to write file");
}
