//! Bindings to [libFuzzer](http://llvm.org/docs/LibFuzzer.html): a runtime for
//! coverage-guided fuzzing.
//!
//! See [the `cargo-fuzz`
//! guide](https://rust-fuzz.github.io/book/cargo-fuzz.html) for a usage
//! tutorial.
//!
//! The main export of this crate is [the `fuzz_target!`
//! macro](./macro.fuzz_target.html), which allows you to define targets for
//! libFuzzer to exercise.

#![deny(missing_docs, missing_debug_implementations)]

pub use arbitrary;
use std::sync::OnceLock;

/// Indicates whether the input should be kept in the corpus or rejected. This
/// should be returned by your fuzz target. If your fuzz target does not return
/// a value (i.e., returns `()`), then the input will be kept in the corpus.
#[derive(Debug)]
pub enum Corpus {
    /// Keep the input in the corpus.
    Keep,

    /// Reject the input and do not keep it in the corpus.
    Reject,
}

impl From<()> for Corpus {
    fn from(_: ()) -> Self {
        Self::Keep
    }
}

impl Corpus {
    #[doc(hidden)]
    /// Convert this Corpus result into the [integer codes used by
    /// `libFuzzer`](https://llvm.org/docs/LibFuzzer.html#rejecting-unwanted-inputs).
    /// This is -1 for reject, 0 for keep.
    pub fn to_libfuzzer_code(self) -> i32 {
        match self {
            Corpus::Keep => 0,
            Corpus::Reject => -1,
        }
    }
}

extern "C" {
    // We do not actually cross the FFI bound here.
    #[allow(improper_ctypes)]
    fn rust_fuzzer_test_input(input: &[u8]) -> i32;

    fn LLVMFuzzerMutate(data: *mut u8, size: usize, max_size: usize) -> usize;
}

/// Do not use; only for LibFuzzer's consumption.
#[doc(hidden)]
#[export_name = "LLVMFuzzerTestOneInput"]
pub unsafe fn test_input_wrap(data: *const u8, size: usize) -> i32 {
    let test_input = ::std::panic::catch_unwind(|| {
        let data_slice = ::std::slice::from_raw_parts(data, size);
        rust_fuzzer_test_input(data_slice)
    });

    match test_input {
        Ok(i) => i,
        Err(_) => {
            // hopefully the custom panic hook will be called before and abort the
            // process before the stack frames are unwinded.
            ::std::process::abort();
        }
    }
}

#[doc(hidden)]
pub fn rust_libfuzzer_debug_path() -> &'static Option<String> {
    static RUST_LIBFUZZER_DEBUG_PATH: OnceLock<Option<String>> = OnceLock::new();
    RUST_LIBFUZZER_DEBUG_PATH.get_or_init(|| std::env::var("RUST_LIBFUZZER_DEBUG_PATH").ok())
}

#[doc(hidden)]
pub fn initialize(_argc: *const isize, _argv: *const *const *const u8) -> isize {
    // Registers a panic hook that aborts the process before unwinding.
    // It is useful to abort before unwinding so that the fuzzer will then be
    // able to analyse the process stack frames to tell different bugs appart.
    //
    // HACK / FIXME: it would be better to use `-C panic=abort` but it's currently
    // impossible to build code using compiler plugins with this flag.
    // We will be able to remove this code when
    // https://github.com/rust-lang/cargo/issues/5423 is fixed.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        default_hook(panic_info);
        std::process::abort();
    }));
    0
}

