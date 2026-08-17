//! Some common traits used by minidump-related crates.

use debugid::{CodeId, DebugId};
use range_map::{Range, RangeMap};

use std::borrow::Cow;
use std::cmp;
use std::fmt::Debug;

/// An executable or shared library loaded in a process.
pub trait Module {
    /// The base address of this code module as it was loaded by the process.
    fn base_address(&self) -> u64;
    /// The size of the code module.
    fn size(&self) -> u64;
    /// The path or file name that the code module was loaded from.
    fn code_file(&self) -> Cow<'_, str>;
    /// An identifying string used to discriminate between multiple versions and
    /// builds of the same code module.  This may contain a uuid, timestamp,
    /// version number, or any combination of this or other information, in an
    /// implementation-defined format.
    fn code_identifier(&self) -> Option<CodeId>;
    /// The filename containing debugging information associated with the code
    /// module.  If debugging information is stored in a file separate from the
    /// code module itself (as is the case when .pdb or .dSYM files are used),
    /// this will be different from code_file.  If debugging information is
    /// stored in the code module itself (possibly prior to stripping), this
    /// will be the same as code_file.
    fn debug_file(&self) -> Option<Cow<'_, str>>;
    /// An identifying string similar to code_identifier, but identifies a
    /// specific version and build of the associated debug file.  This may be
    /// the same as code_identifier when the debug_file and code_file are
    /// identical or when the same identifier is used to identify distinct
    /// debug and code files.
    fn debug_identifier(&self) -> Option<DebugId>;
    /// A human-readable representation of the code module's version.
    fn version(&self) -> Option<Cow<'_, str>>;
}

/// Implement Module for 2-tuples of (&str, DebugId) for convenience.
/// `breakpad-symbols`' `Symbolizer::get_symbol_at_address` uses this.
impl Module for (&str, DebugId) {
    fn base_address(&self) -> u64 {
        0
    }
    fn size(&self) -> u64 {
        0
    }
    fn code_file(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }
    fn code_identifier(&self) -> Option<CodeId> {
        None
    }
    fn debug_file(&self) -> Option<Cow<'_, str>> {
        let &(file, _id) = self;
        Some(Cow::Borrowed(file))
    }
    fn debug_identifier(&self) -> Option<DebugId> {
        let &(_file, id) = self;
        Some(id)
    }
    fn version(&self) -> Option<Cow<'_, str>> {
        None
    }
}

/// This trait exists to allow creating `RangeMap`s from possibly-overlapping input data.
///
/// The `RangeMap` struct will panic if you attempt to initialize it with overlapping data,
/// and we deal with many sources of untrusted input data that could run afoul of this.
/// [Upstream issue](https://github.com/jneem/range-map/issues/1)
pub trait IntoRangeMapSafe<V>: IntoIterator<Item = (Option<Range<u64>>, V)> + Sized
where
    V: Clone + Debug + Eq,
{
    fn into_rangemap_safe(self) -> RangeMap<u64, V> {
        let mut input: Vec<_> = self.into_iter().collect();
        input.sort_by_key(|x| x.0);
        let mut vec: Vec<(Range<u64>, V)> = Vec::with_capacity(input.len());
        for (range, val) in input.into_iter() {
            if range.is_none() {
                // warn!("Unable to create valid range for {:?}", val);
                continue;
            }
            let range = range.unwrap();

            if let Some(&mut (ref mut last_range, ref last_val)) = vec.last_mut() {
                if range.start <= last_range.end && &val != last_val {
                    // This logging is nice to have but some symbol files are absolutely
                    // horribly polluted with duplicate entries with different values(!!!)
                    // and this generates literally a gigabyte of logs, yikes!

                    /*
                    warn!("overlapping ranges {:?} and {:?}", last_range, range);
                    warn!(" value1: {:?}", last_val);
                    warn!(" value2: {:?}\n", val);
                    */
                    continue;
                }

                if range.start <= last_range.end.saturating_add(1) && &val == last_val {
                    last_range.end = cmp::max(range.end, last_range.end);
                    continue;
                }
            }

            vec.push((range, val));
        }
        RangeMap::try_from_iter(vec).unwrap()
    }
}

impl<I, V> IntoRangeMapSafe<V> for I
where
    I: IntoIterator<Item = (Option<Range<u64>>, V)> + Sized,
    V: Clone + Debug + Eq,
{
}
