//! A [Glyph Variations Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/gvar) implementation.

// https://docs.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#tuple-variation-store

// We do have to call clone for readability on some types.
#![allow(clippy::clone_on_copy)]
#![allow(clippy::neg_cmp_op_on_partial_ord)]

use core::cmp;
use core::convert::TryFrom;
use core::num::NonZeroU16;

use crate::parser::{LazyArray16, Offset, Offset16, Offset32, Stream, F2DOT14};
use crate::{glyf, PhantomPoints, PointF};
use crate::{GlyphId, NormalizedCoordinate, OutlineBuilder, Rect, RectF, Transform};

/// 'The TrueType rasterizer dynamically generates 'phantom' points for each glyph
/// that represent horizontal and vertical advance widths and side bearings,
/// and the variation data within the `gvar` table includes data for these phantom points.'
///
/// We don't actually use them, but they are required during deltas parsing.
const PHANTOM_POINTS_LEN: usize = 4;

#[derive(Clone, Copy)]
enum GlyphVariationDataOffsets<'a> {
    Short(LazyArray16<'a, Offset16>),
    Long(LazyArray16<'a, Offset32>),
}

#[derive(Clone, Copy, Default, Debug)]
struct PointAndDelta {
    x: i16,
    y: i16,
    x_delta: f32,
    y_delta: f32,
}

// This structure will be used by the `VariationTuples` stack buffer,
// so it has to be as small as possible.
#[derive(Clone, Copy, Default)]
struct VariationTuple<'a> {
    set_points: Option<SetPointsIter<'a>>,
    deltas: PackedDeltasIter<'a>,
    /// The last parsed point with delta in the contour.
    /// Used during delta resolving.
    prev_point: Option<PointAndDelta>,
}

/// The maximum number of variation tuples stored on the stack.
///
/// The TrueType spec allows up to 4095 tuples, which is way larger
/// than we do. But in reality, an average font will have less than 10 tuples.
/// We can avoid heap allocations if the number of tuples is less than this number.
const MAX_STACK_TUPLES_LEN: u16 = 32;

/// A list of variation tuples, possibly stored on the heap.
///
/// This is the only part of the `gvar` algorithm that actually allocates a data.
/// This is probably unavoidable due to `gvar` structure,
/// since we have to iterate all tuples in parallel.
enum VariationTuples<'a> {
    Stack {
        headers: [VariationTuple<'a>; MAX_STACK_TUPLES_LEN as usize],
        len: u16,
    },
    #[cfg(feature = "gvar-alloc")]
    Heap {
        vec: std::vec::Vec<VariationTuple<'a>>,
    },
}

impl<'a> Default for VariationTuples<'a> {
    fn default() -> Self {
        Self::Stack {
            headers: [VariationTuple::default(); MAX_STACK_TUPLES_LEN as usize],
            len: 0,
        }
    }
}

impl<'a> VariationTuples<'a> {
    /// Attempt to reserve up to `capacity` total slots for variation tuples.
    #[cfg(feature = "gvar-alloc")]
    fn reserve(&mut self, capacity: u16) -> bool {
        // If the requested capacity exceeds the configured maximum stack tuple size ...
        if capacity > MAX_STACK_TUPLES_LEN {
            // ... and we're currently on the stack, move to the heap.
            if let Self::Stack { headers, len } = self {
                let mut vec = std::vec::Vec::with_capacity(capacity as usize);
                for header in headers.iter_mut().take(*len as usize) {
                    let header = core::mem::take(header);
                    vec.push(header);
                }

                *self = Self::Heap { vec };
                return true;
            }
        }

        // Otherwise ...
        match self {
            // ... extend the vec capacity to hold our new elements ...
            Self::Heap { vec } if vec.len() < capacity as usize => {
                vec.reserve(capacity as usize - vec.len());
                true
            }
            // ... or do nothing if the vec is already large enough or we're on the stack.
            _ => true,
        }
    }

    /// Attempt to reserve up to `capacity` total slots for variation tuples.
    #[cfg(not(feature = "gvar-alloc"))]
    fn reserve(&mut self, capacity: u16) -> bool {
        capacity <= MAX_STACK_TUPLES_LEN
    }

    /// Get the number of tuples stored in the structure.
    #[cfg_attr(not(feature = "gvar-alloc"), allow(dead_code))]
    fn len(&self) -> u16 {
        match self {
            Self::Stack { len, .. } => *len,
            #[cfg(feature = "gvar-alloc")]
            Self::Heap { vec } => vec.len() as u16,
        }
    }

    /// Append a new tuple header to the list.
    /// This may panic if the list can't hold a new header.
    #[cfg(feature = "gvar-alloc")]
    fn push(&mut self, header: VariationTuple<'a>) {
        // Reserve space for the new element.
        // This may fail and result in a later panic, but that matches pre-heap behavior.
        self.reserve(self.len() + 1);

        match self {
            Self::Stack { headers, len } => {
                headers[usize::from(*len)] = header;
                *len += 1;
            }
            Self::Heap { vec } => vec.push(header),
        }
    }

    /// Append a new tuple header to the list.
    /// This may panic if the list can't hold a new header.
    #[cfg(not(feature = "gvar-alloc"))]
    #[inline]
    fn push(&mut self, header: VariationTuple<'a>) {
        match self {
            Self::Stack { headers, len } => {
                headers[usize::from(*len)] = header;
                *len += 1;
            }
        }
    }