/// Define a fuzz target.
///
/// ## Example
///
/// This example takes a `&[u8]` slice and attempts to parse it. The parsing
/// might fail and return an `Err`, but it shouldn't ever panic or segfault.
///
/// ```no_run
/// #![no_main]
///
/// use libfuzzer_sys::fuzz_target;
///
/// // Note: `|input|` is short for `|input: &[u8]|`.
/// fuzz_target!(|input| {
///     let _result: Result<_, _> = my_crate::parse(input);
/// });
/// # mod my_crate { pub fn parse(_: &[u8]) -> Result<(), ()> { unimplemented!() } }
/// ```
///
/// ## Rejecting Inputs
///
/// It may be desirable to reject some inputs, i.e. to not add them to the
/// corpus.
///
/// For example, when fuzzing an API consisting of parsing and other logic,
/// one may want to allow only those inputs into the corpus that parse
/// successfully. To indicate whether an input should be kept in or rejected
/// from the corpus, return either [Corpus::Keep] or [Corpus::Reject] from your
/// fuzz target. The default behavior (e.g. if `()` is returned) is to keep the
/// input in the corpus.
///
/// For example:
///
/// ```no_run
/// #![no_main]
///
/// use libfuzzer_sys::{Corpus, fuzz_target};
///
/// fuzz_target!(|input: String| -> Corpus {
///     let parts: Vec<&str> = input.splitn(2, '=').collect();
///     if parts.len() != 2 {
///         return Corpus::Reject;
///     }
///
///     let key = parts[0];
///     let value = parts[1];
///     let _result: Result<_, _> = my_crate::parse(key, value);
///     Corpus::Keep
/// });
/// # mod my_crate { pub fn parse(_key: &str, _value: &str) -> Result<(), ()> { unimplemented!() } }
/// ```
///
/// ## Arbitrary Input Types
///
/// The input is a `&[u8]` slice by default, but you can take arbitrary input
/// types, as long as the type implements [the `arbitrary` crate's `Arbitrary`
/// trait](https://docs.rs/arbitrary/*/arbitrary/trait.Arbitrary.html) (which is
/// also re-exported as `libfuzzer_sys::arbitrary::Arbitrary` for convenience).
///
/// For example, if you wanted to take an arbitrary RGB color, you could do the
/// following:
///
/// ```no_run
/// #![no_main]
/// # mod foo {
///
/// use libfuzzer_sys::{arbitrary::{Arbitrary, Error, Unstructured}, fuzz_target};
///
/// #[derive(Debug)]
/// pub struct Rgb {
///     r: u8,
///     g: u8,
///     b: u8,
/// }
///
/// impl<'a> Arbitrary<'a> for Rgb {
///     fn arbitrary(raw: &mut Unstructured<'a>) -> Result<Self, Error> {
///         let mut buf = [0; 3];
///         raw.fill_buffer(&mut buf)?;
///         let r = buf[0];
///         let g = buf[1];
///         let b = buf[2];
///         Ok(Rgb { r, g, b })
///     }
/// }
///
/// // Write a fuzz target that works with RGB colors instead of raw bytes.
/// fuzz_target!(|color: Rgb| {
///     my_crate::convert_color(color);
/// });
/// # mod my_crate {
/// #     use super::Rgb;
/// #     pub fn convert_color(_: Rgb) {}
/// # }
/// # }
/// ```
///
/// You can also enable the `arbitrary` crate's custom derive via this crate's
/// `"arbitrary-derive"` cargo feature.
///
/// ## Init Code
///
/// Init code to the fuzz target by using the `init` keyword. This is called once before the fuzzer starts.
/// Supports short |input| or |input: <type>| syntax.
///
/// ```no_run
/// #![no_main]
///
/// use libfuzzer_sys::fuzz_target;
/// use std::collections::HashSet;
/// use std::sync::OnceLock;
///
/// static DICTIONARY: OnceLock<HashSet<String>> = OnceLock::new();
///
/// fuzz_target!(
///     init: {
///         let read_dictionary = |_| unimplemented!();
///         let dictionary = read_dictionary("/usr/share/dict/words");
///         DICTIONARY.set(dictionary).unwrap();
///     },
///     |input| {
///         // Use the initialized `DICTIONARY` here...
///     }
/// );
/// ```
///
#[macro_export]
macro_rules! fuzz_target {
    (|$bytes:ident| $body:expr) => {
        $crate::fuzz_target!(init: (), |$bytes: &[u8]| -> () { $body });
    };

    (|$bytes:ident: &[u8]| $body:expr) => {
        $crate::fuzz_target!(init: (), |$bytes: &[u8]| -> () { $body });
    };

    (|$bytes:ident: &[u8]| -> $rty:ty $body:block) => {
        $crate::fuzz_target!(init: (), |$bytes: &[u8]| -> $rty { $body });
    };

    (init: $init:expr, |$bytes:ident| $body:expr) => {
        $crate::fuzz_target!(init: $init, |$bytes: &[u8]| -> () { $body });
    };

    (init: $init:expr, |$bytes:ident: &[u8]| $body:expr) => {
        $crate::fuzz_target!(init: $init, |$bytes: &[u8]| -> () { $body });
    };

    (init: $init:expr, |$bytes:ident: &[u8]| -> $rty:ty $body:block) => {
        const _: () = {
            /// Auto-generated functions
            /// LLVMFuzzerInitialize is called once before the fuzzer starts.
            #[no_mangle]
            pub extern "C" fn LLVMFuzzerInitialize(_argc: *const isize, _argv: *const *const *const u8) -> isize {
                $crate::initialize(_argc, _argv);

                // Supplied init code
                $init;
                0
            }

            #[no_mangle]
            pub extern "C" fn rust_fuzzer_test_input(bytes: &[u8]) -> i32 {
                // When `RUST_LIBFUZZER_DEBUG_PATH` is set, write the debug
                // formatting of the input to that file. This is only intended for
                // `cargo fuzz`'s use!

                // `RUST_LIBFUZZER_DEBUG_PATH` is set in initialization.
                if let Some(path) = $crate::rust_libfuzzer_debug_path() {
                    use std::io::Write;
                    let mut file = std::fs::File::create(path)
                        .expect("failed to create `RUST_LIBFUZZER_DEBUG_PATH` file");
                    writeln!(&mut file, "{:?}", bytes)
                        .expect("failed to write to `RUST_LIBFUZZER_DEBUG_PATH` file");
                    return 0;
                }

                let result = ::libfuzzer_sys::Corpus::from(__libfuzzer_sys_run(bytes));
                result.to_libfuzzer_code()
            }

            // Split out the actual fuzzer into a separate function which is
            // tagged as never being inlined. This ensures that if the fuzzer
            // panics there's at least one stack frame which is named uniquely
            // according to this specific fuzzer that this is embedded within.
            //
            // Systems like oss-fuzz try to deduplicate crashes and without this
            // panics in separate fuzzers can accidentally appear the same
            // because each fuzzer will have a function called
            // `rust_fuzzer_test_input`. By using a normal Rust function here
            // it's named something like `the_fuzzer_name::_::__libfuzzer_sys_run` which should
            // ideally help prevent oss-fuzz from deduplicate fuzz bugs across
            // distinct targets accidentally.
            #[inline(never)]
            fn __libfuzzer_sys_run($bytes: &[u8]) -> $rty {
                $body
            }
        };
    };

    (|$data:ident: $dty:ty| $body:expr) => {
        $crate::fuzz_target!(init: (), |$data: $dty| -> () { $body });
    };

    (|$data:ident: $dty:ty| -> $rty:ty $body:block) => {
        $crate::fuzz_target!(init: (), |$data: $dty| -> $rty { $body });
    };

    (init: $init:expr, |$data:ident: $dty:ty| $body:expr) => {
        $crate::fuzz_target!(init: $init, |$data: $dty| -> () { $body });
    };

    (init: $init:expr, |$data:ident: $dty:ty| -> $rty:ty $body:block) => {
        const _: () = {
            /// Auto-generated functions
            /// LLVMFuzzerInitialize is called once before the fuzzer starts.
            #[no_mangle]
            pub extern "C" fn LLVMFuzzerInitialize(_argc: *const isize, _argv: *const *const *const u8) -> isize {
                $crate::initialize(_argc, _argv);

                // Supplied init code
                $init;
                0
            }

            #[no_mangle]
            pub extern "C" fn rust_fuzzer_test_input(bytes: &[u8]) -> i32 {
                use $crate::arbitrary::{Arbitrary, Unstructured};

                // Early exit if we don't have enough bytes for the `Arbitrary`
                // implementation. This helps the fuzzer avoid exploring all the
                // different not-enough-input-bytes paths inside the `Arbitrary`
                // implementation. Additionally, it exits faster, letting the fuzzer
                // get to longer inputs that actually lead to interesting executions
                // quicker.
                if bytes.len() < <$dty as Arbitrary>::size_hint(0).0 {
                    return -1;
                }

                let mut u = Unstructured::new(bytes);
                let data = <$dty as Arbitrary>::arbitrary_take_rest(u);

                // When `RUST_LIBFUZZER_DEBUG_PATH` is set, write the debug
                // formatting of the input to that file. This is only intended for
                // `cargo fuzz`'s use!

                // `RUST_LIBFUZZER_DEBUG_PATH` is set in initialization.
                if let Some(path) = $crate::rust_libfuzzer_debug_path() {
                    use std::io::Write;
                    let mut file = std::fs::File::create(path)
                        .expect("failed to create `RUST_LIBFUZZER_DEBUG_PATH` file");
                    (match data {
                        Ok(data) => writeln!(&mut file, "{:#?}", data),
                        Err(err) => writeln!(&mut file, "Arbitrary Error: {}", err),
                    })
                    .expect("failed to write to `RUST_LIBFUZZER_DEBUG_PATH` file");
                    return -1;
                }

                let data = match data {
                    Ok(d) => d,
                    Err(_) => return -1,
                };

                let result = ::libfuzzer_sys::Corpus::from(__libfuzzer_sys_run(data));
                result.to_libfuzzer_code()
            }
            // See above for why this is split to a separate function.
            #[inline(never)]
            fn __libfuzzer_sys_run($data: $dty) -> $rty {
                $body
            }
        };
    };
}

