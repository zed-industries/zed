//! Operand stack for CFF/CFF2 parsing.

use types::Fixed;

use super::{BlendState, Error};

/// Maximum size of the operand stack.
///
/// "Operators in Top DICT, Font DICTs, Private DICTs and CharStrings may be
/// preceded by up to a maximum of 513 operands."
///
/// <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#table-9-top-dict-operator-entries>
const MAX_STACK: usize = 513;

/// Operand stack for DICTs and charstrings.
///
/// The operand stack can contain either 32-bit integers or 16.16 fixed point
/// values. The type is known when pushing to the stack and the expected type
/// is also known (based on the operator) when reading from the stack, so the
/// conversion is performed on demand at read time.
///
/// Storing the entries as an enum would require 8 bytes each and since these
/// objects are created on the _stack_, we reduce the required size by storing
/// the entries in parallel arrays holding the raw 32-bit value and a flag that
/// tracks which values are fixed point.
pub struct Stack {
    values: [i32; MAX_STACK],
    value_is_fixed: [bool; MAX_STACK],
    top: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            values: [0; MAX_STACK],
            value_is_fixed: [false; MAX_STACK],
            top: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn len(&self) -> usize {
        self.top
    }

    pub fn verify_exact_len(&self, len: usize) -> Result<(), Error> {
        if self.top != len {
            Err(Error::StackUnderflow)
        } else {
            Ok(())
        }
    }

    pub fn verify_at_least_len(&self, len: usize) -> Result<(), Error> {
        if self.top < len {
            Err(Error::StackUnderflow)
        } else {
            Ok(())
        }
    }

    /// Returns true if the number of elements on the stack is odd.
    ///
    /// Used for processing some charstring operators where an odd
    /// count represents the presence of the glyph advance width at the
    /// bottom of the stack.
    pub fn len_is_odd(&self) -> bool {
        self.top & 1 != 0
    }

    pub fn clear(&mut self) {
        self.top = 0;
    }

    /// Reverse the order of all elements on the stack.
    ///
    /// Some charstring operators are simpler to process on a reversed
    /// stack.
    pub fn reverse(&mut self) {
        self.values[..self.top].reverse();
        self.value_is_fixed[..self.top].reverse();
    }

    pub fn push(&mut self, number: impl Into<Number>) -> Result<(), Error> {
        match number.into() {
            Number::I32(value) => self.push_impl(value, false),
            Number::Fixed(value) => self.push_impl(value.to_bits(), true),
        }
    }

    /// Returns the 32-bit integer at the given index on the stack.
    ///
    /// Will return an error if the value at that index was not pushed as an
    /// integer.
    pub fn get_i32(&self, index: usize) -> Result<i32, Error> {
        // FreeType just returns 0 for OOB access
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psstack.c#L143>
        let Some(value) = self.values.get(index).copied() else {
            return Ok(0);
        };
        if self.value_is_fixed[index] {
            // FreeType returns an error here rather than converting
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psstack.c#L145>
            Err(Error::ExpectedI32StackEntry(index))
        } else {
            Ok(value)
        }
    }

    /// Returns the 16.16 fixed point value at the given index on the stack.
    ///
    /// If the value was pushed as an integer, it will be automatically
    /// converted to 16.16 fixed point.
    pub fn get_fixed(&self, index: usize) -> Result<Fixed, Error> {
        // FreeType just returns 0 for OOB access
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psstack.c#L193>
        let Some(value) = self.values.get(index).copied() else {
            return Ok(Fixed::ZERO);
        };
        Ok(if self.value_is_fixed[index] {
            Fixed::from_bits(value)
        } else {
            Fixed::from_i32(value)
        })
    }

    /// Pops a 32-bit integer from the top of stack.
    ///
    /// Will return an error if the top value on the stack was not pushed as an
    /// integer.
    pub fn pop_i32(&mut self) -> Result<i32, Error> {
        let i = self.pop()?;
        self.get_i32(i)
    }

    /// Pops a 16.16 fixed point value from the top of the stack.
    ///
    /// If the value was pushed as an integer, it will be automatically
    /// converted to 16.16 fixed point.
    pub fn pop_fixed(&mut self) -> Result<Fixed, Error> {
        let i = self.pop()?;
        self.get_fixed(i)
    }