    /// Remove all tuples from the structure.
    fn clear(&mut self) {
        match self {
            Self::Stack { len, .. } => *len = 0,
            #[cfg(feature = "gvar-alloc")]
            Self::Heap { vec } => vec.clear(),
        }
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [VariationTuple<'a>] {
        match self {
            Self::Stack { headers, len } => &mut headers[0..usize::from(*len)],
            #[cfg(feature = "gvar-alloc")]
            Self::Heap { vec } => vec.as_mut_slice(),
        }
    }

    fn apply(
        &mut self,
        all_points: glyf::GlyphPointsIter,
        points: glyf::GlyphPointsIter,
        point: glyf::GlyphPoint,
    ) -> Option<PointF> {
        let mut x = f32::from(point.x);
        let mut y = f32::from(point.y);

        for tuple in self.as_mut_slice() {
            if let Some(ref mut set_points) = tuple.set_points {
                if set_points.next()? {
                    if let Some((x_delta, y_delta)) = tuple.deltas.next() {
                        // Remember the last set point and delta.
                        tuple.prev_point = Some(PointAndDelta {
                            x: point.x,
                            y: point.y,
                            x_delta,
                            y_delta,
                        });

                        x += x_delta;
                        y += y_delta;
                    } else {
                        // If there are no more deltas, we have to resolve them manually.
                        let set_points = set_points.clone();
                        let (x_delta, y_delta) = infer_deltas(
                            tuple,
                            set_points,
                            points.clone(),
                            all_points.clone(),
                            point,
                        );

                        x += x_delta;
                        y += y_delta;
                    }
                } else {
                    // Point is not referenced, so we have to resolve it.
                    let set_points = set_points.clone();
                    let (x_delta, y_delta) =
                        infer_deltas(tuple, set_points, points.clone(), all_points.clone(), point);

                    x += x_delta;
                    y += y_delta;
                }

                if point.last_point {
                    tuple.prev_point = None;
                }
            } else {
                if let Some((x_delta, y_delta)) = tuple.deltas.next() {
                    x += x_delta;
                    y += y_delta;
                }
            }
        }

        Some(PointF { x, y })
    }

    // This is just like `apply()`, but without `infer_deltas`,
    // since we use it only for component points and not a contour.
    // And since there are no contour and no points, `infer_deltas()` will do nothing.
    fn apply_null(&mut self) -> Option<PointF> {
        let mut x = 0.0;
        let mut y = 0.0;

        for tuple in self.as_mut_slice() {
            if let Some(ref mut set_points) = tuple.set_points {
                if set_points.next()? {
                    if let Some((x_delta, y_delta)) = tuple.deltas.next() {
                        x += x_delta;
                        y += y_delta;
                    }
                }
            } else {
                if let Some((x_delta, y_delta)) = tuple.deltas.next() {
                    x += x_delta;
                    y += y_delta;
                }
            }
        }

        Some(PointF { x, y })
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct TupleVariationHeaderData {
    scalar: f32,
    has_private_point_numbers: bool,
    serialized_data_len: u16,
}

// https://docs.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#tuplevariationheader
fn parse_variation_tuples<'a>(
    count: u16,
    coordinates: &[NormalizedCoordinate],
    shared_tuple_records: &LazyArray16<F2DOT14>,
    shared_point_numbers: Option<PackedPointsIter<'a>>,
    points_len: u16,
    mut main_s: Stream<'a>,
    mut serialized_s: Stream<'a>,
    tuples: &mut VariationTuples<'a>,
) -> Option<()> {
    debug_assert!(core::mem::size_of::<VariationTuple>() <= 80);

    // `TupleVariationHeader` has a variable size, so we cannot use a `LazyArray`.
    for _ in 0..count {
        let header = parse_tuple_variation_header(coordinates, shared_tuple_records, &mut main_s)?;
        if !(header.scalar > 0.0) {
            // Serialized data for headers with non-positive scalar should be skipped.
            serialized_s.advance(usize::from(header.serialized_data_len));
            continue;
        }

        let serialized_data_start = serialized_s.offset();

        // Resolve point numbers source.
        let point_numbers = if header.has_private_point_numbers {
            PackedPointsIter::new(&mut serialized_s)?
        } else {
            shared_point_numbers.clone()
        };

        // TODO: this
        // Since the packed representation can include zero values,
        // it is possible for a given point number to be repeated in the derived point number list.
        // In that case, there will be multiple delta values in the deltas data
        // associated with that point number. All of these deltas must be applied
        // cumulatively to the given point.

        let deltas_count = if let Some(point_numbers) = point_numbers.clone() {
            u16::try_from(point_numbers.clone().count()).ok()?
        } else {
            points_len
        };

        let deltas = {
            // Use `checked_sub` in case we went over the `serialized_data_len`.
            let left = usize::from(header.serialized_data_len)
                .checked_sub(serialized_s.offset() - serialized_data_start)?;
            let deltas_data = serialized_s.read_bytes(left)?;
            PackedDeltasIter::new(header.scalar, deltas_count, deltas_data)
        };

        let tuple = VariationTuple {
            set_points: point_numbers.map(SetPointsIter::new),
            deltas,
            prev_point: None,
        };

        tuples.push(tuple);
    }

    Some(())
}

// https://docs.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#tuplevariationheader
fn parse_tuple_variation_header(
    coordinates: &[NormalizedCoordinate],
    shared_tuple_records: &LazyArray16<F2DOT14>,
    s: &mut Stream,
) -> Option<TupleVariationHeaderData> {
    const EMBEDDED_PEAK_TUPLE_FLAG: u16 = 0x8000;
    const INTERMEDIATE_REGION_FLAG: u16 = 0x4000;
    const PRIVATE_POINT_NUMBERS_FLAG: u16 = 0x2000;
    const TUPLE_INDEX_MASK: u16 = 0x0FFF;

    let serialized_data_size = s.read::<u16>()?;
    let tuple_index = s.read::<u16>()?;

    let has_embedded_peak_tuple = tuple_index & EMBEDDED_PEAK_TUPLE_FLAG != 0;
    let has_intermediate_region = tuple_index & INTERMEDIATE_REGION_FLAG != 0;
    let has_private_point_numbers = tuple_index & PRIVATE_POINT_NUMBERS_FLAG != 0;
    let tuple_index = tuple_index & TUPLE_INDEX_MASK;

    let axis_count = coordinates.len() as u16;

    let peak_tuple = if has_embedded_peak_tuple {
        s.read_array16::<F2DOT14>(axis_count)?
    } else {
        // Use shared tuples.
        let start = tuple_index.checked_mul(axis_count)?;
        let end = start.checked_add(axis_count)?;
        shared_tuple_records.slice(start..end)?
    };

    let (start_tuple, end_tuple) = if has_intermediate_region {
        (
            s.read_array16::<F2DOT14>(axis_count)?,
            s.read_array16::<F2DOT14>(axis_count)?,
        )
    } else {
        (
            LazyArray16::<F2DOT14>::default(),
            LazyArray16::<F2DOT14>::default(),
        )
    };

    let mut header = TupleVariationHeaderData {
        scalar: 0.0,
        has_private_point_numbers,
        serialized_data_len: serialized_data_size,
    };

    // Calculate the scalar value according to the pseudo-code described at:
    // https://docs.microsoft.com/en-us/typography/opentype/spec/otvaroverview#algorithm-for-interpolation-of-instance-values
    let mut scalar = 1.0;
    for i in 0..axis_count {
        let v = coordinates[usize::from(i)].get();
        let peak = peak_tuple.get(i)?.0;
        if peak == 0 || v == peak {
            continue;
        }

        if has_intermediate_region {
            let start = start_tuple.get(i)?.0;
            let end = end_tuple.get(i)?.0;
            if start > peak || peak > end || (start < 0 && end > 0 && peak != 0) {
                continue;
            }

            if v < start || v > end {
                return Some(header);
            }

            if v < peak {
                if peak != start {
                    scalar *= f32::from(v - start) / f32::from(peak - start);
                }
            } else {
                if peak != end {
                    scalar *= f32::from(end - v) / f32::from(end - peak);
                }
            }
        } else if v == 0 || v < cmp::min(0, peak) || v > cmp::max(0, peak) {
            // 'If the instance coordinate is out of range for some axis, then the
            // region and its associated deltas are not applicable.'
            return Some(header);
        } else {
            scalar *= f32::from(v) / f32::from(peak);
        }
    }

    header.scalar = scalar;
    Some(header)
}

// https://docs.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#packed-point-numbers
mod packed_points {
    use crate::parser::{FromData, Stream};

    struct Control(u8);

    impl Control {
        const POINTS_ARE_WORDS_FLAG: u8 = 0x80;
        const POINT_RUN_COUNT_MASK: u8 = 0x7F;

        #[inline]
        fn is_points_are_words(&self) -> bool {
            self.0 & Self::POINTS_ARE_WORDS_FLAG != 0
        }

        // 'Mask for the low 7 bits to provide the number of point values in the run, minus one.'
        // So we have to add 1.
        // It will never overflow because of a mask.
        #[inline]
        fn run_count(&self) -> u8 {
            (self.0 & Self::POINT_RUN_COUNT_MASK) + 1
        }
    }

    impl FromData for Control {
        const SIZE: usize = 1;

        #[inline]
        fn parse(data: &[u8]) -> Option<Self> {
            data.get(0).copied().map(Control)
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    enum State {
        Control,
        ShortPoint,
        LongPoint,
    }

    // This structure will be used by the `VariationTuples` stack buffer,
    // so it has to be as small as possible.
    // Therefore we cannot use `Stream` and other abstractions.
    #[derive(Clone, Copy)]
    pub struct PackedPointsIter<'a> {
        data: &'a [u8],
        // u16 is enough, since the maximum number of points is 32767.
        offset: u16,
        state: State,
        points_left: u8,
    }

    impl<'a> PackedPointsIter<'a> {
        // The first Option::None indicates a parsing error.
        // The second Option::None indicates "no points".
        pub fn new<'b>(s: &'b mut Stream<'a>) -> Option<Option<Self>> {
            // The total amount of points can be set as one or two bytes
            // depending on the first bit.
            let b1 = s.read::<u8>()?;
            let mut count = u16::from(b1);
            if b1 & Control::POINTS_ARE_WORDS_FLAG != 0 {
                let b2 = s.read::<u8>()?;
                count = (u16::from(b1 & Control::POINT_RUN_COUNT_MASK) << 8) | u16::from(b2);
            }

            if count == 0 {
                // No points is not an error.
                return Some(None);
            }

            let start = s.offset();
            let tail = s.tail()?;

            // The actual packed points data size is not stored,
            // so we have to parse the points first to advance the provided stream.
            // Since deltas will be right after points.
            let mut i = 0;
            while i < count {
                let control = s.read::<Control>()?;
                let run_count = u16::from(control.run_count());
                let is_points_are_words = control.is_points_are_words();
                // Do not actually parse the number, simply advance.
                s.advance_checked(
                    if is_points_are_words { 2 } else { 1 } * usize::from(run_count),
                )?;
                i += run_count;
            }

            if i == 0 {
                // No points is not an error.
                return Some(None);
            }

            if i > count {
                // Malformed font.
                return None;
            }

            // Check that points data size is smaller than the storage type
            // used by the iterator.
            let data_len = s.offset() - start;
            if data_len > usize::from(u16::MAX) {
                return None;
            }

            Some(Some(PackedPointsIter {
                data: &tail[0..data_len],
                offset: 0,
                state: State::Control,
                points_left: 0,
            }))
        }
    }

    impl<'a> Iterator for PackedPointsIter<'a> {
        type Item = u16;

        fn next(&mut self) -> Option<Self::Item> {
            if usize::from(self.offset) >= self.data.len() {
                return None;
            }

            if self.state == State::Control {
                let control = Control(self.data[usize::from(self.offset)]);
                self.offset += 1;

                self.points_left = control.run_count();
                self.state = if control.is_points_are_words() {
                    State::LongPoint
                } else {
                    State::ShortPoint
                };

                self.next()
            } else {
                let mut s = Stream::new_at(self.data, usize::from(self.offset))?;
                let point = if self.state == State::LongPoint {
                    self.offset += 2;
                    s.read::<u16>()?
                } else {
                    self.offset += 1;
                    u16::from(s.read::<u8>()?)
                };

                self.points_left -= 1;
                if self.points_left == 0 {
                    self.state = State::Control;
                }

                Some(point)
            }
        }
    }

    // The `PackedPointsIter` will return referenced point numbers as deltas.
    // i.e. 1 2 4 is actually 1 3 7
    // But this is not very useful in our current algorithm,
    // so we will convert it once again into:
    // false true false true false false false true
    // This way we can iterate glyph points and point numbers in parallel.
    #[derive(Clone, Copy)]
    pub struct SetPointsIter<'a> {
        iter: PackedPointsIter<'a>,
        unref_count: u16,
    }

    impl<'a> SetPointsIter<'a> {
        #[inline]
        pub fn new(mut iter: PackedPointsIter<'a>) -> Self {
            let unref_count = iter.next().unwrap_or(0);
            SetPointsIter { iter, unref_count }
        }

        #[inline]
        pub fn restart(self) -> Self {
            let mut iter = self.iter.clone();
            iter.offset = 0;
            iter.state = State::Control;
            iter.points_left = 0;

            let unref_count = iter.next().unwrap_or(0);
            SetPointsIter { iter, unref_count }
        }
    }

    impl<'a> Iterator for SetPointsIter<'a> {
        type Item = bool;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            if self.unref_count != 0 {
                self.unref_count -= 1;
                return Some(false);
            }

            if let Some(unref_count) = self.iter.next() {
                self.unref_count = unref_count;
                if self.unref_count != 0 {
                    self.unref_count -= 1;
                }
            }

            // Iterator will be returning `Some(true)` after "finished".
            // This is because this iterator will be zipped with the `glyf::GlyphPointsIter`
            // and the number of glyph points can be larger than the amount of set points.
            // Anyway, this is a non-issue in a well-formed font.
            Some(true)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        struct NewControl {
            deltas_are_words: bool,
            run_count: u8,
        }

        fn gen_control(control: NewControl) -> u8 {
            assert!(control.run_count > 0, "run count cannot be zero");

            let mut n = 0;
            if control.deltas_are_words {
                n |= 0x80;
            }
            n |= (control.run_count - 1) & 0x7F;
            n
        }

        #[test]
        fn empty() {
            let mut s = Stream::new(&[]);
            assert!(PackedPointsIter::new(&mut s).is_none());
        }

        #[test]
        fn single_zero_control() {
            let mut s = Stream::new(&[0]);
            assert!(PackedPointsIter::new(&mut s).unwrap().is_none());
        }

        #[test]
        fn single_point() {
            let data = vec![
                1, // total count
                gen_control(NewControl {
                    deltas_are_words: false,
                    run_count: 1,
                }),
                1,
            ];

            let points_iter = PackedPointsIter::new(&mut Stream::new(&data))
                .unwrap()
                .unwrap();
            let mut iter = SetPointsIter::new(points_iter);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), true); // Endlessly true.
        }

