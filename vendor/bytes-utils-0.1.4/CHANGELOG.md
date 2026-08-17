# 0.1.4

* Deps bumps.
* Fix no-std build on either by bumping edition.

# 0.1.3

* Serde support for the stringly wrappers.
* `string::from_static`.

# 0.1.2

* No-std support.

# 0.1.1

* Implement the `chunk_vectored` in a way it returns more than one chunk if
  possible.
* SegmentedSlice added, for avoiding allocation of the VecDeque in SegmentedBuf.

# 0.1.0

* Initial code for the SegmentedBuf.
* Initial code for the Str/StrMut string wrappers.
