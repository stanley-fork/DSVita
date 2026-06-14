use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

/// All `.png` file names in `dir`, sorted. Empty when the dir is missing.
pub fn list(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            name.rsplit('.').next().is_some_and(|ext| ext.eq_ignore_ascii_case("png")).then_some(name)
        })
        .collect();
    names.sort();
    names
}

/// A decoded overlay: top-down RGBA8 pixels plus the source dimensions.
pub struct Overlay {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// Decode an 8-bit RGB/RGBA PNG to RGBA8. `None` on any error or unsupported
/// format (the caller treats that the same as a missing overlay).
pub fn load(path: &Path) -> Option<Overlay> {
    let decoder = png::Decoder::new(BufReader::new(File::open(path).ok()?));
    let mut reader = decoder.read_info().ok()?;
    let mut buf = vec![0u8; reader.output_buffer_size()?];
    let info = reader.next_frame(&mut buf).ok()?;
    if info.bit_depth != png::BitDepth::Eight {
        return None;
    }
    let data = match info.color_type {
        png::ColorType::Rgba => {
            buf.truncate(info.buffer_size());
            buf
        }
        png::ColorType::Rgb => {
            let pixels = (info.width * info.height) as usize;
            let mut rgba = vec![0u8; pixels * 4];
            for i in 0..pixels {
                rgba[i * 4..i * 4 + 3].copy_from_slice(&buf[i * 3..i * 3 + 3]);
                rgba[i * 4 + 3] = 0xFF;
            }
            rgba
        }
        _ => return None,
    };
    Some(Overlay {
        data,
        width: info.width,
        height: info.height,
    })
}