        #[test]
        fn set_0_and_2() {
            let data = vec![
                2, // total count
                gen_control(NewControl {
                    deltas_are_words: false,
                    run_count: 2,
                }),
                0,
                2,
            ];

            let points_iter = PackedPointsIter::new(&mut Stream::new(&data))
                .unwrap()
                .unwrap();
            let mut iter = SetPointsIter::new(points_iter);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), true); // Endlessly true.
        }

        #[test]
        fn set_1_and_2() {
            let data = vec![
                2, // total count
                gen_control(NewControl {
                    deltas_are_words: false,
                    run_count: 2,
                }),
                1,
                1,
            ];

            let points_iter = PackedPointsIter::new(&mut Stream::new(&data))
                .unwrap()
                .unwrap();
            let mut iter = SetPointsIter::new(points_iter);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), true); // Endlessly true.
        }

        #[test]
        fn set_1_and_3() {
            let data = vec![
                2, // total count
                gen_control(NewControl {
                    deltas_are_words: false,
                    run_count: 2,
                }),
                1,
                2,
            ];

            let points_iter = PackedPointsIter::new(&mut Stream::new(&data))
                .unwrap()
                .unwrap();
            let mut iter = SetPointsIter::new(points_iter);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), true); // Endlessly true.
        }

        #[test]
        fn set_2_5_7() {
            let data = vec![
                3, // total count
                gen_control(NewControl {
                    deltas_are_words: false,
                    run_count: 3,
                }),
                2,
                3,
                2,
            ];

            let points_iter = PackedPointsIter::new(&mut Stream::new(&data))
                .unwrap()
                .unwrap();
            let mut iter = SetPointsIter::new(points_iter);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), true); // Endlessly true.
        }

        #[test]
        fn more_than_127_points() {
            let mut data = vec![];
            // total count
            data.push(Control::POINTS_ARE_WORDS_FLAG);
            data.push(150);

            data.push(gen_control(NewControl {
                deltas_are_words: false,
                run_count: 100,
            }));
            for _ in 0..100 {
                data.push(2);
            }
            data.push(gen_control(NewControl {
                deltas_are_words: false,
                run_count: 50,
            }));
            for _ in 0..50 {
                data.push(2);
            }
            let points_iter = PackedPointsIter::new(&mut Stream::new(&data))
                .unwrap()
                .unwrap();
            let mut iter = SetPointsIter::new(points_iter);
            assert_eq!(iter.next().unwrap(), false);
            for _ in 0..150 {
                assert_eq!(iter.next().unwrap(), false);
                assert_eq!(iter.next().unwrap(), true);
            }
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), true); // Endlessly true.
        }

        #[test]
        fn long_points() {
            let data = vec![
                2, // total count
                gen_control(NewControl {
                    deltas_are_words: true,
                    run_count: 2,
                }),
                0,
                2,
                0,
                3,
            ];

            let points_iter = PackedPointsIter::new(&mut Stream::new(&data))
                .unwrap()
                .unwrap();
            let mut iter = SetPointsIter::new(points_iter);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), true); // Endlessly true.
        }

        #[test]
        fn multiple_runs() {
            let data = vec![
                5, // total count
                gen_control(NewControl {
                    deltas_are_words: true,
                    run_count: 2,
                }),
                0,
                2,
                0,
                3,
                gen_control(NewControl {
                    deltas_are_words: false,
                    run_count: 3,
                }),
                2,
                3,
                2,
            ];

            let points_iter = PackedPointsIter::new(&mut Stream::new(&data))
                .unwrap()
                .unwrap();
            let mut iter = SetPointsIter::new(points_iter);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), false);
            assert_eq!(iter.next().unwrap(), true);
            assert_eq!(iter.next().unwrap(), true); // Endlessly true.
        }

        #[test]
        fn runs_overflow() {
            // TrueType allows up to 32767 points.
            let data = vec![0xFF; 0xFFFF * 2];
            assert!(PackedPointsIter::new(&mut Stream::new(&data)).is_none());
        }
    }
}

