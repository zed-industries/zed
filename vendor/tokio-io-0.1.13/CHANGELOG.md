# 0.1.13 (February 4, 2020)

* Add `tokio 0.2.x` deprecation notice.

# 0.1.12 (March 1, 2019)

### Added
- Add `unsplit` to join previously split `AsyncRead + AsyncWrite` (#807).

# 0.1.11 (January 6, 2019)

* Fix minor error in Decoder::decode API documentation (#797).

# 0.1.10 (October 23, 2018)

* Expose inner codec from `Framed` (#686).
* Implement AsyncRead::prepare_uninitialized_buffer for Take and Chain (#678).

# 0.1.9 (September 27, 2018)

* Fix bug in `AsyncRead::split()` (#655).
* Fix non-terminating loop in `length_delimited::FramedWrite` (#576).

# 0.1.8 (August 23, 2018)

* Documentation improvements

# 0.1.7 (June 13, 2018)

* Move `codec::{Encode, Decode, Framed*}` into `tokio-codec` (#353)

# 0.1.6 (March 09, 2018)

* Add native endian builder fn to length_delimited (#144)
* Add AsyncRead::poll_read, AsyncWrite::poll_write (#170)

# 0.1.5 (February 07, 2018)

* Fix bug in `BytesCodec` and `LinesCodec`.
* Performance improvement to `split`.

# 0.1.4 (November 10, 2017)

* Use `FrameTooBig` as length delimited error type (#70).
* Provide `Bytes` and `Lines` codecs (#78).
* Provide `AllowStdIo` wrapper (#76).

# 0.1.3 (August 14, 2017)

* Fix bug involving zero sized writes in copy helper (#57).
* Add get / set accessors for length delimited max frame length setting. (#65).
* Add `Framed::into_parts_and_codec` (#59).

# 0.1.2 (May 23, 2017)

* Add `from_parts` and `into_parts` to the framing combinators.
* Support passing an initialized buffer to the framing combinators.
* Add `length_adjustment` support to length delimited encoding (#48).

# 0.1.1 (March 22, 2017)

* Add some omitted `Self: Sized` bounds.
* Add missing "inner" fns.

# 0.1.0 (March 15, 2017)

* Initial release
