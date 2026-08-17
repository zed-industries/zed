// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub struct Utf8AwareChunks<'a> {
    bytes: &'a [u8],
    chunk_size: usize,
}

impl<'a> Utf8AwareChunks<'a> {
    fn new(bytes: &'a [u8], chunk_size: usize) -> Self {
        if chunk_size < 4 {
            panic!("chunk_size must be at least 4 due to utf8 encoding rules");
        }
        Utf8AwareChunks { bytes, chunk_size }
    }
}

impl<'a> Iterator for Utf8AwareChunks<'a> {
    type Item = &'a [u8];

    /// Uses the same algorithm as in the LiveKit JS SDK.
    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }

        if self.bytes.len() <= self.chunk_size {
            let chunk = self.bytes;
            self.bytes = &[];
            return Some(chunk);
        }

        let mut k = self.chunk_size;
        while k > 0 {
            let byte = self.bytes[k];
            if (byte & 0xc0) != 0x80 {
                break;
            }
            k -= 1;
        }

        let chunk = &self.bytes[..k];
        self.bytes = &self.bytes[k..];
        Some(chunk)
    }
}

pub trait Utf8AwareChunkExt {
    /// Splits the bytes into chunks of the specified size, ensuring that
    /// UTF-8 character boundaries are respected.
    fn utf8_aware_chunks(&self, chunk_size: usize) -> Utf8AwareChunks<'_>;
}

impl Utf8AwareChunkExt for [u8] {
    fn utf8_aware_chunks(&self, chunk_size: usize) -> Utf8AwareChunks<'_> {
        Utf8AwareChunks::new(self, chunk_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_chunking() {
        let test_string = "Hello, World!".as_bytes();
        let chunks: Vec<&[u8]> = test_string.utf8_aware_chunks(4).collect();
        assert_eq!(
            chunks,
            [&[72, 101, 108, 108], &[111, 44, 32, 87], &[111, 114, 108, 100], &[33][..]]
        );
    }

    #[test]
    fn test_empty_string_chunking() {
        let empty_string = "".as_bytes();
        let chunks: Vec<&[u8]> = empty_string.utf8_aware_chunks(5).collect();
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_single_character_string_chunking() {
        let single_char = "X".as_bytes();
        let chunks: Vec<&[u8]> = single_char.utf8_aware_chunks(5).collect();
        assert_eq!(chunks, [&[88]]);
    }

    #[test]
    fn test_mixed_string_chunking() {
        let mixed_string = "Hello 👋".as_bytes();
        let chunks: Vec<&[u8]> = mixed_string.utf8_aware_chunks(4).collect();
        assert_eq!(
            chunks,
            [&[0x48, 0x65, 0x6C, 0x6C], &[0x6F, 0x20][..], &[0xF0, 0x9F, 0x91, 0x8B]]
        );
    }
}