use packed_points::*;

// https://docs.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#packed-deltas
mod packed_deltas {
    use crate::parser::Stream;

    struct Control(u8);

    impl Control {
        const DELTAS_ARE_ZERO_FLAG: u8 = 0x80;
        const DELTAS_ARE_WORDS_FLAG: u8 = 0x40;
        const DELTA_RUN_COUNT_MASK: u8 = 0x3F;

        #[inline]
        fn is_deltas_are_zero(&self) -> bool {
            self.0 & Self::DELTAS_ARE_ZERO_FLAG != 0
        }

        #[inline]
        fn is_deltas_are_words(&self) -> bool {
            self.0 & Self::DELTAS_ARE_WORDS_FLAG != 0
        }

        // 'Mask for the low 6 bits to provide the number of delta values in the run, minus one.'
        // So we have to add 1.
        // It will never overflow because of a mask.
        #[inline]
        fn run_count(&self) -> u8 {
            (self.0 & Self::DELTA_RUN_COUNT_MASK) + 1
        }
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    enum State {
        Control,
        ZeroDelta,
        ShortDelta,
        LongDelta,
    }

    impl Default for State {
        #[inline]
        fn default() -> Self {
            State::Control
        }
    }

    #[derive(Clone, Copy, Default)]
    struct RunState {
        data_offset: u16,
        state: State,
        run_deltas_left: u8,
    }

