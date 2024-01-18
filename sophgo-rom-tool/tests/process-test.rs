use sophgo_rom_tool::{Error, HeaderInfo};

#[test]
fn process_success() {
    let mut content = vec![0u8; 4098];
    let ops = sophgo_rom_tool::Operations {
        refill_header: None,
        set_image_content: Some(&[0x11, 0x22]),
        resize_image_full_length: 4098,
    };
    let ans = sophgo_rom_tool::process(&mut content, &ops);
    assert_eq!(ans, Ok(()));
    let mut expected = vec![0u8; 4096];
    expected.push(0x11);
    expected.push(0x22);
    assert_eq!(content, expected);
    let mut content = vec![0u8; 4098];
    let ops = sophgo_rom_tool::Operations {
        refill_header: Some(HeaderInfo {
            blcp_image_checksum: 0x11112222,
            bl2_image_checksum: 0x33334444,
            bl2_image_size: 0x55556666,
        }),
        set_image_content: Some(&[0x11, 0x22]),
        resize_image_full_length: 4098,
    };
    let ans = sophgo_rom_tool::process(&mut content, &ops);
    assert_eq!(ans, Ok(()));
    let mut expected = vec![0u8; 4098];
    expected[..12].copy_from_slice(&[
        0x43, 0x56, 0x42, 0x4C, 0x30, 0x31, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    expected[0xBC..0xC0].copy_from_slice(&[0xF8, 0x02, 0x00, 0x00]);
    expected[0xC0..0xC4].copy_from_slice(&[0x22, 0x22, 0x11, 0x11]);
    expected[0xD4..0xD8].copy_from_slice(&[0x44, 0x44, 0x33, 0x33]);
    expected[0xD8..0xDC].copy_from_slice(&[0x66, 0x66, 0x55, 0x55]);
    expected[0xC..0x10].copy_from_slice(&[0xA5, 0xAC, 0xFE, 0xCA]);
    expected[4096..].copy_from_slice(&[0x11, 0x22]);
    assert_eq!(content, expected);
}

#[test]
fn process_error_output_buffer_length() {
    let mut content = vec![0u8; 50];
    let ops = sophgo_rom_tool::Operations {
        refill_header: None,
        set_image_content: None,
        resize_image_full_length: 100,
    };
    let ans = sophgo_rom_tool::process(&mut content, &ops);
    assert_eq!(ans, Err(Error::OutputBufferLength { wrong_length: 50 }))
}

#[test]
fn process_error_image_full_length() {
    let mut content = vec![0u8; 501];
    let ops = sophgo_rom_tool::Operations {
        refill_header: None,
        set_image_content: None,
        resize_image_full_length: 500,
    };
    let ans = sophgo_rom_tool::process(&mut content, &ops);
    assert_eq!(
        ans,
        Err(Error::ImageFullLength {
            wrong_full_length: 500
        })
    );
}

#[test]
fn process_error_image_content_length() {
    let mut content = vec![0u8; 5000];
    let ops = sophgo_rom_tool::Operations {
        refill_header: None,
        set_image_content: Some(&[0x11, 0x22]),
        resize_image_full_length: 600,
    };
    let ans = sophgo_rom_tool::process(&mut content, &ops);
    assert_eq!(
        ans,
        Err(Error::ImageFullLength {
            wrong_full_length: 600
        })
    );
}