/// Define a custom mutator.
///
/// This is optional, and libFuzzer will use its own, default mutation strategy
/// if this is not provided.
///
/// You might consider using a custom mutator when your fuzz target is very
/// particular about the shape of its input:
///
/// * You want to fuzz "deeper" than just the parser.
/// * The input contains checksums that have to match the hash of some subset of
///   the data or else the whole thing is invalid, and therefore mutating any of
///   that subset means you need to recompute the checksums.
/// * Small random changes to the input buffer make it invalid.
///
/// That is, a custom mutator is useful in similar situations where [a `T:
/// Arbitrary` input type](macro.fuzz_target.html#arbitrary-input-types) is
/// useful. Note that the two approaches are not mutually exclusive; you can use
/// whichever is easier for your problem domain or both!
///
/// ## Implementation Contract
///
/// The original, unmodified input is given in `data[..size]`.
///
/// You must modify the data in place and return the new size.
///
/// The new size should not be greater than `max_size`. If this is not the case,
/// then the `data` will be truncated to fit within `max_size`. Note that
/// `max_size < size` is possible when shrinking test cases.
///
/// You must produce the same mutation given the same `seed`. Generally, when
/// choosing what kind of mutation to make or where to mutate, you should start
/// by creating a random number generator (RNG) that is seeded with the given
/// `seed` and then consult the RNG whenever making a decision:
///
/// ```no_run
/// #![no_main]
///
/// use rand::{rngs::StdRng, Rng, SeedableRng};
///
/// libfuzzer_sys::fuzz_mutator!(|data: &mut [u8], size: usize, max_size: usize, seed: u32| {
///     let mut rng = StdRng::seed_from_u64(seed as u64);
///
/// #   let first_mutation = |_, _, _, _| todo!();
/// #   let second_mutation = |_, _, _, _| todo!();
/// #   let third_mutation = |_, _, _, _| todo!();
/// #   let fourth_mutation = |_, _, _, _| todo!();
///     // Choose which of our four supported kinds of mutations we want to make.
///     match rng.gen_range(0..4) {
///         0 => first_mutation(rng, data, size, max_size),
///         1 => second_mutation(rng, data, size, max_size),
///         2 => third_mutation(rng, data, size, max_size),
///         3 => fourth_mutation(rng, data, size, max_size),
///         _ => unreachable!()
///     }
/// });
/// ```
///
/// ## Example: Compression
///
/// Consider a simple fuzz target that takes compressed data as input,
/// decompresses it, and then asserts that the decompressed data doesn't begin
/// with "boom". It is difficult for `libFuzzer` (or any other fuzzer) to crash
/// this fuzz target because nearly all mutations it makes will invalidate the
/// compression format. Therefore, we use a custom mutator that decompresses the
/// raw input, mutates the decompressed data, and then recompresses it. This
/// allows `libFuzzer` to quickly discover crashing inputs.
///
/// ```no_run
/// #![no_main]
///
/// use flate2::{read::GzDecoder, write::GzEncoder, Compression};
/// use libfuzzer_sys::{fuzz_mutator, fuzz_target};
/// use std::io::{Read, Write};
///
/// fuzz_target!(|data: &[u8]| {
///     // Decompress the input data and crash if it starts with "boom".
///     if let Some(data) = decompress(data) {
///         if data.starts_with(b"boom") {
///             panic!();
///         }
///     }
/// });
///
/// fuzz_mutator!(
///     |data: &mut [u8], size: usize, max_size: usize, _seed: u32| {
///         // Decompress the input data. If that fails, use a dummy value.
///         let mut decompressed = decompress(&data[..size]).unwrap_or_else(|| b"hi".to_vec());
///
///         // Mutate the decompressed data with `libFuzzer`'s default mutator. Make
///         // the `decompressed` vec's extra capacity available for insertion
///         // mutations via `resize`.
///         let len = decompressed.len();
///         let cap = decompressed.capacity();
///         decompressed.resize(cap, 0);
///         let new_decompressed_size = libfuzzer_sys::fuzzer_mutate(&mut decompressed, len, cap);
///
///         // Recompress the mutated data.
///         let compressed = compress(&decompressed[..new_decompressed_size]);
///
///         // Copy the recompressed mutated data into `data` and return the new size.
///         let new_size = std::cmp::min(max_size, compressed.len());
///         data[..new_size].copy_from_slice(&compressed[..new_size]);
///         new_size
///     }
/// );
///
/// fn decompress(compressed_data: &[u8]) -> Option<Vec<u8>> {
///     let mut decoder = GzDecoder::new(compressed_data);
///     let mut decompressed = Vec::new();
///     if decoder.read_to_end(&mut decompressed).is_ok() {
///         Some(decompressed)
///     } else {
///         None
///     }
/// }
///
/// fn compress(data: &[u8]) -> Vec<u8> {
///     let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
///     encoder
///         .write_all(data)
///         .expect("writing into a vec is infallible");
///     encoder.finish().expect("writing into a vec is infallible")
/// }
/// ```
///
/// This example is inspired by [a similar example from the official `libFuzzer`
/// docs](https://github.com/google/fuzzing/blob/master/docs/structure-aware-fuzzing.md#example-compression).
///
/// ## More Example Ideas
///
/// * A PNG custom mutator that decodes a PNG, mutates the image, and then
/// re-encodes the mutated image as a new PNG.
///
/// * A [`serde`](https://serde.rs/) custom mutator that deserializes your
///   structure, mutates it, and then reserializes it.
///
/// * A Wasm binary custom mutator that inserts, replaces, and removes a
///   bytecode instruction in a function's body.
///
/// * An HTTP request custom mutator that inserts, replaces, and removes a
///   header from an HTTP request.
#[macro_export]
macro_rules! fuzz_mutator {
    (
        |
        $data:ident : &mut [u8] ,
        $size:ident : usize ,
        $max_size:ident : usize ,
        $seed:ident : u32 $(,)*
        |
        $body:block
    ) => {
        /// Auto-generated function. Do not use; only for LibFuzzer's
        /// consumption.
        #[export_name = "LLVMFuzzerCustomMutator"]
        #[doc(hidden)]
        pub unsafe fn rust_fuzzer_custom_mutator(
            $data: *mut u8,
            $size: usize,
            $max_size: usize,
            $seed: std::os::raw::c_uint,
        ) -> usize {
            // Depending on if we are growing or shrinking the test case, `size`
            // might be larger or smaller than `max_size`. The `data`'s capacity
            // is the maximum of the two.
            let len = std::cmp::max($max_size, $size);
            let $data: &mut [u8] = std::slice::from_raw_parts_mut($data, len);

            // `unsigned int` is generally a `u32`, but not on all targets. Do
            // an infallible (and potentially lossy, but that's okay because it
            // preserves determinism) conversion.
            let $seed = $seed as u32;

            // Define and invoke a new, safe function so that the body doesn't
            // inherit `unsafe`.
            fn custom_mutator(
                $data: &mut [u8],
                $size: usize,
                $max_size: usize,
                $seed: u32,
            ) -> usize {
                $body
            }
            let new_size = custom_mutator($data, $size, $max_size, $seed);

            // Truncate the new size if it is larger than the max.
            std::cmp::min(new_size, $max_size)
        }
    };
}

