# Gaoya


## About
This project implements Locality Sensitive Hashing algorithms and data structures for indexing and querying text documents.
The primary use cases for Gaoya are deduplication and clustering. 

## Main Features
* 64,32,16,8 bit minhash
* 64,128 bit simhash
* Fast implementation in Rust
* Multi-threaded thanks to [rayon](https://github.com/rayon-rs/rayon)
* Python bindings



### Python Example

```python
>>> import gaoya
>>> index = gaoya.minhash.MinHashStringIndex(hash_size=32, 
                                             jaccard_threshold=0.5, 
                                             num_bands=42, 
                                             band_size=3,
                                             num_hashes=42*3,
                                             analyzer='word', 
                                             lowercase=True, 
                                             ngram_range=(1,1))
>>> corpus = [
...     'This is the first document.',
...     'This document is the second document.',
...     'And this is the third document.',
...     'Is this the first document?',
...     'This not the first nor the second nor the third, but the fourth document'
... ]
>>> 
>>> for i, doc in enumerate(corpus): index.insert_document(i, doc)
... 
>>> index.query('This is the first document.')
[0, 1, 2, 3]
>>> 
```

### Installation
```
$ pip3 install gaoya
```

### Examples
[Document Deduplication with Gaoya](https://github.com/serega/gaoya/blob/master/py-gaoya/examples/deduplication_scholarly_articles_gaoya.ipynb)


### Rust Example

```rust
use gaoya::minhash::{MinHashIndex, MinHasher32, MinHasher} ;
use gaoya::text::whitespace_split;
use fxhash::FxHashSet;
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
        let mut expected = FxHashSet::default();
        expected.extend(vec![0, 1, 2, 3].into_iter());
        let signature = minhasher.create_signature(whitespace_split(&doc.to_lowercase()));
        assert_eq!(index.query_owned(&signature), expected);
    } else {
        let mut expected = FxHashSet::default();
        expected.insert(4);
        let signature = minhasher.create_signature(whitespace_split(&doc.to_lowercase()));
        assert_eq!(index.query_owned(&signature), expected);
    }
}

```


## References
[[1] Chapter 3, Mining of Massive Datasets](http://www.mmds.org)

[[2] Similarity Estimation Techniques from Rounding Algorithms](https://www.cs.princeton.edu/courses/archive/spr04/cos598B/bib/CharikarEstim.pdf)

[[3] Detecting Near-Duplicates for Web Crawling](https://static.googleusercontent.com/media/research.google.com/en//pubs/archive/33026.pdf)