    impl RunState {
        fn next(&mut self, data: &[u8], scalar: f32) -> Option<f32> {
            if self.state == State::Control {
                if usize::from(self.data_offset) == data.len() {
                    return None;
                }

                let control = Control(Stream::read_at::<u8>(data, usize::from(self.data_offset))?);
                self.data_offset += 1;

                self.run_deltas_left = control.run_count();
                self.state = if control.is_deltas_are_zero() {
                    State::ZeroDelta
                } else if control.is_deltas_are_words() {
                    State::LongDelta
                } else {
                    State::ShortDelta
                };

                self.next(data, scalar)
            } else {
                let mut s = Stream::new_at(data, usize::from(self.data_offset))?;
                let delta = if self.state == State::LongDelta {
                    self.data_offset += 2;
                    f32::from(s.read::<i16>()?) * scalar
                } else if self.state == State::ZeroDelta {
                    0.0
                } else {
                    self.data_offset += 1;
                    f32::from(s.read::<i8>()?) * scalar
                };

                self.run_deltas_left -= 1;
                if self.run_deltas_left == 0 {
                    self.state = State::Control;
                }

                Some(delta)
            }
        }
    }

    // This structure will be used by the `VariationTuples` stack buffer,
    // so it has to be as small as possible.
    // Therefore we cannot use `Stream` and other abstractions.
    #[derive(Clone, Copy, Default)]
    pub struct PackedDeltasIter<'a> {
        data: &'a [u8],
        x_run: RunState,
        y_run: RunState,

        /// A total number of deltas per axis.
        ///
        /// Required only by restart()
        total_count: u16,

        scalar: f32,
    }

    impl<'a> PackedDeltasIter<'a> {
        /// `count` indicates a number of delta pairs.
        pub fn new(scalar: f32, count: u16, data: &'a [u8]) -> Self {
            debug_assert!(core::mem::size_of::<PackedDeltasIter>() <= 32);

            let mut iter = PackedDeltasIter {
                data,
                total_count: count,
                scalar,
                ..PackedDeltasIter::default()
            };

            // 'The packed deltas are arranged with all of the deltas for X coordinates first,
            // followed by the deltas for Y coordinates.'
            // So we have to skip X deltas in the Y deltas iterator.
            //
            // Note that Y deltas doesn't necessarily start with a Control byte
            // and can actually start in the middle of the X run.
            // So we can't simply split the input data in half
            // and process those chunks separately.
            for _ in 0..count {
                iter.y_run.next(data, scalar);
            }

            iter
        }

        #[inline]
        pub fn restart(self) -> Self {
            PackedDeltasIter::new(self.scalar, self.total_count, self.data)
        }

        #[inline]
        pub fn next(&mut self) -> Option<(f32, f32)> {
            let x = self.x_run.next(self.data, self.scalar)?;
            let y = self.y_run.next(self.data, self.scalar)?;
            Some((x, y))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        struct NewControl {
            deltas_are_zero: bool,
            deltas_are_words: bool,
            run_count: u8,
        }

        fn gen_control(control: NewControl) -> u8 {
            assert!(control.run_count > 0, "run count cannot be zero");

            let mut n = 0;
            if control.deltas_are_zero {
                n |= 0x80;
            }
            if control.deltas_are_words {
                n |= 0x40;
            }
            n |= (control.run_count - 1) & 0x3F;
            n
        }

        #[test]
        fn empty() {
            let mut iter = PackedDeltasIter::new(1.0, 1, &[]);
            assert!(iter.next().is_none());
        }

        #[test]
        fn single_delta() {
            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: false,
                    run_count: 2,
                }),
                2,
                3,
            ];