/// The default `libFuzzer` mutator.
///
/// You generally don't have to use this at all unless you're defining a
/// custom mutator with [the `fuzz_mutator!` macro][crate::fuzz_mutator].
///
/// Mutates `data[..size]` in place such that the mutated data is no larger than
/// `max_size` and returns the new size of the mutated data.
///
/// To only allow shrinking mutations, make `max_size < size`.
///
/// To additionally allow mutations that grow the size of the data, make
/// `max_size > size`.
///
/// Both `size` and `max_size` must be less than or equal to `data.len()`.
///
/// # Example
///
/// ```no_run
/// // Create some data in a buffer.
/// let mut data = vec![0; 128];
/// data[..b"hello".len()].copy_from_slice(b"hello");
///
/// // Ask `libFuzzer` to mutate the data. By setting `max_size` to our buffer's
/// // full length, we are allowing `libFuzzer` to perform mutations that grow
/// // the size of the data, such as insertions.
/// let size = b"hello".len();
/// let max_size = data.len();
/// let new_size = libfuzzer_sys::fuzzer_mutate(&mut data, size, max_size);
///
/// // Get the mutated data out of the buffer.
/// let mutated_data = &data[..new_size];
/// ```
pub fn fuzzer_mutate(data: &mut [u8], size: usize, max_size: usize) -> usize {
    assert!(size <= data.len());
    assert!(max_size <= data.len());
    let new_size = unsafe { LLVMFuzzerMutate(data.as_mut_ptr(), size, max_size) };
    assert!(new_size <= data.len());
    new_size
}

