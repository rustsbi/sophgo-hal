use core::mem::{size_of, size_of_val};
use crc::{Crc, CRC_16_XMODEM};

#[derive(Debug)]
pub enum Error {
    HeadLength {
        wrong_length: usize,
    },
    MagicNumber {
        wrong_magic: u32,
    },
    RawBlobMagic {
        wrong_magic: [u8; 32],
    },
    ImageContentLength {
        wrong_content_length: usize,
        wrong_full_length: usize,
    },
    OutputBufferLength {
        wrong_length: usize,
    },
}

pub type Result<T> = core::result::Result<T, Error>;

pub struct Operations<'a> {
    pub refill_header: Option<HeaderInfo>,
    pub set_image_content: Option<&'a [u8]>,
    pub resize_image_full_length: usize,
}

pub struct HeaderInfo {
    pub blcp_image_checksum: u32,
    pub bl2_image_checksum: u32,
    pub bl2_image_size: u32,
}

// TODO: supports: 1. blob with magic 2. full fip.bin 3. ELF file
// 1. blob with blob magic
//    - returns additional header content and padding size
// 2. full fip.bin (BL2 FIP magic): returns
//    - repaired header if wrong checksum
//    - error if image truncated (image length + image offset > file length)
// 3. ELF containing blob (ELF magic):
//    returns header and image content
pub fn check(buf: &[u8]) -> Result<Operations> {
    if buf.len() < size_of::<u32>() {
        return Err(Error::HeadLength {
            wrong_length: buf.len(),
        });
    }
    let u32_magic = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    match u32_magic {
        0x0200006F => check_raw_blob(buf),
        0x4C425643 => check_cvbl_fip(buf),
        0x7F454C46 => check_elf(buf),
        wrong_magic => Err(Error::MagicNumber { wrong_magic }),
    }
}

const HEADER_LENGTH: usize = 0x1000;
const BLOB_MAGIC: [u8; 32] = [
    0x6F, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

fn check_raw_blob(buf: &[u8]) -> Result<Operations> {
    if buf.len() < size_of_val(&BLOB_MAGIC) || buf.len() >= (u32::MAX - 512) as usize {
        return Err(Error::HeadLength {
            wrong_length: buf.len(),
        });
    }
    if buf[..BLOB_MAGIC.len()] != BLOB_MAGIC {
        return Err(Error::RawBlobMagic {
            wrong_magic: buf[..BLOB_MAGIC.len()].try_into().unwrap(),
        });
    }
    let crc = Crc::<u16>::new(&CRC_16_XMODEM);
    let padding_len = 512 - (buf.len() % 512);
    // TODO no allocations required
    let mut padded_buf = Vec::new();
    padded_buf.extend(buf);
    padded_buf.extend(core::iter::repeat(0).take(padding_len));
    let bl2_checksum_part = crc.checksum(&padded_buf);
    let bl2_checksum = 0xCAFE0000u32 + bl2_checksum_part as u32;
    // TODO no allocations required
    let bl2_padded_size = buf.len() + padding_len;
    Ok(Operations {
        refill_header: Some(HeaderInfo {
            blcp_image_checksum: 0xCAFE0000,
            bl2_image_checksum: bl2_checksum,
            bl2_image_size: bl2_padded_size as u32,
        }),
        set_image_content: Some(buf),
        resize_image_full_length: HEADER_LENGTH + bl2_padded_size,
    })
}

const CVBL01_MAGIC: [u8; 12] = [
    0x43, 0x56, 0x42, 0x4C, 0x30, 0x31, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00,
]; // CVBL01

fn check_cvbl_fip(buf: &[u8]) -> Result<Operations> {
    todo!("{:?}", buf)
}

fn check_elf(buf: &[u8]) -> Result<Operations> {
    todo!("{:?}", buf)
}

pub fn process(buf: &mut [u8], ops: &Operations) -> Result<()> {
    if buf.len() < ops.resize_image_full_length {
        return Err(Error::OutputBufferLength {
            wrong_length: buf.len(),
        });
    }
    if let Some(header) = &ops.refill_header {
        buf[..CVBL01_MAGIC.len()].copy_from_slice(&CVBL01_MAGIC);
        buf[0xBC..0xC0].copy_from_slice(&0x2F8u32.to_le_bytes()); // chip_conf_size
        buf[0xC0..0xC4].copy_from_slice(&header.blcp_image_checksum.to_le_bytes());
        buf[0xD4..0xD8].copy_from_slice(&header.bl2_image_checksum.to_le_bytes());
        buf[0xD8..0xDC].copy_from_slice(&header.bl2_image_size.to_le_bytes());
        let crc = Crc::<u16>::new(&CRC_16_XMODEM);
        let param_checksum = 0xCAFE0000u32 + crc.checksum(&buf[0x10..0x800]) as u32;
        buf[0xC..0x10].copy_from_slice(&param_checksum.to_le_bytes())
    }
    if let Some(image) = &ops.set_image_content {
        if image.len() > u32::MAX as usize
            || ops.resize_image_full_length > u32::MAX as usize
            || ops.resize_image_full_length < HEADER_LENGTH
            || image.len() + HEADER_LENGTH > ops.resize_image_full_length
        {
            return Err(Error::ImageContentLength {
                wrong_content_length: image.len(),
                wrong_full_length: ops.resize_image_full_length,
            });
        }
        buf[HEADER_LENGTH..][..image.len()].copy_from_slice(image);
    }
    Ok(())
}
