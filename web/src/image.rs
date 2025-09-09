use base64::engine::{Engine, general_purpose};

pub fn get_base64_from_blurhash(blurhash: &str) -> String {
    if let Ok(buf) = blurhash::decode(blurhash, 64, 64, 1.0) {
        return encode_bmp_from_rgb_or_rgba(&buf, 64, 64);
    }
    String::new()
}

pub fn encode_bmp_from_rgb_or_rgba(buf: &[u8], width: u32, height: u32) -> String {
    let expected_rgb = (width as usize) * (height as usize) * 3;
    let expected_rgba = (width as usize) * (height as usize) * 4;

    let bmp = if buf.len() == expected_rgb {
        bmp_rgb24_top_down(buf, width, height)
    } else if buf.len() == expected_rgba {
        let mut rgb = Vec::with_capacity(expected_rgb);
        for px in buf.chunks_exact(4) {
            rgb.push(px[0]);
            rgb.push(px[1]);
            rgb.push(px[2]);
        }
        bmp_rgb24_top_down(&rgb, width, height)
    } else {
        return String::new();
    };

    general_purpose::STANDARD.encode(bmp)
}

// vibe bitmap
pub fn bmp_rgb24_top_down(rgb: &[u8], width: u32, height: u32) -> Vec<u8> {
    let row_stride_rgb = (width as usize) * 3;
    if rgb.len() != row_stride_rgb * height as usize {
        return Vec::new();
    }

    let row_size_bmp = ((width * 3 + 3) / 4) * 4;
    let padding = (row_size_bmp - width * 3) as usize;
    let pixel_data_size = (row_size_bmp * height) as usize;
    let file_size = 54 + pixel_data_size;

    let mut out = Vec::with_capacity(file_size);

    out.extend_from_slice(b"BM");
    out.extend_from_slice(&(file_size as u32).to_le_bytes());
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&54u32.to_le_bytes());

    out.extend_from_slice(&40u32.to_le_bytes());
    out.extend_from_slice(&width.to_le_bytes());
    out.extend_from_slice(&(-(height as i32)).to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&24u16.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&(pixel_data_size as u32).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // x ppm
    out.extend_from_slice(&0u32.to_le_bytes()); // y ppm
    out.extend_from_slice(&0u32.to_le_bytes()); // colors used
    out.extend_from_slice(&0u32.to_le_bytes()); // important colors

    let pad = [0u8; 3];
    for y in 0..height as usize {
        let row = &rgb[y * row_stride_rgb..(y + 1) * row_stride_rgb];
        for px in row.chunks_exact(3) {
            out.push(px[2]); // B
            out.push(px[1]); // G
            out.push(px[0]); // R
        }
        out.extend_from_slice(&pad[..padding]);
    }

    out
}