            let mut iter = PackedDeltasIter::new(1.0, 1, &data);
            assert_eq!(iter.next().unwrap(), (2.0, 3.0));
            assert!(iter.next().is_none());
        }

        #[test]
        fn two_deltas() {
            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: false,
                    run_count: 4,
                }),
                2,
                3,
                4,
                5,
            ];

            let mut iter = PackedDeltasIter::new(1.0, 2, &data);
            // Remember that X deltas are defined first.
            assert_eq!(iter.next().unwrap(), (2.0, 4.0));
            assert_eq!(iter.next().unwrap(), (3.0, 5.0));
            assert!(iter.next().is_none());
        }

        #[test]
        fn single_long_delta() {
            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: true,
                    run_count: 2,
                }),
                0,
                2,
                0,
                3,
            ];

            let mut iter = PackedDeltasIter::new(1.0, 1, &data);
            assert_eq!(iter.next().unwrap(), (2.0, 3.0));
            assert!(iter.next().is_none());
        }

        #[test]
        fn zeros() {
            let data = vec![gen_control(NewControl {
                deltas_are_zero: true,
                deltas_are_words: false,
                run_count: 4,
            })];

            let mut iter = PackedDeltasIter::new(1.0, 2, &data);
            assert_eq!(iter.next().unwrap(), (0.0, 0.0));
            assert_eq!(iter.next().unwrap(), (0.0, 0.0));
            assert!(iter.next().is_none());
        }

        #[test]
        fn zero_words() {
            // When `deltas_are_zero` is set, `deltas_are_words` should be ignored.

            let data = vec![gen_control(NewControl {
                deltas_are_zero: true,
                deltas_are_words: true,
                run_count: 4,
            })];

            let mut iter = PackedDeltasIter::new(1.0, 2, &data);
            assert_eq!(iter.next().unwrap(), (0.0, 0.0));
            assert_eq!(iter.next().unwrap(), (0.0, 0.0));
            assert!(iter.next().is_none());
        }

        #[test]
        fn zero_runs() {
            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: true,
                    deltas_are_words: false,
                    run_count: 2,
                }),
                gen_control(NewControl {
                    deltas_are_zero: true,
                    deltas_are_words: false,
                    run_count: 4,
                }),
                gen_control(NewControl {
                    deltas_are_zero: true,
                    deltas_are_words: false,
                    run_count: 6,
                }),
            ];

            let mut iter = PackedDeltasIter::new(1.0, 6, &data);
            // First run.
            assert_eq!(iter.next().unwrap(), (0.0, 0.0));
            // Second run.
            assert_eq!(iter.next().unwrap(), (0.0, 0.0));
            assert_eq!(iter.next().unwrap(), (0.0, 0.0));
            // Third run.
            assert_eq!(iter.next().unwrap(), (0.0, 0.0));
            assert_eq!(iter.next().unwrap(), (0.0, 0.0));
            assert_eq!(iter.next().unwrap(), (0.0, 0.0));
            assert!(iter.next().is_none());
        }

        #[test]
        fn delta_after_zeros() {
            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: true,
                    deltas_are_words: false,
                    run_count: 2,
                }),
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: false,
                    run_count: 2,
                }),
                2,
                3,
            ];

            let mut iter = PackedDeltasIter::new(1.0, 2, &data);
            assert_eq!(iter.next().unwrap(), (0.0, 2.0));
            assert_eq!(iter.next().unwrap(), (0.0, 3.0));
            assert!(iter.next().is_none());
        }

        #[test]
        fn unexpected_end_of_data_1() {
            let data = vec![gen_control(NewControl {
                deltas_are_zero: false,
                deltas_are_words: false,
                run_count: 2,
            })];

            let mut iter = PackedDeltasIter::new(1.0, 1, &data);
            assert!(iter.next().is_none());
        }

        #[test]
        fn unexpected_end_of_data_2() {
            // Only X is set.

            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: false,
                    run_count: 2,
                }),
                1,
            ];

            let mut iter = PackedDeltasIter::new(1.0, 1, &data);
            assert!(iter.next().is_none());
        }

        #[test]
        fn unexpected_end_of_data_3() {
            let data = vec![gen_control(NewControl {
                deltas_are_zero: false,
                deltas_are_words: true,
                run_count: 2,
            })];

            let mut iter = PackedDeltasIter::new(1.0, 1, &data);
            assert!(iter.next().is_none());
        }

        #[test]
        fn unexpected_end_of_data_4() {
            // X data is too short.

            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: true,
                    run_count: 2,
                }),
                1,
            ];

            let mut iter = PackedDeltasIter::new(1.0, 1, &data);
            assert!(iter.next().is_none());
        }

        #[test]
        fn unexpected_end_of_data_6() {
            // Only X is set.

            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: true,
                    run_count: 2,
                }),
                0,
                1,
            ];

            let mut iter = PackedDeltasIter::new(1.0, 1, &data);
            assert!(iter.next().is_none());
        }

        #[test]
        fn unexpected_end_of_data_7() {
            // Y data is too short.

            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: true,
                    run_count: 2,
                }),
                0,
                1,
                0,
            ];

            let mut iter = PackedDeltasIter::new(1.0, 1, &data);
            assert!(iter.next().is_none());
        }

        #[test]
        fn single_run() {
            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: false,
                    run_count: 1,
                }),
                2,
                3,
            ];

            let mut iter = PackedDeltasIter::new(1.0, 1, &data);
            assert!(iter.next().is_none());
        }

        #[test]
        fn too_many_pairs() {
            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: false,
                    run_count: 2,
                }),
                2,
                3,
            ];

            // We have only one pair, not 10.
            let mut iter = PackedDeltasIter::new(1.0, 10, &data);
            assert!(iter.next().is_none());
        }

        #[test]
        fn invalid_number_of_pairs() {
            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: false,
                    run_count: 2,
                }),
                2,
                3,
                4,
                5,
                6,
                7,
            ];

            // We have 3 pairs, not 4.
            // We don't actually check this, since it will be very expensive.
            // And it should not happen in a well-formed font anyway.
            // So as long as it doesn't panic - we are fine.
            let mut iter = PackedDeltasIter::new(1.0, 4, &data);
            assert_eq!(iter.next().unwrap(), (2.0, 7.0));
            assert!(iter.next().is_none());
        }

        #[test]
        fn mixed_runs() {
            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: false,
                    run_count: 3,
                }),
                2,
                3,
                4,
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: true,
                    run_count: 2,
                }),
                0,
                5,
                0,
                6,
                gen_control(NewControl {
                    deltas_are_zero: true,
                    deltas_are_words: false,
                    run_count: 1,
                }),
            ];

            let mut iter = PackedDeltasIter::new(1.0, 3, &data);
            assert_eq!(iter.next().unwrap(), (2.0, 5.0));
            assert_eq!(iter.next().unwrap(), (3.0, 6.0));
            assert_eq!(iter.next().unwrap(), (4.0, 0.0));
            assert!(iter.next().is_none());
        }

        #[test]
        fn non_default_scalar() {
            let data = vec![
                gen_control(NewControl {
                    deltas_are_zero: false,
                    deltas_are_words: false,
                    run_count: 2,
                }),
                2,
                3,
            ];

            let mut iter = PackedDeltasIter::new(0.5, 1, &data);
            assert_eq!(iter.next().unwrap(), (1.0, 1.5));
            assert!(iter.next().is_none());
        }

        #[test]
        fn runs_overflow() {
            let data = vec![0xFF; 0xFFFF];
            let mut iter = PackedDeltasIter::new(1.0, 0xFFFF, &data);
            // As long as it doesn't panic - we are fine.
            assert_eq!(iter.next().unwrap(), (0.0, 0.0));
        }
    }
}

use packed_deltas::PackedDeltasIter;