/// Define a custom cross-over function to combine test cases.
///
/// This is optional, and libFuzzer will use its own, default cross-over strategy
/// if this is not provided. (As of the time of writing, this default strategy
/// takes alternating byte sequences from the two test cases, to construct the
/// new one) (see `FuzzerCrossOver.cpp`)
///
/// This could potentially be useful if your input is, for instance, a
/// sequence of fixed sized, multi-byte values and the crossover could then
/// merge discrete values rather than joining parts of a value.
///
/// ## Implementation Contract
///
/// The original, read-only inputs are given in the full slices of `data1`, and
/// `data2` (as opposed to the, potentially, partial slice of `data` in
/// [the `fuzz_mutator!` macro][crate::fuzz_mutator]).
///
/// You must place the new input merged from the two existing inputs' data
/// into `out` and return the size of the relevant data written to that slice.
///
/// The deterministic requirements from [the `fuzz_mutator!` macro][crate::fuzz_mutator]
/// apply as well to the `seed` parameter
///
/// ## Example: Floating-Point Sum NaN
///
/// ```no_run
/// #![no_main]
///
/// use libfuzzer_sys::{fuzz_crossover, fuzz_mutator, fuzz_target, fuzzer_mutate};
/// use rand::{rngs::StdRng, Rng, SeedableRng};
/// use std::mem::size_of;
///
/// fuzz_target!(|data: &[u8]| {
///     let (_, floats, _) = unsafe { data.align_to::<f64>() };
///
///     let res = floats
///         .iter()
///         .fold(0.0, |a, b| if b.is_nan() { a } else { a + b });
///
///     assert!(
///         !res.is_nan(),
///         "The sum of the following floats resulted in a NaN: {floats:?}"
///     );
/// });
///
/// // Inject some ...potentially problematic values to make the example close
/// // more quickly.
/// fuzz_mutator!(|data: &mut [u8], size: usize, max_size: usize, seed: u32| {
///     let mut gen = StdRng::seed_from_u64(seed.into());
///
///     let (_, floats, _) = unsafe { data[..size].align_to_mut::<f64>() };
///
///     let x = gen.gen_range(0..=1000);
///     if x == 0 && !floats.is_empty() {
///         floats[0] = f64::INFINITY;
///     } else if x == 1000 && floats.len() > 1 {
///         floats[1] = f64::NEG_INFINITY;
///     } else {
///         return fuzzer_mutate(data, size, max_size);
///     }
///
///     size
/// });
///
/// fuzz_crossover!(|data1: &[u8], data2: &[u8], out: &mut [u8], _seed: u32| {
///     // Decode each source to see how many floats we can pull with proper
///     // alignment, and destination as to how many will fit with proper alignment
///     //
///     // Keep track of the unaligned prefix to `out`, as we will need to remember
///     // that those bytes will remain prepended to the actual floats that we
///     // write into the out buffer.
///     let (out_pref, out_floats, _) = unsafe { out.align_to_mut::<f64>() };
///     let (_, d1_floats, _) = unsafe { data1.align_to::<f64>() };
///     let (_, d2_floats, _) = unsafe { data2.align_to::<f64>() };
///
///     // Put into the destination, floats first from data1 then from data2, ...if
///     // possible given the size of `out`
///     let mut i: usize = 0;
///     for float in d1_floats.iter().chain(d2_floats).take(out_floats.len()) {
///         out_floats[i] = *float;
///         i += 1;
///     }
///
///     // Now that we have written the true floats, report back to the fuzzing
///     // engine that we left the unaligned `out` prefix bytes at the beginning of
///     // `out` and also then the floats that we wrote into the aligned float
///     // section.
///     out_pref.len() * size_of::<u8>() + i * size_of::<f64>()
/// });
/// ```
///
/// This example is a minimized version of [Erik Rigtorp's floating point
/// summation fuzzing example][1]. A more detailed version of this experiment
/// can be found in the `example_crossover` directory.
///
/// [1]: https://rigtorp.se/fuzzing-floating-point-code/
#[macro_export]
macro_rules! fuzz_crossover {
    (
        |
        $data1:ident : &[u8] ,
        $data2:ident : &[u8] ,
        $out:ident : &mut [u8] ,
        $seed:ident : u32 $(,)*
        |
        $body:block
    ) => {
        /// Auto-generated function. Do not use; only for LibFuzzer's
        /// consumption.
        #[export_name = "LLVMFuzzerCustomCrossOver"]
        #[doc(hidden)]
        pub unsafe fn rust_fuzzer_custom_crossover(
            $data1: *const u8,
            size1: usize,
            $data2: *const u8,
            size2: usize,
            $out: *mut u8,
            max_out_size: usize,
            $seed: std::os::raw::c_uint,
        ) -> usize {
            let $data1: &[u8] = std::slice::from_raw_parts($data1, size1);
            let $data2: &[u8] = std::slice::from_raw_parts($data2, size2);
            let $out: &mut [u8] = std::slice::from_raw_parts_mut($out, max_out_size);

            // `unsigned int` is generally a `u32`, but not on all targets. Do
            // an infallible (and potentially lossy, but that's okay because it
            // preserves determinism) conversion.
            let $seed = $seed as u32;

            // Define and invoke a new, safe function so that the body doesn't
            // inherit `unsafe`.
            fn custom_crossover(
                $data1: &[u8],
                $data2: &[u8],
                $out: &mut [u8],
                $seed: u32,
            ) -> usize {
                $body
            }

            custom_crossover($data1, $data2, $out, $seed)
        }
    };
}
