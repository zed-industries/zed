use std::fs::{self, File};
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use cfg_if::cfg_if;
use walkdir::{DirEntry, WalkDir};

use qoi::{decode_to_vec, encode_to_vec};

fn find_qoi_png_pairs(root: impl AsRef<Path>) -> Vec<(PathBuf, PathBuf)> {
    let root = root.as_ref();

    let get_ext =
        |path: &Path| path.extension().unwrap_or_default().to_string_lossy().to_ascii_lowercase();
    let check_qoi_png_pair = |path: &Path| {
        let (qoi, png) = (path.to_path_buf(), path.with_extension("png"));
        if qoi.is_file() && get_ext(&qoi) == "qoi" && png.is_file() {
            Some((qoi, png))
        } else {
            None
        }
    };

    let mut out = vec![];
    if let Some(pair) = check_qoi_png_pair(root) {
        out.push(pair);
    } else if root.is_dir() {
        out.extend(
            WalkDir::new(root)
                .follow_links(true)
                .into_iter()
                .filter_map(Result::ok)
                .map(DirEntry::into_path)
                .filter_map(|p| check_qoi_png_pair(&p)),
        )
    }
    out
}

struct Image {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub data: Vec<u8>,
}

impl Image {
    fn from_png(filename: &Path) -> Result<Self> {
        let decoder = png::Decoder::new(File::open(filename)?);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        let bytes = &buf[..info.buffer_size()];
        Ok(Self {
            width: info.width,
            height: info.height,
            channels: info.color_type.samples() as u8,
            data: bytes.to_vec(),
        })
    }
}

fn compare_slices(name: &str, desc: &str, result: &[u8], expected: &[u8]) -> Result<()> {
    if result == expected {
        Ok(())
    } else {
        if let Some(i) =
            (0..result.len().min(expected.len())).position(|i| result[i] != expected[i])
        {
            bail!(
                "{}: {} mismatch at byte {}: expected {:?}, got {:?}",
                name,
                desc,
                i,
                &expected[i..(i + 4).min(expected.len())],
                &result[i..(i + 4).min(result.len())],
            );
        } else {
            bail!(
                "{}: {} length mismatch: expected {}, got {}",
                name,
                desc,
                expected.len(),
                result.len()
            );
        }
    }
}

#[test]
fn test_reference_images() -> Result<()> {
    let pairs = find_qoi_png_pairs("assets");
    assert!(!pairs.is_empty());

    for (qoi_path, png_path) in &pairs {
        let png_name = png_path.file_name().unwrap_or_default().to_string_lossy();
        let img = Image::from_png(png_path)?;
        println!("{} {} {} {}", png_name, img.width, img.height, img.channels);
        let encoded = encode_to_vec(&img.data, img.width, img.height)?;
        let expected = fs::read(qoi_path)?;
        assert_eq!(encoded.len(), expected.len()); // this should match regardless
        cfg_if! {
            if #[cfg(feature = "reference")] {
                compare_slices(&png_name, "encoding", &encoded, &expected)?;
            }
        }
        let (_header1, decoded1) = decode_to_vec(&encoded)?;
        let (_header2, decoded2) = decode_to_vec(&expected)?;
        compare_slices(&png_name, "decoding [1]", &decoded1, &img.data)?;
        compare_slices(&png_name, "decoding [2]", &decoded2, &img.data)?;
    }

    Ok(())
}
