<a name="v4.6.7"></a>
### v4.6.7 (2024-04-10)

* perf: avoid initializing huge buffers in the stream decoder

<a name="v4.6.6"></a>
### v4.6.6 (2022-08-09)

* memchr: use non deprecated feature `std` instead of `use_std`

* refactor: Extract less-generic code from sequence's add_errors (-5%)

<a name="v4.6.4"></a>
### v4.6.4 (2022-04-25)




<a name="v4.6.3"></a>
### v4.6.3 (2022-01-12)


#### Bug Fixes

* **easy:**  Only back-quote parser tokens ([08962d1d](https://github.com/Marwes/combine/commit/08962d1d4287c03edb84bd73a9eb09a42f294bc8), closes [#334](https://github.com/Marwes/combine/issues/334))



<a name="v4.6.3"></a>
### v4.6.3 (2022-01-12)


#### Bug Fixes

* **easy:**  Only back-quote parser tokens ([08962d1d](https://github.com/Marwes/combine/commit/08962d1d4287c03edb84bd73a9eb09a42f294bc8), closes [#334](https://github.com/Marwes/combine/issues/334))



<a name="v4.6.2"></a>
### v4.6.2 (2021-10-28)


#### Performance

*   Handle restarts better in take_until_byte{,s,2,3} ([03b19498](https://github.com/Marwes/combine/commit/03b19498189c873cdf8195a207a9f5a747ca40b7), closes [#327](https://github.com/Marwes/combine/issues/327))

#### Bug Fixes

*   warning ([08753d3e](https://github.com/Marwes/combine/commit/08753d3ebf072c0765e99bd6f440b52ef93cab40))



<a name="v4.6.1"></a>
### v4.6.1 (2021-08-25)


#### Performance

*   Avoid a saturating add in slice_uncons_while ([7f330b0c](https://github.com/Marwes/combine/commit/7f330b0cacd61131df88c919074ffa8136100299))
*   Avoid a saturating add in slice_uncons_while ([ad4180dd](https://github.com/Marwes/combine/commit/ad4180dd7d3530d47502795ead21e13b7816aed7))



<a name="v4.6.0"></a>
## v4.6.0 (2021-06-16)


#### Features

*   Add decode_tokio ([aa20bf64](https://github.com/Marwes/combine/commit/aa20bf641bc5deed7e521de289f2b0963034f750))



<a name="v4.5.2"></a>
### v4.5.2 (2021-01-07)




<a name="v4.5.1"></a>
### v4.5.1 (2020-12-27)


#### Bug Fixes

*   Correct the tokio-02 feature ([466a50d3](https://github.com/Marwes/combine/commit/466a50d3533118cca0bafab48451fd39f92a8233))



<a name="v4.5.0"></a>
## v4.5.0 (2020-12-25)


#### Features

*   Remove pin-project-lite from the 'std' feature ([32ef87b0](https://github.com/Marwes/combine/commit/32ef87b08a643c9814cb9eec6d1f3adfe220c690))
*   Add async decoding for tokio version 1 ([8e91f57d](https://github.com/Marwes/combine/commit/8e91f57d1ddaefafcaf244df7a7eea2096c6d6aa))



<a name="v4.4.0"></a>
## v4.4.0 (2020-11-18)


#### Features

*   Add support for decoding using tokio_03 ([ce1612ff](https://github.com/Marwes/combine/commit/ce1612ffa0a15547bc967474b511ec35eaaf743f))
*   Add a length_prefix combinator ([9e343b2a](https://github.com/Marwes/combine/commit/9e343b2a0067ea70b049369ea1023c364c14b19d), closes [#297](https://github.com/Marwes/combine/issues/297))
*   Add a spanned combinator ([1a70f3f4](https://github.com/Marwes/combine/commit/1a70f3f4219eac71880cbb376e62d3f01c22d981))
*   Add a spanned combinator ([9a8fead8](https://github.com/Marwes/combine/commit/9a8fead85b3eb0d7ee21f80d076af9f59aa408d1))
*   Allow conversion between errors with different token,range,position types ([65fe0af2](https://github.com/Marwes/combine/commit/65fe0af27fa560f90da2723160efd2d1782bea4f), breaks [#](https://github.com/Marwes/combine/issues/))

#### Breaking Changes

*   Allow conversion between errors with different token,range,position types ([65fe0af2](https://github.com/Marwes/combine/commit/65fe0af27fa560f90da2723160efd2d1782bea4f), breaks [#](https://github.com/Marwes/combine/issues/))



<a name="v4.3.2"></a>
### v4.3.2 (2020-09-08)




<a name="v4.3.1"></a>
### v4.3.1 (2020-08-26)


#### Bug Fixes

*   Make the decode macros work for redis ([82b908d8](https://github.com/Marwes/combine/commit/82b908d80ac21aeab27b169a1f1857a68a6bb11c))



<a name="v4.3.0"></a>
## v4.3.0 (2020-07-10)


#### Features

*   Specialize decoding on BufReader ([9559e114](https://github.com/Marwes/combine/commit/9559e114658b60a59d570ffd78f4d7ecb597b814))
*   Add a BufReader abstraction for Decoder ([d65a23e8](https://github.com/Marwes/combine/commit/d65a23e81f482c6a5482e53d63c17fcbcf67d623))
*   Allow tuple structs and functions in struct_parser! ([48a16f6b](https://github.com/Marwes/combine/commit/48a16f6bf84939da9eacb1f82b4cff6ca7f324e0))



<a name="v4.2.1"></a>
### v4.2.1 (2020-05-20)


#### Performance

*   Use size_hint in iterate ([b2649e3b](https://github.com/Marwes/combine/commit/b2649e3b467fc84ef48a91d56ec1fcb40291978e))
*   Pre-allocate collections for count* parsers ([5f37857c](https://github.com/Marwes/combine/commit/5f37857c988b81c2b0613d6f2ab14576c794ae99))



<a name="v4.2.0"></a>
## v4.2.0 (2020-05-17)


#### Features

*   Add any_send_sync_partial_state ([7e90807a](https://github.com/Marwes/combine/commit/7e90807a0949411b6aaf24a677e6530a530a1478))



<a name="v4.1.0"></a>
## v4.1.0 (2020-04-22)




<a name="4.0.1"></a>
### 4.0.1 (2020-01-23)


#### Breaking Changes

*   Make the decode macros take read by parameter ([aa813fb1](https://github.com/Marwes/combine/commit/aa813fb1b486ecdc5258bf2c89e0b18a8f4fc876), breaks [#](https://github.com/Marwes/combine/issues/))

#### Features

*   Make the decode macros take read by parameter ([aa813fb1](https://github.com/Marwes/combine/commit/aa813fb1b486ecdc5258bf2c89e0b18a8f4fc876), breaks [#](https://github.com/Marwes/combine/issues/))

#### Bug Fixes

*   Don't block decoding if there are more data in the buffer ([6659f993](https://github.com/Marwes/combine/commit/6659f993784876bdc5d6f6145aaffe2844ada760))
*   Don't try to read into the entire buffer for std decoding ([d613dc93](https://github.com/Marwes/combine/commit/d613dc937de6e0b745f8cbcca974e3cfcb3db723))



<a name="4.0.0"></a>
## 4.0.0 (2020-01-16)


#### Features

*   Relax the decode*! macros to no longer need BufRead ([bc5b7794](https://github.com/Marwes/combine/commit/bc5b779491c66a18289bc4b237d281bd391b1d69))
*   Add decode_futures_03_buf_read ([f403ecda](https://github.com/Marwes/combine/commit/f403ecda68e5d7c5d5cf89d8feb26aa1b715609e))
*   Add the produce parser ([ac15b87c](https://github.com/Marwes/combine/commit/ac15b87c4eb23065920b3bb7ad8d590f29937895))



<a name="4.0.0-beta.2"></a>
## 4.0.0-beta.2 (2019-12-19)


#### Features

*   Increase tuple parsers to 20 elements ([45781fea](https://github.com/Marwes/combine/commit/45781feac3cd2da252fcc999aa69c9a207af5f6a))
*   Remove deprecated re-exports ([b4e23207](https://github.com/Marwes/combine/commit/b4e23207f9c3e47111c69865389853d89b8ce12d), breaks [#](https://github.com/Marwes/combine/issues/))
*   Add decode_tokio_buf_read! to parse tokio::io::BufRead ([4d38f7c2](https://github.com/Marwes/combine/commit/4d38f7c25cb0cde10cdc869955c0a98079b5fa08))
*   Add `decode_buf_read!` to parse `BufRead` without ([dbe23ce9](https://github.com/Marwes/combine/commit/dbe23ce90803d409c591b02aaeb7005d0f58622a))
*   Add a macro to incrementally decode std::io::BufRead ([6e2f1121](https://github.com/Marwes/combine/commit/6e2f1121203c54b1623f9f5a8b35907867c83874))
*   Rename Consumed to Commit and Empty to Peek ([129046e3](https://github.com/Marwes/combine/commit/129046e3a555318c3e60f658988f1be8fe83a2fd), breaks [#](https://github.com/Marwes/combine/issues/))
*   Rename Consumed{Ok,Err} to Commit{Ok,Err} and Empty{Ok,Err} to Peek{Ok,Err} ([0ac1fd4f](https://github.com/Marwes/combine/commit/0ac1fd4fee89bd540b38b0d3224e0dcf5260ab77))
*   Add a dedicated error for ReadStream ([37bbd843](https://github.com/Marwes/combine/commit/37bbd84383ecaa632df537322426407be7712748))

#### Bug Fixes

*   Handle partial parsing in sep_end_by ([281e0d30](https://github.com/Marwes/combine/commit/281e0d30276cd8d2b730680fb24117bb1b72b198))

#### Breaking Changes

*   Remove deprecated re-exports ([b4e23207](https://github.com/Marwes/combine/commit/b4e23207f9c3e47111c69865389853d89b8ce12d), breaks [#](https://github.com/Marwes/combine/issues/))
*   Rename Consumed to Commit and Empty to Peek ([129046e3](https://github.com/Marwes/combine/commit/129046e3a555318c3e60f658988f1be8fe83a2fd), breaks [#](https://github.com/Marwes/combine/issues/))



<a name="4.0.0-beta.1"></a>
## 4.0.0-beta.1 (2019-10-08)


#### Bug Fixes

*   Handle partial parsing in the num parsers ([47764c7f](https://github.com/Marwes/combine/commit/47764c7feb8becefd1d4c376fc11492ed3f3cd6a))
*   Don't bind the input lifetime to the parser with expected/message ([618c69e0](https://github.com/Marwes/combine/commit/618c69e09afc383dadc9af305394ca82948801c3))

#### Breaking Changes

*   Merge FullRangeStream into RangeStreamOnce ([c160a971](https://github.com/Marwes/combine/commit/c160a971b47c29a0c8d37fbca6f77cd4a6c85831), breaks [#](https://github.com/Marwes/combine/issues/))
*   Rename `StreamOnce::Item` to `Token` ([74a0bbd3](https://github.com/Marwes/combine/commit/74a0bbd363214047236ae88e76bd7e7a7cdc265f), breaks [#](https://github.com/Marwes/combine/issues/), [#](https://github.com/Marwes/combine/issues/))
*   Minimum rust version is now 1.32 ([7b77508c](https://github.com/Marwes/combine/commit/7b77508c05f2075e7795b6026e419a2a57666a1c), breaks [#](https://github.com/Marwes/combine/issues/))
*   Only require `&mut I` in decode ([e154dbc2](https://github.com/Marwes/combine/commit/e154dbc2f2ca3288e6733f29c85f4b1efcd689c7), breaks [#](https://github.com/Marwes/combine/issues/))
*   Generalize factory to take the Input ([19b1a73e](https://github.com/Marwes/combine/commit/19b1a73ed3516d14b392aefeba0363d01937be22), breaks [#](https://github.com/Marwes/combine/issues/))

#### Features

*   Merge FullRangeStream into RangeStreamOnce ([c160a971](https://github.com/Marwes/combine/commit/c160a971b47c29a0c8d37fbca6f77cd4a6c85831), breaks [#](https://github.com/Marwes/combine/issues/))
*   Rename `StreamOnce::Item` to `Token` ([74a0bbd3](https://github.com/Marwes/combine/commit/74a0bbd363214047236ae88e76bd7e7a7cdc265f), breaks [#](https://github.com/Marwes/combine/issues/), [#](https://github.com/Marwes/combine/issues/))
*   Allow fmt::Display be used to specify errors in combinators ([82796d4a](https://github.com/Marwes/combine/commit/82796d4ae5ea23290920ae4e1586b20535285d49), closes [#255](https://github.com/Marwes/combine/issues/255))
*   Minimum rust version is now 1.32 ([7b77508c](https://github.com/Marwes/combine/commit/7b77508c05f2075e7795b6026e419a2a57666a1c), breaks [#](https://github.com/Marwes/combine/issues/))
*   Only require `&mut I` in decode ([e154dbc2](https://github.com/Marwes/combine/commit/e154dbc2f2ca3288e6733f29c85f4b1efcd689c7), breaks [#](https://github.com/Marwes/combine/issues/))
*   Add RepeatUntil ([7f4a310f](https://github.com/Marwes/combine/commit/7f4a310f8edaebe7d3bd450aca7a2823fcf2e5d6))
*   Generalize factory to take the Input ([19b1a73e](https://github.com/Marwes/combine/commit/19b1a73ed3516d14b392aefeba0363d01937be22), breaks [#](https://github.com/Marwes/combine/issues/))
*   Add the iterate parser ([342a45b4](https://github.com/Marwes/combine/commit/342a45b484240ace2e313138b2818c95cbec3427))
*   Add dispatch! ([0740ce0f](https://github.com/Marwes/combine/commit/0740ce0fd2ade2bd19981261dfeb89cae1f63120))
*   impl Stream* for &mut T ([51e7e2b7](https://github.com/Marwes/combine/commit/51e7e2b736f3b3ab8d9ec9877e2afa8fc31f5207))
*   Add `From<S>` for `easy::Stream<S>` ([8f695cc6](https://github.com/Marwes/combine/commit/8f695cc62268a855c3c3847661f6ffe8308b745e))
*   Allow ReadStream parsers to use &[u8] in errors ([6b62a857](https://github.com/Marwes/combine/commit/6b62a857f441ab930e2f595aff3e87f992c769b8), closes [#249](https://github.com/Marwes/combine/issues/249))



<a name="4.1.0-alpha.2"></a>
## 4.0.0-alpha.2 (2019-06-17)


#### Features

*   impl Stream* for &mut T ([51e7e2b7](https://github.com/Marwes/combine/commit/51e7e2b736f3b3ab8d9ec9877e2afa8fc31f5207))
*   Add `From<S>` for `easy::Stream<S>` ([8f695cc6](https://github.com/Marwes/combine/commit/8f695cc62268a855c3c3847661f6ffe8308b745e))
*   Allow ReadStream parsers to use `&[u8]` in errors ([6b62a857](https://github.com/Marwes/combine/commit/6b62a857f441ab930e2f595aff3e87f992c769b8), closes [#249](https://github.com/Marwes/combine/issues/249))



<a name="4.0.0-alpha.1"></a>
## 4.0.0-alpha.1 (2019-05-07)


#### Breaking Changes

*   Drop support for regex 0.2 ([bf6aeb06](https://github.com/Marwes/combine/commit/bf6aeb06494abe2f1890c5bf90db86ac01ec9772), closes [#247](https://github.com/Marwes/combine/issues/247), breaks [#](https://github.com/Marwes/combine/issues/))
*   Rename tokens2 to tokens and tokens to tokens_cmp ([3dadbb4f](https://github.com/Marwes/combine/commit/3dadbb4f2adb0447c883ea56bdbfcfd53c58d384), closes [#166](https://github.com/Marwes/combine/issues/166), breaks [#](https://github.com/Marwes/combine/issues/))
*   Remove the std::result::Result returning parse functions ([6ec094ef](https://github.com/Marwes/combine/commit/6ec094efd2eecefeb2281fb99687143e7a5580e8), closes [#244](https://github.com/Marwes/combine/issues/244), breaks [#](https://github.com/Marwes/combine/issues/))
*   Make PointerOffset easier to understand and use ([5f6d65b3](https://github.com/Marwes/combine/commit/5f6d65b30679d39c4c8a41ef0877bca8dc199095), closes [#238](https://github.com/Marwes/combine/issues/238), breaks [#](https://github.com/Marwes/combine/issues/))
*   Allow reset to return errors ([3055c810](https://github.com/Marwes/combine/commit/3055c810fd4904d2eba3f51ea232ef0232f8fbe7), closes [#231](https://github.com/Marwes/combine/issues/231), breaks [#](https://github.com/Marwes/combine/issues/))
*   Remove the PartialEq bound from Item and Range ([24e1087c](https://github.com/Marwes/combine/commit/24e1087cbff4938d48ae3e5947e2eb0dcbb8cc87), closes [#219](https://github.com/Marwes/combine/issues/219), breaks [#](https://github.com/Marwes/combine/issues/))

#### Features

*   Drop support for regex 0.2 ([bf6aeb06](https://github.com/Marwes/combine/commit/bf6aeb06494abe2f1890c5bf90db86ac01ec9772), closes [#247](https://github.com/Marwes/combine/issues/247), breaks [#](https://github.com/Marwes/combine/issues/))
*   Rename tokens2 to tokens and tokens to tokens_cmp ([3dadbb4f](https://github.com/Marwes/combine/commit/3dadbb4f2adb0447c883ea56bdbfcfd53c58d384), closes [#166](https://github.com/Marwes/combine/issues/166), breaks [#](https://github.com/Marwes/combine/issues/))
*   Remove the std::result::Result returning parse functions ([6ec094ef](https://github.com/Marwes/combine/commit/6ec094efd2eecefeb2281fb99687143e7a5580e8), closes [#244](https://github.com/Marwes/combine/issues/244), breaks [#](https://github.com/Marwes/combine/issues/))
*   Allow reset to return errors ([3055c810](https://github.com/Marwes/combine/commit/3055c810fd4904d2eba3f51ea232ef0232f8fbe7), closes [#231](https://github.com/Marwes/combine/issues/231), breaks [#](https://github.com/Marwes/combine/issues/))
*   Remove the PartialEq bound from Item and Range ([24e1087c](https://github.com/Marwes/combine/commit/24e1087cbff4938d48ae3e5947e2eb0dcbb8cc87), closes [#219](https://github.com/Marwes/combine/issues/219), breaks [#](https://github.com/Marwes/combine/issues/))

#### Bug Fixes

*   Don't require macro_use when using parser! in rust 2018 ([72e4c70f](https://github.com/Marwes/combine/commit/72e4c70fda2f283e1feadfd926ed9bb0d384cb59), closes [#241](https://github.com/Marwes/combine/issues/241))
*   Make PointerOffset easier to understand and use ([5f6d65b3](https://github.com/Marwes/combine/commit/5f6d65b30679d39c4c8a41ef0877bca8dc199095), closes [#238](https://github.com/Marwes/combine/issues/238), breaks [#](https://github.com/Marwes/combine/issues/))



<a name="3.8.1"></a>
### 3.8.1 (2019-03-13)


#### Bug Fixes

*   Handle partial parsing in `take_fn/take_until*` ([bae1e3bb](https://github.com/Marwes/combine/commit/bae1e3bbe476fb3fe7c4ff6cc375e50f3d9cfd45))



<a name="3.8.0"></a>
## 3.8.0 (2019-03-12)


#### Features

*   Add take_fn and take_until_bytes ([5f560780](https://github.com/Marwes/combine/commit/5f5607806f8e133485c990419a03de8c7531fa14))



<a name="3.6.7"></a>
### 3.6.7 (2019-02-13)


#### Bug Fixes

*   Forward is_partial in all Stream adaptors ([121b3987](https://github.com/Marwes/combine/commit/121b39879543c074924185dfe17d6b4f434ce413))



<a name="3.6.6"></a>
### 3.6.6 (2019-01-12)

* Fix two inconsisties regarding EOI [#227](https://github.com/Marwes/combine/pull/227)


<a name="3.6.5"></a>
### 3.6.5 (2019-01-09)


#### Bug Fixes

*   not_followed_by should fail on empty successful parses ([aa17e7d2](https://github.com/Marwes/combine/commit/aa17e7d25532eebdb6c447459c8550a4fb6bbe93))
*   Propagate the PartialState through parser! uses ([ae888244](https://github.com/Marwes/combine/commit/ae8882449ee935f7ffb0dd3ef7f0e2d8d6932409), closes [#223](https://github.com/Marwes/combine/issues/223))


<a name="3.6.3"></a>
### 3.6.3 (2018-11-16)




<a name="3.6.2"></a>
### 3.6.2 (2018-11-06)




<a name="3.6.1"></a>
### 3.6.1 (2018-10-13)


#### Bug Fixes

*   Allow clippy to run on stable rust ([6cb00803](https://github.com/Marwes/combine/commit/6cb00803d0135a3849a7f0b35dcf635764b32c06))



<a name="3.6.0"></a>
## 3.6.0 (2018-10-07)


#### Features

*   Deprecates `try` in favor of `attempt` ([a9c79321](https://github.com/Marwes/combine/commit/a9c79321b28ea5332d30429936bfca0c034105ca))
*   Adds `attempt`, a 2018-compatible alias for `try` ([18edaec4](https://github.com/Marwes/combine/commit/18edaec46f67a3803b96bac3762b6f809d0c2724))



<a name="3.5.3"></a>
### 3.5.3 (2018-10-06)


#### Features

*   Allow regex parsers to use regex-1.0 ([77fe362c](https://github.com/Marwes/combine/commit/77fe362c56efa6ce20e3074388aeda33cfe91c4b))
*   pub use unexpected_any ([8b2ca559](https://github.com/Marwes/combine/commit/8b2ca55971e01e8a87ec20ec7b9fe8476023117f))

#### Bug Fixes

*   use unexpected_any inside `then` examples ([f2018db5](https://github.com/Marwes/combine/commit/f2018db5cf5487d02488d017c23f6993e6ed6f82))



<a name="3.5.2"></a>
### 3.5.2 (2018-09-12)


#### Bug Fixes

*   Don't report previous errors from expected ([0048c5ef](https://github.com/Marwes/combine/commit/0048c5ef4577592df94e0c5296bee3a128117211))



<a name="3.5.1"></a>
### 3.5.1 (2018-08-13)


#### Bug Fixes

*   Add all errors of nested choice parsers ([f2b8fbbf](https://github.com/Marwes/combine/commit/f2b8fbbfe300e2b219eb72116856c217fcec8b2b))



<a name="3.5.0"></a>
## 3.5.0 (2018-08-13)


#### Features

*   Add opaque! as convenience over the opaque parser ([9855aa1f](https://github.com/Marwes/combine/commit/9855aa1f39b62b09addb1c7a25035616a8cbef42))
*   Add the factory parser ([fdd38d46](https://github.com/Marwes/combine/commit/fdd38d46b5dbdeece7f6f3a99b12e470a08e9a92))



<a name="3.4.0"></a>
## 3.4.0 (2018-08-04)


#### Features

*   Add the silent combinator ([b9bc28d3](https://github.com/Marwes/combine/commit/b9bc28d32b80644ba5a7fdc6969e7e8e734ee4de))
*   Add the `opaque` parser ([1a1123f5](https://github.com/Marwes/combine/commit/1a1123f5b6970c88d99643d87d75c8fb63117607))
*   Add tokens2 as a simpler version of tokens ([072a8c13](https://github.com/Marwes/combine/commit/072a8c13f90c640d2a98910276bc96d5aa27fae9))
*   Add the from_str combinator ([908f9ebd](https://github.com/Marwes/combine/commit/908f9ebdc3593eda67e8eba8f5b467962076964e))

#### Bug Fixes

*   Make (many1(p), end) report more expected messages ([9e26d38e](https://github.com/Marwes/combine/commit/9e26d38e06c8bac9dbaa547893b8f2aea01f047d))
*   Report more errors in the middle of sequence parsers ([f9e404f6](https://github.com/Marwes/combine/commit/f9e404f6840eb4d42016122095416f76bee5abf3))
*   Report more expected information from nested sequence parsers ([9aa1db92](https://github.com/Marwes/combine/commit/9aa1db92dcd17a35d5775bf78b6642c4e8c89d1d))
*   Use the message variant in from_str to work on no_std ([edf5ff60](https://github.com/Marwes/combine/commit/edf5ff6002d6b6f2f8bd23692dce5084e9fc01dd))
*   Remove redundant Any bound on easy::Errors ([16601046](https://github.com/Marwes/combine/commit/16601046f3c2aa8b4ad75ba2390486f7b5306219), closes [#177](https://github.com/Marwes/combine/issues/177))

<a name="3.3.6"></a>
### 3.3.6

*   Don't forget the state in any_send_partial_state ([4e2eb928](https://github.com/Marwes/combine/commit/4e2eb928e6059a4182b2717bf2bfb7cccdc60127))

<a name="3.3.5"></a>
### 3.3.5 (2018-06-30)


#### Bug Fixes

*   Propagate the ParseMode in range::recognize ([c330a737](https://github.com/Marwes/combine/commit/c330a73746f6adfa22c6b13b15d796d48f589614))



<a name="3.3.4"></a>
### 3.3.4 (2018-06-30)

* fix: Forward the partial mode through the parser! macro correctly


<a name="3.3.3"></a>
### 3.3.3 (2018-06-29)

#### Bug fixes
* Parse from the start in sequence parsers in first mode


<a name="3.3.1"></a>
### 3.3.1 (2018-06-01)


#### Bug Fixes

*   support resuming streams in TakeUntilRange ([b54ff061](https://github.com/Marwes/combine/commit/b54ff0619663aaf7d3c33185b1a4b7ec73cc1f61))
*   reset stream on error in take_until_range ([27449f21](https://github.com/Marwes/combine/commit/27449f2131ecb3d6ef956e2e67b588cae58a9810))
*   support multi-byte `Item`s in take_until_range ([4a690d65](https://github.com/Marwes/combine/commit/4a690d65160ea6e1866fd6dbe8865acae4070c3a))



<a name="v3.3.0"></a>
## v3.3.0 (2018-05-19)


#### Features

*   Add the escaped parser ([0db58a20](https://github.com/Marwes/combine/commit/0db58a20f227d923ffcd3451fbb07ace87dba07c))
*   Add the lazy parser combinator ([496ac836](https://github.com/Marwes/combine/commit/496ac83628ba33ee9a886f989f8749388f918652))

#### Breaking Changes

*   Remove redundant state comparison in Iter ([9d434c3f](https://github.com/Marwes/combine/commit/9d434c3ff89480aeb9c5552e439b465adc28e31d), breaks [#](https://github.com/Marwes/combine/issues/))

#### Performance

*   Specialize uncons_while1 on all streams ([c995ad61](https://github.com/Marwes/combine/commit/c995ad6125996b594b8bf45da078fdebeb41b86a))
*   Unroll the loop for <[T]>::uncons_while ([f593e85d](https://github.com/Marwes/combine/commit/f593e85d4d0ecb9c7c91b769685864fda3291646))
*   Inline from/into for results ([fff248e4](https://github.com/Marwes/combine/commit/fff248e494361e04cedbf849df959193560416ff))
*   Unroll take_while1 ([279a4526](https://github.com/Marwes/combine/commit/279a4526494e1cd7e90252c3bc1cfc8f8c35ebcb))
*   Remove redundant state comparison in Iter ([9d434c3f](https://github.com/Marwes/combine/commit/9d434c3ff89480aeb9c5552e439b465adc28e31d), breaks [#](https://github.com/Marwes/combine/issues/))
*   Add uncons_while1 as a default method on RangeStream ([5d154f15](https://github.com/Marwes/combine/commit/5d154f15a13091c26246627486b309cbdef06d14))



<a name="v3.2.0"></a>
## v3.2.0 (2018-04-24)


#### Features

*   Add any_send_partial_state ([d4153d31](https://github.com/Marwes/combine/commit/d4153d31b074e950f4752f29c8ed188102534e91))



<a name="v3.1.0"></a>
## v3.1.0 (2018-03-26)


#### Features

*   Allow the the `num` parsers to be used without RangeStream ([b1cb0668](https://github.com/Marwes/combine/commit/b1cb0668cc13df3a2f1b6cc35f221089d0279579))
*   Add the take_until parser ([7b03b596](https://github.com/Marwes/combine/commit/7b03b596a58e4cdbe84d6008e277f0cc57394fae))
*   Allow try parsers to be used with partial parsing ([cb2da7ad](https://github.com/Marwes/combine/commit/cb2da7ad74758bc89b17bedd90f2f53ea9f83e7d))



<a name="v3.0.0"></a>
## v3.0.0 (2018-03-18)

3.0.0 is the final stabilization of all the changes made in the 3.x-alpha/beta releases. You can read more about these changes
at https://marwes.github.io/2018/02/08/combine-3.html and https://www.reddit.com/r/rust/comments/6s792a/combine_250_and_300alpha1/

#### Features

*   Let single element tuples work as sequence and choice parsers ([81e34d2d](https://github.com/Marwes/combine/commit/81e34d2d8d823b9962f4036e7576353252f211b9))



<a name="3.0.0-beta.1"></a>
## v3.0.0-beta.1 (2018-02-02)


#### Features

*   Encode parsers as resumable state machines [342fc47](https://github.com/Marwes/combine/commit/342fc4770ee9dc62df51683ccca2e612d6e1ea33)
*   Add the unexpected_any parser ([979e0d7e](https://github.com/Marwes/combine/commit/979e0d7e4ac63c41712352bc87b51001aa067879), closes [#126](https://github.com/Marwes/combine/issues/126))
*   Don't have the error type be generic for uncons* ([df3e84f0](https://github.com/Marwes/combine/commit/df3e84f0275352fba44672b2701d452f6bb55596))
*   Add a alias which helps to refer to the StreamError type ([95eb70cb](https://github.com/Marwes/combine/commit/95eb70cb025aec89925e3f1992a6d1b266328eb2))
*   Add memchr optimized take_until_byte parsers ([30cc7d1d](https://github.com/Marwes/combine/commit/30cc7d1d39e754d2c2e8491eb2123dacec5d30f3))
*   Add the `then_partial` parser ([5d402f6b](https://github.com/Marwes/combine/commit/5d402f6be0c1be4d69159d4df1d1d4589f5cd66e))
*   Don't require `Clone` for `Stream` ([3fc0b540](https://github.com/Marwes/combine/commit/3fc0b540521aff959ce62628df1ac7554a9df861), breaks [#](https://github.com/Marwes/combine/issues/))

#### Breaking Changes

*   Don't require `Clone` for `Stream` ([3fc0b540](https://github.com/Marwes/combine/commit/3fc0b540521aff959ce62628df1ac7554a9df861), breaks [#](https://github.com/Marwes/combine/issues/))

#### Bug Fixes

*   Allow `parser!` to be used within functions ([916bb824](https://github.com/Marwes/combine/commit/916bb824741f054ed2f0686dcce316d0a770d9db))
*   Allow multiple unnamed public parsers to be in the same scope ([c04e2247](https://github.com/Marwes/combine/commit/c04e2247ab04f098ef0eae887c7a7739d99212fc))
*   Return the correct distance for slices with larger than 1 byte items ([6b3c661a](https://github.com/Marwes/combine/commit/6b3c661af23fd221d2b28c37ed207d37409c2491))
*   Don't add extra expected errors when erroring in the middle of a sequence ([44eac24d](https://github.com/Marwes/combine/commit/44eac24d46f265a1b3a94d5587d4f200ebebc18f))

#### Performance

*   Avoid cloning input in satisfy ([9aeaefa9](https://github.com/Marwes/combine/commit/9aeaefa95f97bd8b9d186923e16cd8def98d8e81))



<a name="3.0.0-alpha.4"></a>
## v3.0.0-alpha.4 (2017-10-11)


#### Breaking Changes

*   Rename EasyStream -> Stream, ParsingError => ParseError ... ([d2f4ab14](https://github.com/Marwes/combine/commit/d2f4ab1471cc0616a46bfe965a611d465434d19a), breaks [#](https://github.com/Marwes/combine/issues/))
*   Make the RangeStreamOnce function generic over the returned error ([818d8629](https://github.com/Marwes/combine/commit/818d8629116fec8eef64494a938f0340c04d6ad6), breaks [#](https://github.com/Marwes/combine/issues/))
*   Re-export the type generated by parser! if it is public ([61469f0a](https://github.com/Marwes/combine/commit/61469f0a2db899a1144d0335dd47b9bb8d3105f2), breaks [#](https://github.com/Marwes/combine/issues/))

#### Performance

*   Add inline annotations on error traits ([c8b495b4](https://github.com/Marwes/combine/commit/c8b495b41a21cd71b62782f62bbae77f13f92fb7))

#### Features

*   Rename EasyStream -> Stream, ParsingError => ParseError ... ([d2f4ab14](https://github.com/Marwes/combine/commit/d2f4ab1471cc0616a46bfe965a611d465434d19a), breaks [#](https://github.com/Marwes/combine/issues/))
*   Make the RangeStreamOnce function generic over the returned error ([818d8629](https://github.com/Marwes/combine/commit/818d8629116fec8eef64494a938f0340c04d6ad6), breaks [#](https://github.com/Marwes/combine/issues/))
*   Allow combine to be used in no_std environments ([9fd310ac](https://github.com/Marwes/combine/commit/9fd310ac6b795f8f4152892a698dcf29d9c72b7b))
*   Re-export the type generated by parser! if it is public ([61469f0a](https://github.com/Marwes/combine/commit/61469f0a2db899a1144d0335dd47b9bb8d3105f2), breaks [#](https://github.com/Marwes/combine/issues/))



<a name=""></a>
##  v3.0.0-alpha.3 (2017-08-20)

*   Implement Copy for more types ([e60395d6](https://github.com/Marwes/combine/commit/e60395d683faf52be772d222f28a5d38aec05f5c))



<a name="v3.0.0-alpha.1"></a>
## v3.0.0-alpha.1 (2017-08-07)


#### Features

*   Remove the old State type and Positioner trait ([ae43f8ae](https://github.com/Marwes/combine/commit/ae43f8ae2b303aca3b5ae9fbb1a87475349f2745), breaks [#](https://github.com/Marwes/combine/issues/))
*   Teach the choice parser to take tuples ([96da7ee0](https://github.com/Marwes/combine/commit/96da7ee0cf8a112e60747a0be8a4dbd90efbecba), breaks [#](https://github.com/Marwes/combine/issues/))
*   Add the range_of parser ([7e692086](https://github.com/Marwes/combine/commit/7e69208650f7fdc75279370b193030b09ccdbc7a), closes [#83](https://github.com/Marwes/combine/issues/83), breaks [#](https://github.com/Marwes/combine/issues/))
*   Add map_token and map_range methods to ParseError ([2f92b296](https://github.com/Marwes/combine/commit/2f92b29669b618535bcd7533b7dd39b7daa8579b), closes [#86](https://github.com/Marwes/combine/issues/86))
*   Allow ParseError to be used without the StreamOnce constraint ([520da8e8](https://github.com/Marwes/combine/commit/520da8e89f7162b4d6ba3a3bca05a05f3bd37999), breaks [#](https://github.com/Marwes/combine/issues/))

#### Bug Fixes

*   Remove depreceated items ([9107342a](https://github.com/Marwes/combine/commit/9107342a89a5efc664bac9c2919a93a992ca6809), breaks [#](https://github.com/Marwes/combine/issues/))
*   Don't forward tuple parsers to frunk to prevent a performance loss ([7e27c523](https://github.com/Marwes/combine/commit/7e27c523da46828b254ee4fc7c1f9750623e5aff))
*   Add the correct errors after sequencing has returned EmptyOk ([54fecc62](https://github.com/Marwes/combine/commit/54fecc62938445aae15373a6b1ec7c4419582025), closes [#95](https://github.com/Marwes/combine/issues/95))
*   Renamed SharedBufferedStream and BufferedStream to be less confusing ([3add407e](https://github.com/Marwes/combine/commit/3add407eecf886cc72ce05414d58a2b3b19a0bb9), breaks [#](https://github.com/Marwes/combine/issues/))
*   Add From<u8> for Info ([4cf8cff6](https://github.com/Marwes/combine/commit/4cf8cff64466519bf2d4a4dc1dcbe8deb449e004))
*   Make the positions of slice streams harder to misuse ([f50ab9e2](https://github.com/Marwes/combine/commit/f50ab9e2f42ec2465368bfb11a60b2339b699fc4), closes [#104](https://github.com/Marwes/combine/issues/104), breaks [#](https://github.com/Marwes/combine/issues/))

#### Breaking Changes

*   Remove depreceated items ([9107342a](https://github.com/Marwes/combine/commit/9107342a89a5efc664bac9c2919a93a992ca6809), breaks [#](https://github.com/Marwes/combine/issues/))
*   Renamed SharedBufferedStream and BufferedStream to be less confusing ([3add407e](https://github.com/Marwes/combine/commit/3add407eecf886cc72ce05414d58a2b3b19a0bb9), breaks [#](https://github.com/Marwes/combine/issues/))
*   Remove the old State type and Positioner trait ([ae43f8ae](https://github.com/Marwes/combine/commit/ae43f8ae2b303aca3b5ae9fbb1a87475349f2745), breaks [#](https://github.com/Marwes/combine/issues/))
*   Teach the choice parser to take tuples ([96da7ee0](https://github.com/Marwes/combine/commit/96da7ee0cf8a112e60747a0be8a4dbd90efbecba), breaks [#](https://github.com/Marwes/combine/issues/))
*   Add the range_of parser ([7e692086](https://github.com/Marwes/combine/commit/7e69208650f7fdc75279370b193030b09ccdbc7a), closes [#83](https://github.com/Marwes/combine/issues/83), breaks [#](https://github.com/Marwes/combine/issues/))
*   Make the positions of slice streams harder to misuse ([f50ab9e2](https://github.com/Marwes/combine/commit/f50ab9e2f42ec2465368bfb11a60b2339b699fc4), closes [#104](https://github.com/Marwes/combine/issues/104), breaks [#](https://github.com/Marwes/combine/issues/))
*   Allow ParseError to be used without the StreamOnce constraint ([520da8e8](https://github.com/Marwes/combine/commit/520da8e89f7162b4d6ba3a3bca05a05f3bd37999), breaks [#](https://github.com/Marwes/combine/issues/))



<a name="v2.5.0"></a>
## v2.5.0 (2017-08-07)

#### Features

*   Rename captures to captures_many and add a captures parser ([9d301e42](https://github.com/Marwes/combine/commit/9d301e42ee2da23c90ce78982d9dbef6d7586b4c))
*   Add regex parsers (match_, find_many) ([5ac12b98](https://github.com/Marwes/combine/commit/5ac12b9883c49b345341ad47aeac2c8accd52c33))
*   Add a macro to parse values directly into structs ([1656a620](https://github.com/Marwes/combine/commit/1656a620960e2b6256e724058cf39892d6e16944))
*   add count_min_max and skip_count_min_max ([8f3413a7](https://github.com/Marwes/combine/commit/8f3413a7431f4459d67695156f0b259df422bf09))
*   Add the skip_count parser ([15171d10](https://github.com/Marwes/combine/commit/15171d10495a5a221713ca0f67f3afc0b0eaf580))
*   Add the recognize parser ([61c9b269](https://github.com/Marwes/combine/commit/61c9b269826707e7fa7409512f21122c9fd8f137))
*   Add a macro for declaring parsers ([7fe1d9f7](https://github.com/Marwes/combine/commit/7fe1d9f723a14d20c9879849e104283ee24d254e), closes [#70](https://github.com/Marwes/combine/issues/70))
*   Provide parsers for decoding big-endian and little-endian numbers ([05ec0bc8](https://github.com/Marwes/combine/commit/05ec0bc8675a2de0a71268a458ceefa7ee99f7a0))

#### Bug Fixes

*   Report and_then errors as if at the start of the parse ([b71a78f1](https://github.com/Marwes/combine/commit/b71a78f12a40e90425d59f72d28c628d28aebe1d))
*   Return EmptyErr when the any parser fails ([93208e9c](https://github.com/Marwes/combine/commit/93208e9c6fd92628eb02c0b32a0d6d3120a9af7f), closes [#99](https://github.com/Marwes/combine/issues/99))
* **doc:**  regex find consumes input until the end of the first match ([d1bbf1d4](https://github.com/Marwes/combine/commit/d1bbf1d4198cb71d9c4b9e6d13399e38078518f0))



<a name="v2.3.0"></a>
## v2.3.0 (2017-02-22)


#### Performance

*   Don't call parse_stream in optional ([a4bf28d2](a4bf28d2))

#### Features

*   Add the choice! macro ([6f2cec69](6f2cec69))
*   Add map functions for Error<> and Info<> ranges. (#86)
*   Add Parser::boxed ([3af9c9b3](3af9c9b3))

<a name="2.1.0"></a>
##  2.1.0 (2016-10-30)


#### Features

*   Add a read adapter for the stream trait ([a2a9f214](a2a9f214))



<a name="2.0.0"></a>
## 2.0.0 (2016-10-19)


#### Features

*   Version 2.0.0 ([80b24186](https://github.com/Marwes/combine/commit/80b24186fb4854d3242f32abc727107545e08c7b))
*   Add the count parser ([a7949f3a](https://github.com/Marwes/combine/commit/a7949f3aef8585523e730e2c1224c3725b360d32))
*   Add the Parser::by_ref method ([15554d0c](https://github.com/Marwes/combine/commit/15554d0c64a2415e8c234708595cc544ada6c585))
*   Add the one_of and none_of parsers ([941b277c](https://github.com/Marwes/combine/commit/941b277c8f4d8e8af804c88678181be7743f912b))
*   Add the position parser ([d6c65f6d](https://github.com/Marwes/combine/commit/d6c65f6da5a2af47254abe2db4b04c3ecbd74803))
*   Add bytes_cmp and string_cmp ([ee6b430d](https://github.com/Marwes/combine/commit/ee6b430d17508daf305d5f48fabae2d662a94d34))
*   Add the `tokens` parser ([886c4523](https://github.com/Marwes/combine/commit/886c45235be207241874a0a412ebcc0733959466))
*   Version 2.0.0-beta3 ([55c59322](https://github.com/Marwes/combine/commit/55c59322f8ead037dad703a41e1f6d769c059f31))
*   Break out the error formatting into a separate function ([b6ccb0c1](https://github.com/Marwes/combine/commit/b6ccb0c1807f0f182878b68d4dbdcfa739fd5157))
*   Rename parse_state to parse_stream ([b375df48](https://github.com/Marwes/combine/commit/b375df4811570d14bbd8db7cb74a6834e54679cf))
*   Simplify the flat_map parser ([08a91ce2](https://github.com/Marwes/combine/commit/08a91ce201b67f5528a18228bdfb079e7d86dd7f))
*   Merge the ParserExt trait into Parser ([26a84154](https://github.com/Marwes/combine/commit/26a841540107b79542bb874a60abb83f99c78a58))
*   Add the bytes parser ([9c73c053](https://github.com/Marwes/combine/commit/9c73c053f37b149c35d60377f6dcbbbfc145dda9))
*   Add parsers specialized on byte streams ([01ba3759](https://github.com/Marwes/combine/commit/01ba375929daac2cb81a3e966e529f0909014620))
*   Make ctry usable outside the crate ([f45740dd](https://github.com/Marwes/combine/commit/f45740dd71cf9c71e0900e932c2f10ccbefae35e))
*   Add versions of parse_* which return an unpacked version of ParseResult ([2bbd14ab](https://github.com/Marwes/combine/commit/2bbd14abd2b372afbfda56fb73d4aa036bd427e1))
*   Add the satisy_map parser ([4d97d296](https://github.com/Marwes/combine/commit/4d97d2968c48026e8369e1f0bcee3c6ef5784664))
*   Replace the And parser with the pair parser ([b1f56113](https://github.com/Marwes/combine/commit/b1f561139169caa1a5a2e3e2d84248b28f22bb82))
*   Remove reexport of the char module from the root module ([e39dacb5](https://github.com/Marwes/combine/commit/e39dacb57999c3cfb0bb4ae6d5db0b696da60a3f))
*   Version 2.0.0-beta ([5bdbf584](https://github.com/Marwes/combine/commit/5bdbf58484800717c7d7c20b9161562520f425cb))
*   Remove the buffered_stream feature ([3fdbf217](https://github.com/Marwes/combine/commit/3fdbf217ec0a66b052b8d11792ce3ff3d13b7463))
*   Version 1.3.0 ([acea26cd](https://github.com/Marwes/combine/commit/acea26cda536ffc681ca4fa9e4c1bf28f5184582))
*   Add the eof parser ([6a89cbf2](https://github.com/Marwes/combine/commit/6a89cbf2ef11ed5bf4145a296c208e5f5f90438c))
*   Stabilize RangeStream and all functions using it ([d932375d](https://github.com/Marwes/combine/commit/d932375d13a196fc74602f8e76ad5bd3512ca370))
*   Reexport Stream and StreamOnce from the crate root ([2c2b3f5c](https://github.com/Marwes/combine/commit/2c2b3f5cd21a04fbc157a95ce76fe72bfdc1a2c3))
*   Merge the HasPosition trait into StreamOnce ([3bda4a16](https://github.com/Marwes/combine/commit/3bda4a163e8f3b57dd4efa65384c97f9c3554aeb))
*   Add the StreamOnce trait ([9ea0ed5d](https://github.com/Marwes/combine/commit/9ea0ed5d6c8f8cead773a24b968d4a0bbb606721), breaks [#](https://github.com/Marwes/combine/issues/))
*   Make Stream::uncons take &mut self ([4ddc4257](https://github.com/Marwes/combine/commit/4ddc4257d1e719a9f1c17a49c39f08ebf20d2999))
*   Separate the Position type and position method from Stream ([9cfb9a89](https://github.com/Marwes/combine/commit/9cfb9a895be34b288ee9fc9f926cd1b9c5b97b03))
*   Version 1.2.1 ([f737af27](https://github.com/Marwes/combine/commit/f737af27306160088188900a1cdad255b5ca58d3))
*   Move the position handling inside the Stream trait ([f41f65e9](https://github.com/Marwes/combine/commit/f41f65e9f34b64481f81af078ecdb10a80e75f6f))
* **range_stream:**  Implement RangeStream on State ([f5679dc9](https://github.com/Marwes/combine/commit/f5679dc954be093a7a0278d2311cf5a162396833))

#### Performance

*   Specialize and_then, then and flat_map ([9dc7dc6b](https://github.com/Marwes/combine/commit/9dc7dc6b9bcb638888be448efb7002d362aded16))
*   Specialize the tuple parser to avoid unnecessary branches ([2b294f80](https://github.com/Marwes/combine/commit/2b294f8009021897d9652981dfb107dd2102a902))
*   Add inline annotations and more forwarding parse functions ([0e5ee38e](https://github.com/Marwes/combine/commit/0e5ee38e1b15847908f6676c0c4032dc844e3462))
*   Avoid indirection in Skip and With ([52d335ca](https://github.com/Marwes/combine/commit/52d335caa2e698de9be50e46e8fbcf241d4e3081))
*   Optimize Iter by simplifying the state machine ([9631700a](https://github.com/Marwes/combine/commit/9631700a306cb5546e37dfb8f05d54728fb3bc8c))
*   Speedup tuple parsers by simplifying the expanded code ([5d86dcf2](https://github.com/Marwes/combine/commit/5d86dcf2d14f1cae078d1a4b8831d37041eaf7a2))
*   Avoid creating an error when take_while1 parses no input ([9bad15c0](https://github.com/Marwes/combine/commit/9bad15c0f79e3ff897fb92cdca6b92f988c69347))
*   Possibly improve performance of the RangeStream impl for &str ([abb1de7f](https://github.com/Marwes/combine/commit/abb1de7f15b65b9bc2c40572319269191bd0819f))

#### Bug Fixes

*   Rename the String parser to Str ([d846bf0e](https://github.com/Marwes/combine/commit/d846bf0e7ddb3350ce9245b3682d7c054ff5cdd8))
*   Use five copies in the large http test anyway to match nom_benchmarks ([eb089f5b](https://github.com/Marwes/combine/commit/eb089f5bef175b96e097286b9c8c3e7d5f6e3922))
*   Avoid storing the position in primitives::uncons_while ([9912507a](https://github.com/Marwes/combine/commit/9912507a80e178737e16d4ff3d19d7a1fee9fbc8))
*   Calling uncons_range with the same size as is remaining should succeed ([cce6214e](https://github.com/Marwes/combine/commit/cce6214ed4722880881c8c6998e00f4509a22588))
*   Add Sync to to the Error::Other variant ([22add3ec](https://github.com/Marwes/combine/commit/22add3eca62ff5e6f4d58122a4b366290b1d9385))
*   Fix positions of BufferedStream being for the next token ([66eab92a](https://github.com/Marwes/combine/commit/66eab92a7dd63269f48cf0fbd0722a6eeea9135d))
*   Fix the position handling of BufferedStream ([f21148b3](https://github.com/Marwes/combine/commit/f21148b3c4c5c6f10d8b6d90ce4a7925596879b3))
*   Remove the Positioner bound from Stream::Token an Stream::Range ([fba3f1e7](https://github.com/Marwes/combine/commit/fba3f1e760505305b6a586b6ff5a53eff645e1d1))
* **buffered_stream:**  BufferedStream no longer emits the last token after EOF ([6532884c](https://github.com/Marwes/combine/commit/6532884cc16307e1753584dd40b2b59e3daa6267))
* **travis:**
  *  Dont pass the test feature to travis ([382a608d](https://github.com/Marwes/combine/commit/382a608da2851c5cc2d3477025951e9a133732bc))
  *  Add travis_wait so travis does not time out the beta builds ([a3f0792a](https://github.com/Marwes/combine/commit/a3f0792ab347805e3f0ce619997a2c154f5e8c87))

#### Breaking Changes

*   Add the StreamOnce trait ([9ea0ed5d](https://github.com/Marwes/combine/commit/9ea0ed5d6c8f8cead773a24b968d4a0bbb606721), breaks [#](https://github.com/Marwes/combine/issues/))