    /// Returns an iterator yielding all elements on the stack
    /// as 16.16 fixed point values.
    ///
    /// Used to read array style DICT entries such as blue values,
    /// font matrix and font bounding box.
    pub fn fixed_values(&self) -> impl Iterator<Item = Fixed> + '_ {
        self.values[..self.top]
            .iter()
            .zip(&self.value_is_fixed)
            .map(|(value, is_real)| {
                if *is_real {
                    Fixed::from_bits(*value)
                } else {
                    Fixed::from_i32(*value)
                }
            })
    }

    /// Returns an array of `N` 16.16 fixed point values starting at
    /// `first_index`.
    pub fn fixed_array<const N: usize>(&self, first_index: usize) -> Result<[Fixed; N], Error> {
        let mut result = [Fixed::ZERO; N];
        let end = first_index + N;
        // FreeType doesn't have an equivalent to this function but always
        // returns 0 on failure so we take the maximum valid range and
        // pad the remainder with zeros
        let range = first_index.min(self.top)..end.min(self.top);
        for ((src, is_fixed), dest) in self.values[range.clone()]
            .iter()
            .zip(&self.value_is_fixed[range])
            .zip(&mut result)
        {
            let value = if *is_fixed {
                Fixed::from_bits(*src)
            } else {
                Fixed::from_i32(*src)
            };
            *dest = value;
        }
        Ok(result)
    }

    /// Returns an iterator yielding all elements on the stack as number
    /// values.
    ///
    /// This is useful for capturing the current state of the stack.
    pub fn number_values(&self) -> impl Iterator<Item = Number> + '_ {
        self.values[..self.top]
            .iter()
            .zip(&self.value_is_fixed)
            .map(|(value, is_fixed)| Number::from_stack(*value, *is_fixed))
    }

    /// Apply a prefix sum to decode delta-encoded numbers.
    ///
    /// "The second and subsequent numbers in a delta are encoded as the
    /// difference between successive values."
    ///
    /// Roughly equivalent to the FreeType logic at
    /// <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/cff/cffparse.c#L1431>
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#table-6-operand-types>
    pub fn apply_delta_prefix_sum(&mut self) {
        if self.top > 1 {
            let mut sum = Fixed::ZERO;
            for (value, is_fixed) in self.values[..self.top]
                .iter_mut()
                .zip(&mut self.value_is_fixed)
            {
                let fixed_value = if *is_fixed {
                    Fixed::from_bits(*value)
                } else {
                    Fixed::from_i32(*value)
                };
                // See <https://github.com/googlefonts/fontations/issues/1193>
                // The "DIN Alternate" font contains incorrect blue values
                // that cause an overflow in this computation. FreeType does
                // not use checked arithmetic so we need to explicitly use
                // wrapping behavior to produce matching outlines.
                sum = sum.wrapping_add(fixed_value);
                *value = sum.to_bits();
                *is_fixed = true;
            }
        }
    }

    /// Apply the `blend` operator.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2charstr#syntax-for-font-variations-support-operators>
    #[inline(never)]
    pub fn apply_blend(&mut self, blend_state: &BlendState) -> Result<(), Error> {
        // When the blend operator is invoked, the stack will contain a set
        // of target values, followed by sets of deltas for those values for
        // each variation region, followed by the count of target values.
        //
        // For example, if we're blending two target values across three
        // variation regions, the stack would be setup as follows (parentheses
        // added to signify grouping of deltas):
        //
        // value_0 value_1 (delta_0_0 delta_0_1 delta_0_2) (delta_1_0 delta_1_1 delta_1_2) 2
        //
        // where delta_i_j represents the delta for value i and region j.
        //
        // We compute the scalars for each region, multiply them by the
        // associated deltas and add the result to the respective target value.
        // Then the stack is popped so only the final target values remain.
        let target_value_count = self.pop_i32()? as usize;
        if target_value_count > self.top {
            return Err(Error::StackUnderflow);
        }
        let region_count = blend_state.region_count()?;
        // We expect at least `target_value_count * (region_count + 1)`
        // elements on the stack.
        let operand_count = target_value_count * (region_count + 1);
        if self.len() < operand_count {
            return Err(Error::StackUnderflow);
        }
        // The stack may contain more elements than necessary, so keep track of
        // our active range.
        let start = self.len() - operand_count;
        let end = start + operand_count;
        // For simplicity, convert all elements to fixed up front.
        for (value, is_fixed) in self.values[start..end]
            .iter_mut()
            .zip(&mut self.value_is_fixed[start..])
        {
            if !*is_fixed {
                *value = Fixed::from_i32(*value).to_bits();
                *is_fixed = true;
            }
        }
        let (values, deltas) = self.values[start..].split_at_mut(target_value_count);
        // Note: we specifically loop over scalars in the outer loop to avoid
        // computing them more than once in the case that we overflow our
        // precomputed cache.
        for (region_ix, maybe_scalar) in blend_state.scalars()?.enumerate() {
            let scalar = maybe_scalar?;
            // We could omit these in `BlendState::scalars()` but that would
            // significantly reduce the clarity of the already complex
            // chained iterator code there. Do the simple thing here instead.
            if scalar == Fixed::ZERO {
                continue;
            }
            for (value_ix, value) in values.iter_mut().enumerate() {
                let delta_ix = (region_count * value_ix) + region_ix;
                let delta = Fixed::from_bits(deltas[delta_ix]);
                *value = (Fixed::from_bits(*value).wrapping_add(delta * scalar)).to_bits();
            }
        }
        self.top = start + target_value_count;
        Ok(())
    }

    fn push_impl(&mut self, value: i32, is_fixed: bool) -> Result<(), Error> {
        if self.top == MAX_STACK {
            return Err(Error::StackOverflow);
        }
        self.values[self.top] = value;
        self.value_is_fixed[self.top] = is_fixed;
        self.top += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<usize, Error> {
        if self.top > 0 {
            self.top -= 1;
            Ok(self.top)
        } else {
            Ok(0)
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

/// Either a signed 32-bit integer or a 16.16 fixed point number.
///
/// This represents the CFF "number" operand type.
/// See "Table 6 Operand Types" at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf>
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Number {
    I32(i32),
    Fixed(Fixed),
}

impl Number {
    fn from_stack(raw: i32, is_fixed: bool) -> Self {
        if is_fixed {
            Self::Fixed(Fixed::from_bits(raw))
        } else {
            Self::I32(raw)
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<Fixed> for Number {
    fn from(value: Fixed) -> Self {
        Self::Fixed(value)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::I32(value) => value.fmt(f),
            Self::Fixed(value) => value.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use types::{F2Dot14, Fixed};

    use super::Stack;
    use crate::{
        tables::{postscript::BlendState, variations::ItemVariationStore},
        FontData, FontRead,
    };

    #[test]
    fn push_pop() {
        let mut stack = Stack::new();
        stack.push(20).unwrap();
        stack.push(Fixed::from_f64(42.42)).unwrap();
        assert!(!stack.len_is_odd());
        stack.verify_exact_len(2).unwrap();
        stack.verify_at_least_len(2).unwrap();
        assert_eq!(stack.pop_fixed().unwrap(), Fixed::from_f64(42.42));
        assert_eq!(stack.pop_i32().unwrap(), 20);
    }

    #[test]
    fn push_fixed_pop_i32() {
        let mut stack = Stack::new();
        stack.push(Fixed::from_f64(42.42)).unwrap();
        assert!(stack.pop_i32().is_err());
    }

    #[test]
    fn push_i32_pop_fixed() {
        let mut stack = Stack::new();
        stack.push(123).unwrap();
        assert_eq!(stack.pop_fixed().unwrap(), Fixed::from_f64(123.0));
    }

    #[test]
    fn reverse() {
        let mut stack = Stack::new();
        stack.push(Fixed::from_f64(1.5)).unwrap();
        stack.push(42).unwrap();
        stack.push(Fixed::from_f64(4.2)).unwrap();
        stack.reverse();
        assert_eq!(stack.pop_fixed().unwrap(), Fixed::from_f64(1.5));
        assert_eq!(stack.pop_i32().unwrap(), 42);
        assert_eq!(stack.pop_fixed().unwrap(), Fixed::from_f64(4.2));
    }

    #[test]
    fn delta_prefix_sum() {
        let mut stack = Stack::new();
        stack.push(Fixed::from_f64(1.5)).unwrap();
        stack.push(42).unwrap();
        stack.push(Fixed::from_f64(4.2)).unwrap();
        stack.apply_delta_prefix_sum();
        assert!(stack.len_is_odd());
        let values: Vec<_> = stack.fixed_values().collect();
        let expected = &[
            Fixed::from_f64(1.5),
            Fixed::from_f64(43.5),
            Fixed::from_f64(47.69999694824219),
        ];
        assert_eq!(&values, expected);
    }

    #[test]
    fn blend() {
        let ivs_data = &font_test_data::cff2::EXAMPLE[18..];
        let ivs = ItemVariationStore::read(FontData::new(ivs_data)).unwrap();
        // This coordinate will generate scalars [0.5, 0.5]
        let coords = &[F2Dot14::from_f32(-0.75)];
        let blend_state = BlendState::new(ivs, coords, 0).unwrap();
        let mut stack = Stack::new();
        // Push our target values
        stack.push(10).unwrap();
        stack.push(20).unwrap();
        // Push deltas for 2 regions for the first value
        stack.push(4).unwrap();
        stack.push(-8).unwrap();
        // Push deltas for 2 regions for the second value
        stack.push(-60).unwrap();
        stack.push(2).unwrap();
        // Push target value count
        stack.push(2).unwrap();
        stack.apply_blend(&blend_state).unwrap();
        let result: Vec<_> = stack.fixed_values().collect();
        // Expected values:
        // 0: 10 + (4 * 0.5) + (-8 * 0.5) = 8
        // 1: 20 + (-60 * 0.5) + (2 * 0.5) = -9
        let expected = &[Fixed::from_f64(8.0), Fixed::from_f64(-9.0)];
        assert_eq!(&result, expected);
    }

    #[test]
    fn invalid_access_yields_zero() {
        let mut stack = Stack::new();
        assert_eq!(stack.pop_i32().unwrap(), 0);
        assert_eq!(stack.pop_fixed().unwrap(), Fixed::ZERO);
        assert_eq!(stack.get_i32(10).unwrap(), 0);
        assert_eq!(stack.get_fixed(10).unwrap(), Fixed::ZERO);
        // In this case, we get the first valid value followed by the
        // remainder of the requested array size padded with zeros
        stack.push(5).unwrap();
        assert_eq!(
            stack.fixed_array::<3>(0).unwrap(),
            [Fixed::from_i32(5), Fixed::ZERO, Fixed::ZERO]
        );
    }
}
