//! Parsing for PostScript charstrings.

use super::{BlendState, Error, Index, Stack};
use crate::{
    tables::{cff::Cff, postscript::StringId},
    types::{Fixed, Point},
    Cursor, FontData, FontRead,
};

/// Maximum nesting depth for subroutine calls.
///
/// See "Appendix B Type 2 Charstring Implementation Limits" at
/// <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=33>
pub const NESTING_DEPTH_LIMIT: u32 = 10;

/// Trait for processing commands resulting from charstring evaluation.
///
/// During processing, the path construction operators (see "4.1 Path
/// Construction Operators" at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=15>)
/// are simplified into the basic move, line, curve and close commands.
///
/// This also has optional callbacks for processing hint operators. See "4.3
/// Hint Operators" at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=21>
/// for more detail.
#[allow(unused_variables)]
pub trait CommandSink {
    // Path construction operators.
    fn move_to(&mut self, x: Fixed, y: Fixed);
    fn line_to(&mut self, x: Fixed, y: Fixed);
    fn curve_to(&mut self, cx0: Fixed, cy0: Fixed, cx1: Fixed, cy1: Fixed, x: Fixed, y: Fixed);
    fn close(&mut self);
    // Hint operators.
    /// Horizontal stem hint at `y` with height `dy`.
    fn hstem(&mut self, y: Fixed, dy: Fixed) {}
    /// Vertical stem hint at `x` with width `dx`.
    fn vstem(&mut self, x: Fixed, dx: Fixed) {}
    /// Bitmask defining the hints that should be made active for the
    /// commands that follow.
    fn hint_mask(&mut self, mask: &[u8]) {}
    /// Bitmask defining the counter hints that should be made active for the
    /// commands that follow.
    fn counter_mask(&mut self, mask: &[u8]) {}
    /// Clear accumulated stem hints and all data derived from them.
    fn clear_hints(&mut self) {}
}

/// Evaluates the given charstring and emits the resulting commands to the
/// specified sink.
///
/// If the Private DICT associated with this charstring contains local
/// subroutines, then the `subrs` index must be provided, otherwise
/// `Error::MissingSubroutines` will be returned if a callsubr operator
/// is present.
///
/// If evaluating a CFF2 charstring and the top-level table contains an
/// item variation store, then `blend_state` must be provided, otherwise
/// `Error::MissingBlendState` will be returned if a blend operator is
/// present.
pub fn evaluate(
    cff_data: &[u8],
    charstrings: Index,
    global_subrs: Index,
    subrs: Option<Index>,
    blend_state: Option<BlendState>,
    charstring_data: &[u8],
    sink: &mut impl CommandSink,
) -> Result<(), Error> {
    let mut evaluator = Evaluator::new(
        cff_data,
        charstrings,
        global_subrs,
        subrs,
        blend_state,
        sink,
    );
    evaluator.evaluate(charstring_data, 0)?;
    Ok(())
}

/// Transient state for evaluating a charstring and handling recursive
/// subroutine calls.
struct Evaluator<'a, S> {
    cff_data: &'a [u8],
    charstrings: Index<'a>,
    global_subrs: Index<'a>,
    subrs: Option<Index<'a>>,
    blend_state: Option<BlendState<'a>>,
    sink: &'a mut S,
    is_open: bool,
    have_read_width: bool,
    stem_count: usize,
    x: Fixed,
    y: Fixed,
    stack: Stack,
    stack_ix: usize,
}

