## Version 0.2.54

- Add simple encoder
- Fix no_std compilation

## Version 0.2.52

- Add small fix for refilling where the decoder lacked bits

## Version 0.2.51

- Correctly check for limits in the inner loop

## Version 0.2.0

- Initial release

## Version 0.2.1

- Fix bug where raw deflate outputs would cause errors.

## Version 0.2.2

- Fix bug in which some paths would cause the stream not to refill

## Version 0.2.3

- Small performance improvements, especially on files with a lot of RLE redundant data

## Version 0.2.4

- Fix bug with some gzip that would cause errors during decoding
- Small performance improvement

## Version 0.2.41

- Improve documentation of exposed values

## Version 0.2.42

- Remove broken links in README.

## Version 0.2.50

- Mark library as `#[no_std]`
- Impl `std::error::Error` for library