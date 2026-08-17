use std::cmp::Ordering;

/// String encodings.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
	/// Unknown encoding.
	Unknown,
	/// Common [US_ASCII](https://en.wikipedia.org/wiki/ASCII) encoding.
	Ansi,
	/// [Windows-1252](https://en.wikipedia.org/wiki/Windows-1252) encoding.
	Win1252,
	/// [UTF-8](https://en.wikipedia.org/wiki/UTF-8) encoding.
	Utf8,
	/// [UTF-16](https://en.wikipedia.org/wiki/UTF-16) encoding, big-endian.
	Utf16be,
	/// [UTF-16](https://en.wikipedia.org/wiki/UTF-16) encoding, little-endian.
	Utf16le,
	/// [UTF-32](https://en.wikipedia.org/wiki/UTF-32) encoding, big-endian.
	Utf32be,
	/// [UTF-32](https://en.wikipedia.org/wiki/UTF-32) encoding, little-endian.
	Utf32le,
	/// [Standard Compression Scheme for Unicode](https://en.wikipedia.org/wiki/Standard_Compression_Scheme_for_Unicode).
	Scsu,
	/// [Binary Ordered Compression for Unicode](https://en.wikipedia.org/wiki/Binary_Ordered_Compression_for_Unicode).
	Bocu1,
}

impl std::fmt::Display for Encoding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(
			match self {
				Self::Unknown => "Unknown",
				Self::Ansi => "ANSI",
				Self::Win1252 => "Windows 1252",
				Self::Utf8 => "UTF-8",
				Self::Utf16be => "UTF-16 BE",
				Self::Utf16le => "UTF-16 LE",
				Self::Utf32be => "UTF-32 BE",
				Self::Utf32le => "UTF-32 LE",
				Self::Scsu => "SCSU",
				Self::Bocu1 => "BOCU1",
			},
			f,
		)
	}
}

impl Encoding {
	/// Guesses the encoding of the given raw data, also returning the size of
	/// its [BOM](https://en.wikipedia.org/wiki/Byte_order_mark), if any.
	#[must_use]
	pub fn guess(data: &[u8]) -> (Self, usize) {
		if let Some((enc, bom_sz)) = Self::guess_bom(data) {
			return (enc, bom_sz); // BOM found, we already guessed the encoding
		}

		if Self::guess_utf8(data) {
			return (Self::Utf8, 0);
		}

		let has_non_ansi_char = data.iter().find(|ch| **ch > 0x7f).is_some();
		if has_non_ansi_char {
			(Self::Win1252, 0) // by exclusion, not assertive
		} else {
			(Self::Ansi, 0)
		}
	}

	fn guess_bom(data: &[u8]) -> Option<(Self, usize)> {
		let has_bom = |bom_bytes: &[u8]| -> bool {
			data.len() >= bom_bytes.len()
				&& data[..bom_bytes.len()].cmp(bom_bytes) == Ordering::Equal
		};

		const UTF8: [u8; 3] = [0xef, 0xbb, 0xbf];
		if has_bom(&UTF8) { // UTF-8 BOM
			return Some((Self::Utf8, UTF8.len()));
		}

		const UTF16BE: [u8; 2] = [0xfe, 0xff];
		if has_bom(&UTF16BE) {
			return Some((Self::Utf16be, UTF16BE.len()));
		}

		const UTF16LE: [u8; 2] = [0xff, 0xfe];
		if has_bom(&UTF16LE) {
			return Some((Self::Utf16le, UTF16LE.len()));
		}

		const UTF32BE: [u8; 4] = [0x00, 0x00, 0xfe, 0xff];
		if has_bom(&UTF32BE) {
			return Some((Self::Utf32be, UTF32BE.len()));
		}

		const UTF32LE: [u8; 4] = [0xff, 0xfe, 0x00, 0x00];
		if has_bom(&UTF32LE) {
			return Some((Self::Utf32le, UTF32LE.len()));
		}

		const SCSU: [u8; 3] = [0x0e, 0xfe, 0xff];
		if has_bom(&SCSU) {
			return Some((Self::Scsu, SCSU.len()));
		}

		const BOCU1: [u8; 3] = [0xfb, 0xee, 0x28];
		if has_bom(&BOCU1) {
			return Some((Self::Bocu1, BOCU1.len()));
		}

		None // no BOM found
	}

	fn guess_utf8(data: &[u8]) -> bool {
		let mut i = 0; // https://stackoverflow.com/a/1031773/6923555
		while i < data.len() {
			let ch0 = unsafe { *data.get_unchecked(i) };

			if ch0 == 0x00 { // end of string
				break;
			}

			if ch0 == 0x09 || // ASCII
				ch0 == 0x0a ||
				ch0 == 0x0d ||
				(0x20 <= ch0 && ch0 <= 0x7e)
			{
				i += 1;
				continue;
			}

			if i < data.len() - 1 {
				let ch1 = unsafe { *data.get_unchecked(i + 1) };

				if (0xc2 <= ch0 && ch0 <= 0xdf) && // non-overlong 2-byte
					(0x80 <= ch1 && ch1 <= 0xbf)
				{
					i += 2;
					continue;
				}

				if i < data.len() - 2 {
					let ch2 = unsafe { *data.get_unchecked(i + 2) };

					if (ch0 == 0xe0 && // excluding overlongs
							(0xa0 <= ch1 && ch1 <= 0xbf) &&
							(0x80 <= ch2 && ch2 <= 0xbf)
						) ||
						(
							(
								(0xe1 <= ch0 && ch0 <= 0xec) || // straight 3-byte
								ch0 == 0xee ||
								ch0 == 0xef
							) &&
							(0x80 <= ch1 && ch1 <= 0xbf) &&
							(0x80 <= ch2 && ch2 <= 0xbf)
						) ||
						(ch0 == 0xed && // excluding surrogates
							(0x80 <= ch1 && ch1 <= 0x9f) &&
							(0x80 <= ch2 && ch2 <= 0xbf)
						)
					{
						i += 3;
						continue;
					}

					if i < data.len() - 3 {
						let ch3 = unsafe { *data.get_unchecked(i + 3) };

						if (ch0 == 0xf0 && // planes 1-3
								(0x90 <= ch1 && ch1 <= 0xbf) &&
								(0x80 <= ch2 && ch2 <= 0xbf) &&
								(0x80 <= ch3 && ch3 <= 0xbf)
							) ||
							(
								(0xf1 <= ch0 && ch0 <= 0xf3) && // planes 4-15
								(0x80 <= ch1 && ch1 <= 0xbf) &&
								(0x80 <= ch2 && ch2 <= 0xbf) &&
								(0x80 <= ch3 && ch3 <= 0xbf)
							) ||
							(
								ch0 == 0xf4 && // plane 16
								(0x80 <= ch1 && ch1 <= 0x8f) &&
								(0x80 <= ch2 && ch2 <= 0xbf) &&
								(0x80 <= ch3 && ch3 <= 0xbf)
							)
						{
							i += 4;
							continue;
						}
					}
				}
			}

	 		return false; // none of the conditions were accepted, not UTF-8
		}
		true // all the conditions accepted through the whole string
	}
}