impl<'a, S> Evaluator<'a, S>
where
    S: CommandSink,
{
    fn new(
        cff_data: &'a [u8],
        charstrings: Index<'a>,
        global_subrs: Index<'a>,
        subrs: Option<Index<'a>>,
        blend_state: Option<BlendState<'a>>,
        sink: &'a mut S,
    ) -> Self {
        Self {
            cff_data,
            charstrings,
            global_subrs,
            subrs,
            blend_state,
            sink,
            is_open: false,
            have_read_width: false,
            stem_count: 0,
            stack: Stack::new(),
            x: Fixed::ZERO,
            y: Fixed::ZERO,
            stack_ix: 0,
        }
    }

    fn evaluate(&mut self, charstring_data: &[u8], nesting_depth: u32) -> Result<(), Error> {
        if nesting_depth > NESTING_DEPTH_LIMIT {
            return Err(Error::CharstringNestingDepthLimitExceeded);
        }
        let mut cursor = crate::FontData::new(charstring_data).cursor();
        while cursor.remaining_bytes() != 0 {
            let b0 = cursor.read::<u8>()?;
            match b0 {
                // See "3.2 Charstring Number Encoding" <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=12>
                //
                // Push an integer to the stack
                28 | 32..=254 => {
                    self.stack.push(super::dict::parse_int(&mut cursor, b0)?)?;
                }
                // Push a fixed point value to the stack
                255 => {
                    let num = Fixed::from_bits(cursor.read::<i32>()?);
                    self.stack.push(num)?;
                }
                _ => {
                    // FreeType ignores reserved (unknown) operators.
                    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L703>
                    // and fontations issue <https://github.com/googlefonts/fontations/issues/1680>
                    if let Ok(operator) = Operator::read(&mut cursor, b0) {
                        if !self.evaluate_operator(operator, &mut cursor, nesting_depth)? {
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Evaluates a single charstring operator.
    ///
    /// Returns `Ok(true)` if evaluation should continue.
    fn evaluate_operator(
        &mut self,
        operator: Operator,
        cursor: &mut Cursor,
        nesting_depth: u32,
    ) -> Result<bool, Error> {
        use Operator::*;
        use PointMode::*;
        match operator {
            // The following "flex" operators are intended to emit
            // either two curves or a straight line depending on
            // a "flex depth" parameter and the distance from the
            // joining point to the chord connecting the two
            // end points. In practice, we just emit the two curves,
            // following FreeType:
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L335>
            //
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=18>
            Flex => {
                self.emit_curves([DxDy; 6])?;
                self.reset_stack();
            }
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=19>
            HFlex => {
                self.emit_curves([DxY, DxDy, DxY, DxY, DxInitialY, DxY])?;
                self.reset_stack();
            }
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=19>
            HFlex1 => {
                self.emit_curves([DxDy, DxDy, DxY, DxY, DxDy, DxInitialY])?;
                self.reset_stack();
            }
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=20>
            Flex1 => {
                self.emit_curves([DxDy, DxDy, DxDy, DxDy, DxDy, DLargerCoordDist])?;
                self.reset_stack();
            }
            // Set the variation store index
            // <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2charstr#syntax-for-font-variations-support-operators>
            VariationStoreIndex => {
                let blend_state = self.blend_state.as_mut().ok_or(Error::MissingBlendState)?;
                let store_index = self.stack.pop_i32()? as u16;
                blend_state.set_store_index(store_index)?;
            }
            // Apply blending to the current operand stack
            // <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2charstr#syntax-for-font-variations-support-operators>
            Blend => {
                let blend_state = self.blend_state.as_ref().ok_or(Error::MissingBlendState)?;
                self.stack.apply_blend(blend_state)?;
            }
            // Return from the current subroutine
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=29>
            Return => {
                return Ok(false);
            }
            // End the current charstring
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=21>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L2463>
            EndChar => {
                if self.stack.len() == 4 || self.stack.len() == 5 && !self.have_read_width {
                    self.handle_seac(nesting_depth)?;
                } else if !self.stack.is_empty() && !self.have_read_width {
                    self.have_read_width = true;
                    self.stack.clear();
                }
                if self.is_open {
                    self.is_open = false;
                    self.sink.close();
                }
                return Ok(false);
            }
            // Emits a sequence of stem hints
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=21>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L777>
            HStem | VStem | HStemHm | VStemHm => {
                let mut i = 0;
                let len = if self.stack.len_is_odd() && !self.have_read_width {
                    self.have_read_width = true;
                    i = 1;
                    self.stack.len() - 1
                } else {
                    self.stack.len()
                };
                let is_horizontal = matches!(operator, HStem | HStemHm);
                let mut u = Fixed::ZERO;
                while i < self.stack.len() {
                    let args = self.stack.fixed_array::<2>(i)?;
                    u += args[0];
                    let w = args[1];
                    let v = u.wrapping_add(w);
                    if is_horizontal {
                        self.sink.hstem(u, v);
                    } else {
                        self.sink.vstem(u, v);
                    }
                    u = v;
                    i += 2;
                }
                self.stem_count += len / 2;
                self.reset_stack();
            }
            // Applies a hint or counter mask.
            // If there are arguments on the stack, this is also an
            // implied series of VSTEMHM operators.
            // Hint and counter masks are bitstrings that determine
            // the currently active set of hints.
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=24>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L2580>
            HintMask | CntrMask => {
                let mut i = 0;
                let len = if self.stack.len_is_odd() && !self.have_read_width {
                    self.have_read_width = true;
                    i = 1;
                    self.stack.len() - 1
                } else {
                    self.stack.len()
                };
                let mut u = Fixed::ZERO;
                while i < self.stack.len() {
                    let args = self.stack.fixed_array::<2>(i)?;
                    u += args[0];
                    let w = args[1];
                    let v = u + w;
                    self.sink.vstem(u, v);
                    u = v;
                    i += 2;
                }
                self.stem_count += len / 2;
                let count = self.stem_count.div_ceil(8);
                let mask = cursor.read_array::<u8>(count)?;
                if operator == HintMask {
                    self.sink.hint_mask(mask);
                } else {
                    self.sink.counter_mask(mask);
                }
                self.reset_stack();
            }
            // Starts a new subpath
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=16>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L2653>
            RMoveTo => {
                let mut i = 0;
                if self.stack.len() == 3 && !self.have_read_width {
                    self.have_read_width = true;
                    i = 1;
                }
                if !self.is_open {
                    self.is_open = true;
                } else {
                    self.sink.close();
                }
                let [dx, dy] = self.stack.fixed_array::<2>(i)?;
                self.x += dx;
                self.y += dy;
                self.sink.move_to(self.x, self.y);
                self.reset_stack();
            }
            // Starts a new subpath by moving the current point in the
            // horizontal or vertical direction
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=16>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L839>
            HMoveTo | VMoveTo => {
                let mut i = 0;
                if self.stack.len() == 2 && !self.have_read_width {
                    self.have_read_width = true;
                    i = 1;
                }
                if !self.is_open {
                    self.is_open = true;
                } else {
                    self.sink.close();
                }
                let delta = self.stack.get_fixed(i)?;
                if operator == HMoveTo {
                    self.x += delta;
                } else {
                    self.y += delta;
                }
                self.sink.move_to(self.x, self.y);
                self.reset_stack();
            }
            // Emits a sequence of lines
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=16>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L863>
            RLineTo => {
                let mut i = 0;
                while i < self.stack.len() {
                    let [dx, dy] = self.stack.fixed_array::<2>(i)?;
                    self.x += dx;
                    self.y += dy;
                    self.sink.line_to(self.x, self.y);
                    i += 2;
                }
                self.reset_stack();
            }
            // Emits a sequence of alternating horizontal and vertical
            // lines
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=16>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L885>
            HLineTo | VLineTo => {
                let mut is_x = operator == HLineTo;
                for i in 0..self.stack.len() {
                    let delta = self.stack.get_fixed(i)?;
                    if is_x {
                        self.x += delta;
                    } else {
                        self.y += delta;
                    }
                    is_x = !is_x;
                    self.sink.line_to(self.x, self.y);
                }
                self.reset_stack();
            }
            // Emits curves that start and end horizontal, unless
            // the stack count is odd, in which case the first
            // curve may start with a vertical tangent
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=17>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L2789>
            HhCurveTo => {
                if self.stack.len_is_odd() {
                    self.y += self.stack.get_fixed(0)?;
                    self.stack_ix = 1;
                }
                // We need at least 4 coordinates to emit these curves
                while self.coords_remaining() >= 4 {
                    self.emit_curves([DxY, DxDy, DxY])?;
                }
                self.reset_stack();
            }
            // Alternates between curves with horizontal and vertical
            // tangents
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=17>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L2834>
            HvCurveTo | VhCurveTo => {
                let count1 = self.stack.len();
                let count = count1 & !2;
                let mut is_horizontal = operator == HvCurveTo;
                self.stack_ix = count1 - count;
                while self.stack_ix < count {
                    let do_last_delta = count - self.stack_ix == 5;
                    if is_horizontal {
                        self.emit_curves([DxY, DxDy, MaybeDxDy(do_last_delta)])?;
                    } else {
                        self.emit_curves([XDy, DxDy, DxMaybeDy(do_last_delta)])?;
                    }
                    is_horizontal = !is_horizontal;
                }
                self.reset_stack();
            }
            // Emits a sequence of curves possibly followed by a line
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=17>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L915>
            RrCurveTo | RCurveLine => {
                while self.coords_remaining() >= 6 {
                    self.emit_curves([DxDy; 3])?;
                }
                if operator == RCurveLine {
                    let [dx, dy] = self.stack.fixed_array::<2>(self.stack_ix)?;
                    self.x += dx;
                    self.y += dy;
                    self.sink.line_to(self.x, self.y);
                }
                self.reset_stack();
            }
            // Emits a sequence of lines followed by a curve
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=18>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L2702>
            RLineCurve => {
                while self.coords_remaining() > 6 {
                    let [dx, dy] = self.stack.fixed_array::<2>(self.stack_ix)?;
                    self.x += dx;
                    self.y += dy;
                    self.sink.line_to(self.x, self.y);
                    self.stack_ix += 2;
                }
                self.emit_curves([DxDy; 3])?;
                self.reset_stack();
            }
            // Emits curves that start and end vertical, unless
            // the stack count is odd, in which case the first
            // curve may start with a horizontal tangent
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=18>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L2744>
            VvCurveTo => {
                if self.stack.len_is_odd() {
                    self.x += self.stack.get_fixed(0)?;
                    self.stack_ix = 1;
                }
                while self.coords_remaining() > 0 {
                    self.emit_curves([XDy, DxDy, XDy])?;
                }
                self.reset_stack();
            }
            // Call local or global subroutine
            // Spec: <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=29>
            // FT: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L972>
            CallSubr | CallGsubr => {
                let subrs_index = if operator == CallSubr {
                    self.subrs.as_ref().ok_or(Error::MissingSubroutines)?
                } else {
                    &self.global_subrs
                };
                let biased_index = (self.stack.pop_i32()? + subrs_index.subr_bias()) as usize;
                let subr_charstring_data = subrs_index.get(biased_index)?;
                self.evaluate(subr_charstring_data, nesting_depth + 1)?;
            }
        }
        Ok(true)
    }

    /// See `endchar` in Appendix C at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf#page=35>
    fn handle_seac(&mut self, nesting_depth: u32) -> Result<(), Error> {
        // handle implied seac operator
        let cff = Cff::read(FontData::new(self.cff_data))?;
        let charset = cff.charset(0)?.ok_or(Error::MissingCharset)?;
        let seac_to_gid = |code: i32| {
            let code: u8 = code.try_into().ok()?;
            let sid = *super::encoding::STANDARD_ENCODING.get(code as usize)?;
            charset.glyph_id(StringId::new(sid as u16)).ok()
        };
        let accent_code = self.stack.pop_i32()?;
        let accent_gid = seac_to_gid(accent_code).ok_or(Error::InvalidSeacCode(accent_code))?;
        let base_code = self.stack.pop_i32()?;
        let base_gid = seac_to_gid(base_code).ok_or(Error::InvalidSeacCode(base_code))?;
        let dy = self.stack.pop_fixed()?;
        let dx = self.stack.pop_fixed()?;
        if !self.stack.is_empty() && !self.have_read_width {
            self.stack.pop_i32()?;
            self.have_read_width = true;
        }
        // The accent must be evaluated first to match FreeType but the
        // base should be placed at the current position, so save it
        let x = self.x;
        let y = self.y;
        self.x = dx;
        self.y = dy;
        let accent_charstring = self.charstrings.get(accent_gid.to_u32() as usize)?;
        // FreeType calls cf2_interpT2CharString for each component
        // which uses a fresh set of stem hints. Since our hinter is in
        // a separate crate, we signal this through the sink. Also
        // reset our own stem count so we read the correct number of
        // bytes for each hint mask instruction.
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L1443>
        // and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psintrp.c#L540>
        self.sink.clear_hints();
        self.stem_count = 0;
        self.evaluate(accent_charstring, nesting_depth + 1)?;
        self.x = x;
        self.y = y;
        let base_charstring = self.charstrings.get(base_gid.to_u32() as usize)?;
        self.sink.clear_hints();
        self.stem_count = 0;
        self.evaluate(base_charstring, nesting_depth + 1)
    }

    fn coords_remaining(&self) -> usize {
        // This is overly defensive to avoid overflow but in the case of
        // broken fonts, just return 0 when stack_ix > stack_len to prevent
        // potential runaway while loops in the evaluator if this wraps
        self.stack.len().saturating_sub(self.stack_ix)
    }

    fn emit_curves<const N: usize>(&mut self, modes: [PointMode; N]) -> Result<(), Error> {
        use PointMode::*;
        let initial_x = self.x;
        let initial_y = self.y;
        let mut count = 0;
        let mut points = [Point::default(); 2];
        for mode in modes {
            let stack_used = match mode {
                DxDy => {
                    self.x += self.stack.get_fixed(self.stack_ix)?;
                    self.y += self.stack.get_fixed(self.stack_ix + 1)?;
                    2
                }
                XDy => {
                    self.y += self.stack.get_fixed(self.stack_ix)?;
                    1
                }
                DxY => {
                    self.x += self.stack.get_fixed(self.stack_ix)?;
                    1
                }
                DxInitialY => {
                    self.x += self.stack.get_fixed(self.stack_ix)?;
                    self.y = initial_y;
                    1
                }
                // Emits a delta for the coordinate with the larger distance
                // from the original value. Sets the other coordinate to the
                // original value.
                DLargerCoordDist => {
                    let delta = self.stack.get_fixed(self.stack_ix)?;
                    if (self.x - initial_x).abs() > (self.y - initial_y).abs() {
                        self.x += delta;
                        self.y = initial_y;
                    } else {
                        self.y += delta;
                        self.x = initial_x;
                    }
                    1
                }
                // Apply delta to y if `do_dy` is true.
                DxMaybeDy(do_dy) => {
                    self.x += self.stack.get_fixed(self.stack_ix)?;
                    if do_dy {
                        self.y += self.stack.get_fixed(self.stack_ix + 1)?;
                        2
                    } else {
                        1
                    }
                }
                // Apply delta to x if `do_dx` is true.
                MaybeDxDy(do_dx) => {
                    self.y += self.stack.get_fixed(self.stack_ix)?;
                    if do_dx {
                        self.x += self.stack.get_fixed(self.stack_ix + 1)?;
                        2
                    } else {
                        1
                    }
                }
            };
            self.stack_ix += stack_used;
            if count == 2 {
                self.sink.curve_to(
                    points[0].x,
                    points[0].y,
                    points[1].x,
                    points[1].y,
                    self.x,
                    self.y,
                );
                count = 0;
            } else {
                points[count] = Point::new(self.x, self.y);
                count += 1;
            }
        }
        Ok(())
    }

    fn reset_stack(&mut self) {
        self.stack.clear();
        self.stack_ix = 0;
    }
}

/// Specifies how point coordinates for a curve are computed.
#[derive(Copy, Clone)]
enum PointMode {
    DxDy,
    XDy,
    DxY,
    DxInitialY,
    DLargerCoordDist,
    DxMaybeDy(bool),
    MaybeDxDy(bool),
}

/// PostScript charstring operator.
///
/// See <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2charstr#appendix-a-cff2-charstring-command-codes>
// TODO: This is currently missing legacy math and logical operators.
// fonttools doesn't even implement these: <https://github.com/fonttools/fonttools/blob/65598197c8afd415781f6667a7fb647c2c987fff/Lib/fontTools/misc/psCharStrings.py#L409>
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Operator {
    HStem,
    VStem,
    VMoveTo,
    RLineTo,
    HLineTo,
    VLineTo,
    RrCurveTo,
    CallSubr,
    Return,
    EndChar,
    VariationStoreIndex,
    Blend,
    HStemHm,
    HintMask,
    CntrMask,
    RMoveTo,
    HMoveTo,
    VStemHm,
    RCurveLine,
    RLineCurve,
    VvCurveTo,
    HhCurveTo,
    CallGsubr,
    VhCurveTo,
    HvCurveTo,
    HFlex,
    Flex,
    HFlex1,
    Flex1,
}

impl Operator {
    fn read(cursor: &mut Cursor, b0: u8) -> Result<Self, Error> {
        // Escape opcode for accessing two byte operators
        const ESCAPE: u8 = 12;
        let (opcode, operator) = if b0 == ESCAPE {
            let b1 = cursor.read::<u8>()?;
            (b1, Self::from_two_byte_opcode(b1))
        } else {
            (b0, Self::from_opcode(b0))
        };
        operator.ok_or(Error::InvalidCharstringOperator(opcode))
    }

    /// Creates an operator from the given opcode.
    fn from_opcode(opcode: u8) -> Option<Self> {
        use Operator::*;
        Some(match opcode {
            1 => HStem,
            3 => VStem,
            4 => VMoveTo,
            5 => RLineTo,
            6 => HLineTo,
            7 => VLineTo,
            8 => RrCurveTo,
            10 => CallSubr,
            11 => Return,
            14 => EndChar,
            15 => VariationStoreIndex,
            16 => Blend,
            18 => HStemHm,
            19 => HintMask,
            20 => CntrMask,
            21 => RMoveTo,
            22 => HMoveTo,
            23 => VStemHm,
            24 => RCurveLine,
            25 => RLineCurve,
            26 => VvCurveTo,
            27 => HhCurveTo,
            29 => CallGsubr,
            30 => VhCurveTo,
            31 => HvCurveTo,
            _ => return None,
        })
    }

    /// Creates an operator from the given extended opcode.
    ///
    /// These are preceded by a byte containing the escape value of 12.
    pub fn from_two_byte_opcode(opcode: u8) -> Option<Self> {
        use Operator::*;
        Some(match opcode {
            34 => HFlex,
            35 => Flex,
            36 => HFlex1,
            37 => Flex1,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tables::variations::ItemVariationStore, types::F2Dot14, FontData, FontRead};

    #[derive(Copy, Clone, PartialEq, Debug)]
    #[allow(clippy::enum_variant_names)]
    enum Command {
        MoveTo(Fixed, Fixed),
        LineTo(Fixed, Fixed),
        CurveTo(Fixed, Fixed, Fixed, Fixed, Fixed, Fixed),
    }

    #[derive(PartialEq, Default, Debug)]
    struct CaptureCommandSink(Vec<Command>);

    impl CommandSink for CaptureCommandSink {
        fn move_to(&mut self, x: Fixed, y: Fixed) {
            self.0.push(Command::MoveTo(x, y))
        }

        fn line_to(&mut self, x: Fixed, y: Fixed) {
            self.0.push(Command::LineTo(x, y))
        }

        fn curve_to(&mut self, cx0: Fixed, cy0: Fixed, cx1: Fixed, cy1: Fixed, x: Fixed, y: Fixed) {
            self.0.push(Command::CurveTo(cx0, cy0, cx1, cy1, x, y))
        }

        fn close(&mut self) {
            // For testing purposes, replace the close command
            // with a line to the most recent move or (0, 0)
            // if none exists
            let mut last_move = [Fixed::ZERO; 2];
            for command in self.0.iter().rev() {
                if let Command::MoveTo(x, y) = command {
                    last_move = [*x, *y];
                    break;
                }
            }
            self.0.push(Command::LineTo(last_move[0], last_move[1]));
        }
    }

    #[test]
    fn cff2_example_subr() {
        use Command::*;
        let charstring = &font_test_data::cff2::EXAMPLE[0xc8..=0xe1];
        let empty_index_bytes = [0u8; 8];
        let store =
            ItemVariationStore::read(FontData::new(&font_test_data::cff2::EXAMPLE[18..])).unwrap();
        let global_subrs = Index::new(&empty_index_bytes, true).unwrap();
        let coords = &[F2Dot14::from_f32(0.0)];
        let blend_state = BlendState::new(store, coords, 0).unwrap();
        let mut commands = CaptureCommandSink::default();
        evaluate(
            &[],
            Index::Empty,
            global_subrs,
            None,
            Some(blend_state),
            charstring,
            &mut commands,
        )
        .unwrap();
        // 50 50 100 1 blend 0 rmoveto
        // 500 -100 -200 1 blend hlineto
        // 500 vlineto
        // -500 100 200 1 blend hlineto
        //
        // applying blends at default location results in:
        // 50 0 rmoveto
        // 500 hlineto
        // 500 vlineto
        // -500 hlineto
        //
        // applying relative operators:
        // 50 0 moveto
        // 550 0 lineto
        // 550 500 lineto
        // 50 500 lineto
        let expected = &[
            MoveTo(Fixed::from_f64(50.0), Fixed::ZERO),
            LineTo(Fixed::from_f64(550.0), Fixed::ZERO),
            LineTo(Fixed::from_f64(550.0), Fixed::from_f64(500.0)),
            LineTo(Fixed::from_f64(50.0), Fixed::from_f64(500.0)),
        ];
        assert_eq!(&commands.0, expected);
    }

    #[test]
    fn all_path_ops() {
        // This charstring was manually constructed in
        // font-test-data/test_data/ttx/charstring_path_ops.ttx
        //
        // The encoded version was extracted from the font and inlined below
        // for simplicity.
        //
        // The geometry is arbitrary but includes the full set of path
        // construction operators:
        // --------------------------------------------------------------------
        // -137 -632 rmoveto
        // 34 -5 20 -6 rlineto
        // 1 2 3 hlineto
        // -179 -10 3 vlineto
        // -30 15 22 8 -50 26 -14 -42 -41 19 -15 25 rrcurveto
        // -30 15 22 8 hhcurveto
        // 8 -30 15 22 8 hhcurveto
        // 24 20 15 41 42 -20 14 -24 -25 -19 -14 -42 -41 19 -15 25 hvcurveto
        // 20 vmoveto
        // -20 14 -24 -25 -19 -14 4 5 rcurveline
        // -20 14 -24 -25 -19 -14 4 5 rlinecurve
        // -55 -23 -22 -59 vhcurveto
        // -30 15 22 8 vvcurveto
        // 8 -30 15 22 8 vvcurveto
        // 24 20 15 41 42 -20 14 -24 -25 -19 -14 -42 23 flex
        // 24 20 15 41 42 -20 14 hflex
        // 13 hmoveto
        // 41 42 -20 14 -24 -25 -19 -14 -42 hflex1
        // 15 41 42 -20 14 -24 -25 -19 -14 -42 8 flex1
        // endchar
        let charstring = &[
            251, 29, 253, 12, 21, 173, 134, 159, 133, 5, 140, 141, 142, 6, 251, 71, 129, 142, 7,
            109, 154, 161, 147, 89, 165, 125, 97, 98, 158, 124, 164, 8, 109, 154, 161, 147, 27,
            147, 109, 154, 161, 147, 27, 163, 159, 154, 180, 181, 119, 153, 115, 114, 120, 125, 97,
            98, 158, 124, 164, 31, 159, 4, 119, 153, 115, 114, 120, 125, 143, 144, 24, 119, 153,
            115, 114, 120, 125, 143, 144, 25, 84, 116, 117, 80, 30, 109, 154, 161, 147, 26, 147,
            109, 154, 161, 147, 26, 163, 159, 154, 180, 181, 119, 153, 115, 114, 120, 125, 97, 162,
            12, 35, 163, 159, 154, 180, 181, 119, 153, 12, 34, 152, 22, 180, 181, 119, 153, 115,
            114, 120, 125, 97, 12, 36, 154, 180, 181, 119, 153, 115, 114, 120, 125, 97, 147, 12,
            37, 14,
        ];
        let empty_index_bytes = [0u8; 8];
        let global_subrs = Index::new(&empty_index_bytes, false).unwrap();
        use Command::*;
        let mut commands = CaptureCommandSink::default();
        evaluate(
            &[],
            Index::Empty,
            global_subrs,
            None,
            None,
            charstring,
            &mut commands,
        )
        .unwrap();
        // Expected results from extracted glyph data in
        // font-test-data/test_data/extracted/charstring_path_ops-glyphs.txt
        // --------------------------------------------------------------------
        // m  -137,-632
        // l  -103,-637
        // l  -83,-643
        // l  -82,-643
        // l  -82,-641
        // l  -79,-641
        // l  -79,-820
        // l  -89,-820
        // l  -89,-817
        // c  -119,-802 -97,-794 -147,-768
        // c  -161,-810 -202,-791 -217,-766
        // c  -247,-766 -232,-744 -224,-744
        // c  -254,-736 -239,-714 -231,-714
        // c  -207,-714 -187,-699 -187,-658
        // c  -187,-616 -207,-602 -231,-602
        // c  -256,-602 -275,-616 -275,-658
        // c  -275,-699 -256,-714 -231,-714
        // l  -137,-632
        // m  -231,-694
        // c  -251,-680 -275,-705 -294,-719
        // l  -290,-714
        // l  -310,-700
        // c  -334,-725 -353,-739 -349,-734
        // c  -349,-789 -372,-811 -431,-811
        // c  -431,-841 -416,-819 -416,-811
        // c  -408,-841 -393,-819 -393,-811
        // c  -369,-791 -354,-750 -312,-770
        // c  -298,-794 -323,-813 -337,-855
        // c  -313,-855 -293,-840 -252,-840
        // c  -210,-840 -230,-855 -216,-855
        // l  -231,-694
        // m  -203,-855
        // c  -162,-813 -182,-799 -206,-799
        // c  -231,-799 -250,-813 -292,-855
        // c  -277,-814 -235,-834 -221,-858
        // c  -246,-877 -260,-919 -292,-911
        // l  -203,-855
        let expected = &[
            MoveTo(Fixed::from_i32(-137), Fixed::from_i32(-632)),
            LineTo(Fixed::from_i32(-103), Fixed::from_i32(-637)),
            LineTo(Fixed::from_i32(-83), Fixed::from_i32(-643)),
            LineTo(Fixed::from_i32(-82), Fixed::from_i32(-643)),
            LineTo(Fixed::from_i32(-82), Fixed::from_i32(-641)),
            LineTo(Fixed::from_i32(-79), Fixed::from_i32(-641)),
            LineTo(Fixed::from_i32(-79), Fixed::from_i32(-820)),
            LineTo(Fixed::from_i32(-89), Fixed::from_i32(-820)),
            LineTo(Fixed::from_i32(-89), Fixed::from_i32(-817)),
            CurveTo(
                Fixed::from_i32(-119),
                Fixed::from_i32(-802),
                Fixed::from_i32(-97),
                Fixed::from_i32(-794),
                Fixed::from_i32(-147),
                Fixed::from_i32(-768),
            ),
            CurveTo(
                Fixed::from_i32(-161),
                Fixed::from_i32(-810),
                Fixed::from_i32(-202),
                Fixed::from_i32(-791),
                Fixed::from_i32(-217),
                Fixed::from_i32(-766),
            ),
            CurveTo(
                Fixed::from_i32(-247),
                Fixed::from_i32(-766),
                Fixed::from_i32(-232),
                Fixed::from_i32(-744),
                Fixed::from_i32(-224),
                Fixed::from_i32(-744),
            ),
            CurveTo(
                Fixed::from_i32(-254),
                Fixed::from_i32(-736),
                Fixed::from_i32(-239),
                Fixed::from_i32(-714),
                Fixed::from_i32(-231),
                Fixed::from_i32(-714),
            ),
            CurveTo(
                Fixed::from_i32(-207),
                Fixed::from_i32(-714),
                Fixed::from_i32(-187),
                Fixed::from_i32(-699),
                Fixed::from_i32(-187),
                Fixed::from_i32(-658),
            ),
            CurveTo(
                Fixed::from_i32(-187),
                Fixed::from_i32(-616),
                Fixed::from_i32(-207),
                Fixed::from_i32(-602),
                Fixed::from_i32(-231),
                Fixed::from_i32(-602),
            ),
            CurveTo(
                Fixed::from_i32(-256),
                Fixed::from_i32(-602),
                Fixed::from_i32(-275),
                Fixed::from_i32(-616),
                Fixed::from_i32(-275),
                Fixed::from_i32(-658),
            ),
            CurveTo(
                Fixed::from_i32(-275),
                Fixed::from_i32(-699),
                Fixed::from_i32(-256),
                Fixed::from_i32(-714),
                Fixed::from_i32(-231),
                Fixed::from_i32(-714),
            ),
            LineTo(Fixed::from_i32(-137), Fixed::from_i32(-632)),
            MoveTo(Fixed::from_i32(-231), Fixed::from_i32(-694)),
            CurveTo(
                Fixed::from_i32(-251),
                Fixed::from_i32(-680),
                Fixed::from_i32(-275),
                Fixed::from_i32(-705),
                Fixed::from_i32(-294),
                Fixed::from_i32(-719),
            ),
            LineTo(Fixed::from_i32(-290), Fixed::from_i32(-714)),
            LineTo(Fixed::from_i32(-310), Fixed::from_i32(-700)),
            CurveTo(
                Fixed::from_i32(-334),
                Fixed::from_i32(-725),
                Fixed::from_i32(-353),
                Fixed::from_i32(-739),
                Fixed::from_i32(-349),
                Fixed::from_i32(-734),
            ),
            CurveTo(
                Fixed::from_i32(-349),
                Fixed::from_i32(-789),
                Fixed::from_i32(-372),
                Fixed::from_i32(-811),
                Fixed::from_i32(-431),
                Fixed::from_i32(-811),
            ),
            CurveTo(
                Fixed::from_i32(-431),
                Fixed::from_i32(-841),
                Fixed::from_i32(-416),
                Fixed::from_i32(-819),
                Fixed::from_i32(-416),
                Fixed::from_i32(-811),
            ),
            CurveTo(
                Fixed::from_i32(-408),
                Fixed::from_i32(-841),
                Fixed::from_i32(-393),
                Fixed::from_i32(-819),
                Fixed::from_i32(-393),
                Fixed::from_i32(-811),
            ),
            CurveTo(
                Fixed::from_i32(-369),
                Fixed::from_i32(-791),
                Fixed::from_i32(-354),
                Fixed::from_i32(-750),
                Fixed::from_i32(-312),
                Fixed::from_i32(-770),
            ),
            CurveTo(
                Fixed::from_i32(-298),
                Fixed::from_i32(-794),
                Fixed::from_i32(-323),
                Fixed::from_i32(-813),
                Fixed::from_i32(-337),
                Fixed::from_i32(-855),
            ),
            CurveTo(
                Fixed::from_i32(-313),
                Fixed::from_i32(-855),
                Fixed::from_i32(-293),
                Fixed::from_i32(-840),
                Fixed::from_i32(-252),
                Fixed::from_i32(-840),
            ),
            CurveTo(
                Fixed::from_i32(-210),
                Fixed::from_i32(-840),
                Fixed::from_i32(-230),
                Fixed::from_i32(-855),
                Fixed::from_i32(-216),
                Fixed::from_i32(-855),
            ),
            LineTo(Fixed::from_i32(-231), Fixed::from_i32(-694)),
            MoveTo(Fixed::from_i32(-203), Fixed::from_i32(-855)),
            CurveTo(
                Fixed::from_i32(-162),
                Fixed::from_i32(-813),
                Fixed::from_i32(-182),
                Fixed::from_i32(-799),
                Fixed::from_i32(-206),
                Fixed::from_i32(-799),
            ),
            CurveTo(
                Fixed::from_i32(-231),
                Fixed::from_i32(-799),
                Fixed::from_i32(-250),
                Fixed::from_i32(-813),
                Fixed::from_i32(-292),
                Fixed::from_i32(-855),
            ),
            CurveTo(
                Fixed::from_i32(-277),
                Fixed::from_i32(-814),
                Fixed::from_i32(-235),
                Fixed::from_i32(-834),
                Fixed::from_i32(-221),
                Fixed::from_i32(-858),
            ),
            CurveTo(
                Fixed::from_i32(-246),
                Fixed::from_i32(-877),
                Fixed::from_i32(-260),
                Fixed::from_i32(-919),
                Fixed::from_i32(-292),
                Fixed::from_i32(-911),
            ),
            LineTo(Fixed::from_i32(-203), Fixed::from_i32(-855)),
        ];
        assert_eq!(&commands.0, expected);
    }

    /// Fuzzer caught subtract with overflow
    /// <https://g-issues.oss-fuzz.com/issues/383609770>
    #[test]
    fn coords_remaining_avoid_overflow() {
        // Test case:
        // Evaluate HHCURVETO operator with 2 elements on the stack
        let mut commands = CaptureCommandSink::default();
        let mut evaluator =
            Evaluator::new(&[], Index::Empty, Index::Empty, None, None, &mut commands);
        evaluator.stack.push(0).unwrap();
        evaluator.stack.push(0).unwrap();
        let mut cursor = FontData::new(&[]).cursor();
        // Just don't panic
        let _ = evaluator.evaluate_operator(Operator::HhCurveTo, &mut cursor, 0);
    }

    #[test]
    fn ignore_reserved_operators() {
        let charstring = &[
            0u8, // reserved
            32,  // push -107
            22,  // hmoveto
            2,   // reserved
        ];
        let empty_index_bytes = [0u8; 8];
        let global_subrs = Index::new(&empty_index_bytes, false).unwrap();
        let mut commands = CaptureCommandSink::default();
        evaluate(
            &[],
            Index::Empty,
            global_subrs,
            None,
            None,
            charstring,
            &mut commands,
        )
        .unwrap();
        assert_eq!(
            commands.0,
            [Command::MoveTo(Fixed::from_i32(-107), Fixed::ZERO)]
        );
    }
}
