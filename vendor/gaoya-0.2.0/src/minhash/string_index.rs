use crate::minhash::min_hasher64::MinHasher64V1;
use crate::minhash::{MinHasher, MinHashIndex};
use crate::text::whitespace_split;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter, write};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::io::{Read, Write};
use fnv::FnvBuildHasher;


pub struct MinHashStringIndex {
    lsh_index: MinHashIndex<u64, u64>,
    min_hash: MinHasher64V1<FnvBuildHasher>,
    doc_map: HashMap<u64, String>,
    doc_id: u64,
}

impl Display for MinHashStringIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "MinHashStringIndex {{ ");
        self.lsh_index.fmt(f);
        write!(f, " }} ")
    }
}

impl MinHashStringIndex {

    pub fn new(num_bands: usize, band_width: usize, jaccard_threshold: f64) -> Self {
        MinHashStringIndex {
            lsh_index: MinHashIndex::new(num_bands, band_width, jaccard_threshold),
            min_hash: MinHasher64V1::new(num_bands * band_width),
            doc_map: HashMap::new(),
            doc_id: 0,
        }
    }

    pub fn insert(&mut self, text: String) {
        let min_hashes = self.min_hash.create_signature(whitespace_split(text.as_str()));
        self.doc_id += 1;
        self.doc_map.insert(self.doc_id, text);
        self.lsh_index.insert(self.doc_id, min_hashes);
    }

    pub fn query(&self, text: &str) -> Vec<&String> {
        let min_hashes = self.min_hash.create_signature(whitespace_split(text));
        let ids = self.lsh_index.query(&min_hashes);
        ids.iter().map(|id| self.doc_map.get(id).unwrap()).collect()
    }

    pub fn load_from_lines<R: Read>(&mut self, reader: &mut BufReader<R>) {
        for line_result in reader.lines() {
            match line_result {
                Ok(line) => self.insert(line),
                Err(e) => (),
            }
        }
    }

    pub fn load_from_file(&mut self, file_name: &str) -> Result<usize, Error> {
        match File::open(file_name) {
            Ok(file) => {
                let current_size = self.size();
                let mut reader: BufReader<File> = BufReader::new(file);
                self.load_from_lines(&mut reader);
                let new_count = self.size() - current_size;
                Ok(new_count)
            }
            Err(e) => Err(e),
        }
    }

    pub fn load_from_file_parallel(&mut self, file_name: &str) -> Result<usize, Error> {
        match File::open(file_name) {
            Ok(file) => {
                let current_size = self.size();
                let mut reader: BufReader<File> = BufReader::new(file);
                let lines: Vec<(u64, String)> = reader
                    .lines()
                    .enumerate()
                    .map(|v| (v.0 as u64 + self.doc_id, v.1.unwrap()))
                    .collect();
                let minhashes = lines
                    .par_iter()
                    .map(|line| {
                        (
                            line.0,
                            self.min_hash.create_signature(whitespace_split(&line.1)),
                        )
                    })
                    .collect();
                self.lsh_index.par_bulk_insert_pairs(minhashes);
                self.doc_id += lines.len() as u64;
                for line in lines {
                    self.doc_map.insert(line.0, line.1);
                }
                let new_count = self.size() - current_size;
                Ok(new_count)
            }
            Err(e) => Err(e),
        }
    }

    pub fn size(&self) -> usize {
        self.doc_id as usize
    }
}

#[cfg(test)]
mod tests {
    use super::MinHashStringIndex;
    use std::io::{BufReader, Read, Write};

    #[test]
    fn test_load_from_file() {
        let strings: Vec<String> = [
            "locality sensitive hashing is a cool algorithm",
            "locality sensitive hashing is a great algorithm",
            "locality sensitive hashing is a awesome algorithm",
            "we all scream for ice cream",
            "we all scream for ice cream",
            "we all scream for ice cream sandwich",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        // Create fake "file"
        let mut file = Vec::new();

        // Write into the "file"
        for bytes in strings.iter().map(|s| s.as_bytes()) {
            file.write_all(&bytes).unwrap();
            file.write_all("\n".as_bytes()).unwrap();
        }
        let mut lsh_index = MinHashStringIndex::new(42, 4, 0.5);

        lsh_index.load_from_lines(&mut BufReader::new(file.as_slice()));
        assert_eq!(6, lsh_index.size());

        println!("{}", lsh_index);

        let result = lsh_index.query(&strings[0]);
        assert_eq!(result.len(), 3);

        assert!(result.contains(&(&strings[0])));
        assert!(result.contains(&(&strings[1])));
        assert!(result.contains(&(&strings[2])));
    }
}
