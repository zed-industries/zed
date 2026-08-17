/*!
This library implements probabilistic Locality Sensitive Hashing algorithms
for indexing and searching text documents.
* [MinHash](https://en.wikipedia.org/wiki/MinHash)
* [SimHash](https://en.wikipedia.org/wiki/SimHash)

Main use cases for gaoya are clustering and deduplication


## Example

 ```
 use gaoya::minhash::{MinHashIndex, MinHasher32, MinHasher} ;
 use gaoya::text::whitespace_split;
 use std::collections::HashSet;
 let corpus = [
     "This is the first document.",
     "This document is the second document.",
     "And this is the third document.",
     "Is this the first document?",
     "This not the first nor the second nor the third, but the fourth document"];
 let (num_bands, band_width) = (42, 3);
 let minhasher = MinHasher32::new(num_bands * band_width);
 let mut index = MinHashIndex::new(num_bands, band_width, 0.5);
 for (i, doc) in corpus.iter().enumerate() {
     index.insert(i, minhasher.create_signature(whitespace_split(&doc.to_lowercase())));
 }
 for (i, doc) in corpus.iter().enumerate() {
     if i < 4 {
         let mut expected = HashSet::default();
         expected.extend(vec![0, 1, 2, 3].into_iter());
         assert_eq!(index.query_owned(&minhasher.create_signature(whitespace_split(&doc.to_lowercase()))), expected);
     } else {
         let mut expected = HashSet::default();
         expected.insert(4);
         assert_eq!(index.query_owned(&minhasher.create_signature(whitespace_split(&doc.to_lowercase()))), expected);
     }
 }

 ```

## References
[[1] Chapter 3, Mining of Massive Datasets](http://www.mmds.org)

[[2] Similarity Estimation Techniques from Rounding Algorithms](https://www.cs.princeton.edu/courses/archive/spr04/cos598B/bib/CharikarEstim.pdf)

[[3] Detecting Near-Duplicates for Web Crawling](https://static.googleusercontent.com/media/research.google.com/en//pubs/archive/33026.pdf)

 */
#![allow(dead_code)]
#![allow(unused)]
#![cfg_attr(feature = "unstable", feature(hash_raw_entry))]
//#![feature(get_mut_unchecked)]

pub mod minhash;
pub mod simhash;
pub mod text;
pub mod clustering;
