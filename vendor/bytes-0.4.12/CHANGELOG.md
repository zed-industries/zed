# 0.4.12 (March 6, 2018)

### Added
- Implement `FromIterator<&'a u8>` for `BytesMut`/`Bytes` (#244).
- Implement `Buf` for `VecDeque` (#249).

# 0.4.11 (November 17, 2018)

* Use raw pointers for potentially racy loads (#233).
* Implement `BufRead` for `buf::Reader` (#232).
* Documentation tweaks (#234).

# 0.4.10 (September 4, 2018)

* impl `Buf` and `BufMut` for `Either` (#225).
* Add `Bytes::slice_ref` (#208).

# 0.4.9 (July 12, 2018)

* Add 128 bit number support behind a feature flag (#209).
* Implement `IntoBuf` for `&mut [u8]`

# 0.4.8 (May 25, 2018)

* Fix panic in `BytesMut` `FromIterator` implementation.
* Bytes: Recycle space when reserving space in vec mode (#197).
* Bytes: Add resize fn (#203).

# 0.4.7 (April 27, 2018)

* Make `Buf` and `BufMut` usable as trait objects (#186).
* impl BorrowMut for BytesMut (#185).
* Improve accessor performance (#195).

# 0.4.6 (Janary 8, 2018)

* Implement FromIterator for Bytes/BytesMut (#148).
* Add `advance` fn to Bytes/BytesMut (#166).
* Add `unsplit` fn to `BytesMut` (#162, #173).
* Improvements to Bytes split fns (#92).

# 0.4.5 (August 12, 2017)

* Fix range bug in `Take::bytes`
* Misc performance improvements
* Add extra `PartialEq` implementations.
* Add `Bytes::with_capacity`
* Implement `AsMut[u8]` for `BytesMut`

# 0.4.4 (May 26, 2017)

* Add serde support behind feature flag
* Add `extend_from_slice` on `Bytes` and `BytesMut`
* Add `truncate` and `clear` on `Bytes`
* Misc additional std trait implementations
* Misc performance improvements

# 0.4.3 (April 30, 2017)

* Fix Vec::advance_mut bug
* Bump minimum Rust version to 1.15
* Misc performance tweaks

# 0.4.2 (April 5, 2017)

* Misc performance tweaks
* Improved `Debug` implementation for `Bytes`
* Avoid some incorrect assert panics

# 0.4.1 (March 15, 2017)

* Expose `buf` module and have most types available from there vs. root.
* Implement `IntoBuf` for `T: Buf`.
* Add `FromBuf` and `Buf::collect`.
* Add iterator adapter for `Buf`.
* Add scatter/gather support to `Buf` and `BufMut`.
* Add `Buf::chain`.
* Reduce allocations on repeated calls to `BytesMut::reserve`.
* Implement `Debug` for more types.
* Remove `Source` in favor of `IntoBuf`.
* Implement `Extend` for `BytesMut`.


# 0.4.0 (February 24, 2017)

* Initial release