/// Infer unreferenced deltas.
///
/// A font can define deltas only for specific points, to reduce the file size.
/// In this case, we have to infer undefined/unreferenced deltas manually,
/// depending on the context.
///
/// This is already a pretty complex task, since deltas should be resolved
/// only inside the current contour (do not confuse with component).
/// And during resolving we can actually wrap around the contour.
/// So if there is no deltas after the current one, we have to use
/// the first delta of the current contour instead.
/// Same goes for the previous delta. If there are no deltas
/// before the current one, we have to use the last one in the current contour.
///
/// And in case of `ttf-parser` everything is becoming even more complex,
/// since we don't actually have a list of points and deltas, only iterators.
/// Because of `ttf-parser`'s allocation free policy.
/// Which makes the code even more complicated.
///
/// https://docs.microsoft.com/en-us/typography/opentype/spec/gvar#inferred-deltas-for-un-referenced-point-numbers
fn infer_deltas(
    tuple: &VariationTuple,
    points_set: SetPointsIter,
    // A points iterator that starts after the current point.
    points: glyf::GlyphPointsIter,
    // A points iterator that starts from the first point in the glyph.
    all_points: glyf::GlyphPointsIter,
    curr_point: glyf::GlyphPoint,
) -> (f32, f32) {
    let mut current_contour = points.current_contour();
    if curr_point.last_point && current_contour != 0 {
        // When we parsed the last point of a contour,
        // an iterator had switched to the next contour.
        // So we have to move to the previous one.
        current_contour -= 1;
    }

    let prev_point = if let Some(prev_point) = tuple.prev_point {
        // If a contour already had a delta - just use it.
        prev_point
    } else {
        // If not, find the last point with delta in the current contour.
        let mut last_point = None;
        let mut deltas = tuple.deltas.clone();
        for (point, is_set) in points.clone().zip(points_set.clone()) {
            if is_set {
                if let Some((x_delta, y_delta)) = deltas.next() {
                    last_point = Some(PointAndDelta {
                        x: point.x,
                        y: point.y,
                        x_delta,
                        y_delta,
                    });
                }
            }

            if point.last_point {
                break;
            }
        }

        // If there is no last point, there are no deltas.
        match last_point {
            Some(p) => p,
            None => return (0.0, 0.0),
        }
    };

    let mut next_point = None;
    if !curr_point.last_point {
        // If the current point is not the last one in the contour,
        // find the first set delta in the current contour.
        let mut deltas = tuple.deltas.clone();
        for (point, is_set) in points.clone().zip(points_set.clone()) {
            if is_set {
                if let Some((x_delta, y_delta)) = deltas.next() {
                    next_point = Some(PointAndDelta {
                        x: point.x,
                        y: point.y,
                        x_delta,
                        y_delta,
                    });
                }

                break;
            }

            if point.last_point {
                break;
            }
        }
    }

    if next_point.is_none() {
        // If there were no deltas after the current point,
        // restart from the start of the contour.
        //
        // This is probably the most expensive branch,
        // but nothing we can do about it since `glyf`/`gvar` data structure
        // doesn't allow implementing a reverse iterator.
        // So we have to parse everything once again.

        let mut all_points = all_points.clone();
        let mut deltas = tuple.deltas.clone().restart();
        let mut points_set = points_set.clone().restart();

        let mut contour = 0;
        while let (Some(point), Some(is_set)) = (all_points.next(), points_set.next()) {
            // First, we have to skip already processed contours.
            if contour != current_contour {
                if is_set {
                    let _ = deltas.next();
                }

                contour = all_points.current_contour();
                continue;
            }

            if is_set {
                let (x_delta, y_delta) = deltas.next().unwrap_or((0.0, 0.0));
                next_point = Some(PointAndDelta {
                    x: point.x,
                    y: point.y,
                    x_delta,
                    y_delta,
                });

                break;
            }

            if point.last_point {
                break;
            }
        }
    }

    // If there is no next point, there are no deltas.
    let next_point = match next_point {
        Some(p) => p,
        None => return (0.0, 0.0),
    };

    let dx = infer_delta(
        prev_point.x,
        curr_point.x,
        next_point.x,
        prev_point.x_delta,
        next_point.x_delta,
    );

    let dy = infer_delta(
        prev_point.y,
        curr_point.y,
        next_point.y,
        prev_point.y_delta,
        next_point.y_delta,
    );

    (dx, dy)
}

fn infer_delta(
    prev_point: i16,
    target_point: i16,
    next_point: i16,
    prev_delta: f32,
    next_delta: f32,
) -> f32 {
    if prev_point == next_point {
        if prev_delta == next_delta {
            prev_delta
        } else {
            0.0
        }
    } else if target_point <= prev_point.min(next_point) {
        if prev_point < next_point {
            prev_delta
        } else {
            next_delta
        }
    } else if target_point >= prev_point.max(next_point) {
        if prev_point > next_point {
            prev_delta
        } else {
            next_delta
        }
    } else {
        // 'Target point coordinate is between adjacent point coordinates.'
        //
        // 'Target point delta is derived from the adjacent point deltas
        // using linear interpolation.'
        let target_sub = target_point.checked_sub(prev_point);
        let next_sub = next_point.checked_sub(prev_point);
        let d = if let (Some(target_sub), Some(next_sub)) = (target_sub, next_sub) {
            f32::from(target_sub) / f32::from(next_sub)
        } else {
            return 0.0;
        };
        (1.0 - d) * prev_delta + d * next_delta
    }
}

/// A [Glyph Variations Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/gvar).
#[derive(Clone, Copy)]
pub struct Table<'a> {
    axis_count: NonZeroU16,
    shared_tuple_records: LazyArray16<'a, F2DOT14>,
    offsets: GlyphVariationDataOffsets<'a>,
    glyphs_variation_data: &'a [u8],
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let version = s.read::<u32>()?;
        if version != 0x00010000 {
            return None;
        }

        let axis_count = s.read::<u16>()?;
        let shared_tuple_count = s.read::<u16>()?;
        let shared_tuples_offset = s.read::<Offset32>()?;
        let glyph_count = s.read::<u16>()?;
        let flags = s.read::<u16>()?;
        let glyph_variation_data_array_offset = s.read::<Offset32>()?;

        // The axis count cannot be zero.
        let axis_count = NonZeroU16::new(axis_count)?;

        let shared_tuple_records = {
            let mut sub_s = Stream::new_at(data, shared_tuples_offset.to_usize())?;
            sub_s.read_array16::<F2DOT14>(shared_tuple_count.checked_mul(axis_count.get())?)?
        };

        let glyphs_variation_data = data.get(glyph_variation_data_array_offset.to_usize()..)?;
        let offsets = {
            let offsets_count = glyph_count.checked_add(1)?;
            let is_long_format = flags & 1 == 1; // The first bit indicates a long format.
            if is_long_format {
                GlyphVariationDataOffsets::Long(s.read_array16::<Offset32>(offsets_count)?)
            } else {
                GlyphVariationDataOffsets::Short(s.read_array16::<Offset16>(offsets_count)?)
            }
        };

        Some(Table {
            axis_count,
            shared_tuple_records,
            offsets,
            glyphs_variation_data,
        })
    }

    #[inline]
    fn parse_variation_data(
        &self,
        glyph_id: GlyphId,
        coordinates: &[NormalizedCoordinate],
        points_len: u16,
        tuples: &mut VariationTuples<'a>,
    ) -> Option<()> {
        tuples.clear();

        if coordinates.len() != usize::from(self.axis_count.get()) {
            return None;
        }

        let next_glyph_id = glyph_id.0.checked_add(1)?;

        let (start, end) = match self.offsets {
            GlyphVariationDataOffsets::Short(ref array) => {
                // 'If the short format (Offset16) is used for offsets,
                // the value stored is the offset divided by 2.'
                (
                    array.get(glyph_id.0)?.to_usize() * 2,
                    array.get(next_glyph_id)?.to_usize() * 2,
                )
            }
            GlyphVariationDataOffsets::Long(ref array) => (
                array.get(glyph_id.0)?.to_usize(),
                array.get(next_glyph_id)?.to_usize(),
            ),
        };

        // Ignore empty data.
        if start == end {
            return Some(());
        }

        let data = self.glyphs_variation_data.get(start..end)?;
        parse_variation_data(
            coordinates,
            &self.shared_tuple_records,
            points_len,
            data,
            tuples,
        )
    }

    /// Outlines a glyph.
    pub fn outline(
        &self,
        glyf_table: glyf::Table,
        coordinates: &[NormalizedCoordinate],
        glyph_id: GlyphId,
        builder: &mut dyn OutlineBuilder,
    ) -> Option<Rect> {
        let mut b = glyf::Builder::new(Transform::default(), RectF::new(), builder);
        let glyph_data = glyf_table.get(glyph_id)?;
        outline_var_impl(
            glyf_table,
            self,
            glyph_id,
            glyph_data,
            coordinates,
            0,
            &mut b,
        );
        b.bbox.to_rect()
    }

    pub(crate) fn phantom_points(
        &self,
        glyf_table: glyf::Table,
        coordinates: &[NormalizedCoordinate],
        glyph_id: GlyphId,
    ) -> Option<PhantomPoints> {
        let outline_points = glyf_table.outline_points(glyph_id);
        let mut tuples = VariationTuples::default();
        self.parse_variation_data(glyph_id, coordinates, outline_points, &mut tuples)?;

        // Skip all outline deltas.
        for _ in 0..outline_points {
            tuples.apply_null()?;
        }

        Some(PhantomPoints {
            left: tuples.apply_null()?,
            right: tuples.apply_null()?,
            top: tuples.apply_null()?,
            bottom: tuples.apply_null()?,
        })
    }
}

impl core::fmt::Debug for Table<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Table {{ ... }}")
    }
}

#[allow(clippy::comparison_chain)]
fn outline_var_impl(
    glyf_table: glyf::Table,
    gvar_table: &Table,
    glyph_id: GlyphId,
    data: &[u8],
    coordinates: &[NormalizedCoordinate],
    depth: u8,
    builder: &mut glyf::Builder,
) -> Option<()> {
    if depth >= glyf::MAX_COMPONENTS {
        return None;
    }

    let mut s = Stream::new(data);
    let number_of_contours = s.read::<i16>()?;

    // Skip bbox.
    //
    // In case of a variable font, a bounding box defined in the `glyf` data
    // refers to the default variation values. Which is not what we want.
    // Instead, we have to manually calculate outline's bbox.
    s.advance(8);

    // TODO: This is the most expensive part. Find a way to allocate it only once.
    // `VariationTuples` is a very large struct, so allocate it once.
    let mut tuples = VariationTuples::default();

    if number_of_contours > 0 {
        // Simple glyph.

        let number_of_contours = NonZeroU16::new(number_of_contours as u16)?;
        let mut glyph_points = glyf::parse_simple_outline(s.tail()?, number_of_contours)?;
        let all_glyph_points = glyph_points.clone();
        let points_len = glyph_points.points_left;
        gvar_table.parse_variation_data(glyph_id, coordinates, points_len, &mut tuples)?;

        while let Some(point) = glyph_points.next() {
            let p = tuples.apply(all_glyph_points.clone(), glyph_points.clone(), point)?;
            builder.push_point(p.x, p.y, point.on_curve_point, point.last_point);
        }

        Some(())
    } else if number_of_contours < 0 {
        // Composite glyph.

        // In case of a composite glyph, `gvar` data contains position adjustments
        // for each component.
        // Basically, an additional translation used during transformation.
        // So we have to push zero points manually, instead of parsing the `glyf` data.
        //
        // Details:
        // https://docs.microsoft.com/en-us/typography/opentype/spec/gvar#point-numbers-and-processing-for-composite-glyphs

        let components = glyf::CompositeGlyphIter::new(s.tail()?);
        let components_count = components.clone().count() as u16;
        gvar_table.parse_variation_data(glyph_id, coordinates, components_count, &mut tuples)?;

        for component in components {
            let t = tuples.apply_null()?;

            let mut transform = builder.transform;

            // Variation component offset should be applied only when
            // the ARGS_ARE_XY_VALUES flag is set.
            if component.flags.args_are_xy_values() {
                transform = Transform::combine(transform, Transform::new_translate(t.x, t.y));
            }

            transform = Transform::combine(transform, component.transform);

            let mut b = glyf::Builder::new(transform, builder.bbox, builder.builder);
            if let Some(glyph_data) = glyf_table.get(component.glyph_id) {
                outline_var_impl(
                    glyf_table,
                    gvar_table,
                    component.glyph_id,
                    glyph_data,
                    coordinates,
                    depth + 1,
                    &mut b,
                )?;

                // Take updated bbox.
                builder.bbox = b.bbox;
            }
        }

        Some(())
    } else {
        // An empty glyph.
        None
    }
}

// https://docs.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#tuple-variation-store-header
fn parse_variation_data<'a>(
    coordinates: &[NormalizedCoordinate],
    shared_tuple_records: &LazyArray16<F2DOT14>,
    points_len: u16,
    data: &'a [u8],
    tuples: &mut VariationTuples<'a>,
) -> Option<()> {
    const SHARED_POINT_NUMBERS_FLAG: u16 = 0x8000;
    const COUNT_MASK: u16 = 0x0FFF;

    let mut main_stream = Stream::new(data);
    let tuple_variation_count = main_stream.read::<u16>()?;
    let data_offset = main_stream.read::<Offset16>()?;

    // 'The high 4 bits are flags, and the low 12 bits
    // are the number of tuple variation tables for this glyph.'
    let has_shared_point_numbers = tuple_variation_count & SHARED_POINT_NUMBERS_FLAG != 0;
    let tuple_variation_count = tuple_variation_count & COUNT_MASK;

    // 'The number of tuple variation tables can be any number between 1 and 4095.'
    // No need to check for 4095, because this is 0x0FFF that we masked before.
    if tuple_variation_count == 0 {
        return None;
    }

    // Attempt to reserve space for the tuples we're about to parse.
    // If it fails, bail out.
    if !tuples.reserve(tuple_variation_count) {
        return None;
    }

    // A glyph variation data consists of three parts: header + variation tuples + serialized data.
    // Each tuple has it's own chunk in the serialized data.
    // Because of that, we are using two parsing streams: one for tuples and one for serialized data.
    // So we can parse them in parallel and avoid needless allocations.
    let mut serialized_stream = Stream::new_at(data, data_offset.to_usize())?;

    // All tuples in the variation data can reference the same point numbers,
    // which are defined at the start of the serialized data.
    let mut shared_point_numbers = None;
    if has_shared_point_numbers {
        shared_point_numbers = PackedPointsIter::new(&mut serialized_stream)?;
    }

    parse_variation_tuples(
        tuple_variation_count,
        coordinates,
        shared_tuple_records,
        shared_point_numbers,
        points_len.checked_add(PHANTOM_POINTS_LEN as u16)?,
        main_stream,
        serialized_stream,
        tuples,
    )
}
