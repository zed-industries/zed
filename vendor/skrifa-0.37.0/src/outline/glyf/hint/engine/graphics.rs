//! Managing the graphics state.
//!
//! Implements 45 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#managing-the-graphics-state>

use super::{
    super::{math, program::Program, round::RoundMode},
    Engine, F26Dot6, HintErrorKind, OpResult, Point,
};

impl Engine<'_> {
    /// Set vectors to coordinate axis.
    ///
    /// SVTCA\[a\] (0x00 - 0x01)
    ///
    /// Sets both the projection_vector and freedom_vector to the same one of
    /// the coordinate axes.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-freedom-and-projection-vectors-to-coordinate-axis>
    ///
    /// SPVTCA\[a\] (0x02 - 0x03)
    ///
    /// Sets the projection_vector to one of the coordinate axes depending on
    /// the value of the flag a.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-projection_vector-to-coordinate-axis>
    ///
    /// SFVTCA\[a\] (0x04 - 0x05)
    ///
    /// Sets the freedom_vector to one of the coordinate axes depending on
    /// the value of the flag a.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-freedom_vector-to-coordinate-axis>
    ///
    /// FreeType combines these into a single function using some bit magic on
    /// the opcode to determine which axes and vectors to set.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4051>
    pub(super) fn op_svtca(&mut self, opcode: u8) -> OpResult {
        let opcode = opcode as i32;
        // The low bit of the opcode determines the axis to set (1 = x, 0 = y).
        let x = (opcode & 1) << 14;
        let y = x ^ 0x4000;
        // Opcodes 0..4 set the projection vector.
        if opcode < 4 {
            self.graphics.proj_vector.x = x;
            self.graphics.proj_vector.y = y;
            self.graphics.dual_proj_vector.x = x;
            self.graphics.dual_proj_vector.y = y;
        }
        // Opcodes with bit 2 unset modify the freedom vector.
        if opcode & 2 == 0 {
            self.graphics.freedom_vector.x = x;
            self.graphics.freedom_vector.y = y;
        }
        self.graphics.update_projection_state();
        Ok(())
    }

    /// Set vectors to line.
    ///
    /// SPVTL\[a\] (0x06 - 0x07)
    ///
    /// Sets the projection_vector to a unit vector parallel or perpendicular
    /// to the line segment from point p1 to point p2.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-projection_vector-to-line>
    ///
    /// SFVTL\[a\] (0x08 - 0x09)
    ///
    /// Sets the freedom_vector to a unit vector parallel or perpendicular
    /// to the line segment from point p1 to point p2.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-freedom_vector-to-line>
    ///
    /// Pops: p1, p2 (point number)
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3986>
    pub(super) fn op_svtl(&mut self, opcode: u8) -> OpResult {
        let index1 = self.value_stack.pop_usize()?;
        let index2 = self.value_stack.pop_usize()?;
        let is_parallel = opcode & 1 == 0;
        let p1 = self.graphics.zp1().point(index2)?;
        let p2 = self.graphics.zp2().point(index1)?;
        let vector = line_vector(p1, p2, is_parallel);
        if opcode < 8 {
            self.graphics.proj_vector = vector;
            self.graphics.dual_proj_vector = vector;
        } else {
            self.graphics.freedom_vector = vector;
        }
        self.graphics.update_projection_state();
        Ok(())
    }

    /// Set freedom vector to projection vector.
    ///
    /// SFVTPV[] (0x0E)
    ///
    /// Sets the freedom_vector to be the same as the projection_vector.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-freedom_vector-to-projection-vector>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4128>
    pub(super) fn op_sfvtpv(&mut self) -> OpResult {
        self.graphics.freedom_vector = self.graphics.proj_vector;
        self.graphics.update_projection_state();
        Ok(())
    }

    /// Set dual projection vector to line.
    ///
    /// SDPVTL\[a\] (0x86 - 0x87)
    ///
    /// Pops: p1, p2 (point number)
    ///
    /// Pops two point numbers from the stack and uses them to specify a line
    /// that defines a second, dual_projection_vector.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-dual-projection_vector-to-line>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4663>
    pub(super) fn op_sdpvtl(&mut self, opcode: u8) -> OpResult {
        let index1 = self.value_stack.pop_usize()?;
        let index2 = self.value_stack.pop_usize()?;
        let is_parallel = opcode & 1 == 0;
        // First set the dual projection vector from *original* points.
        let p1 = self.graphics.zp1().original(index2)?;
        let p2 = self.graphics.zp2().original(index1)?;
        self.graphics.dual_proj_vector = line_vector(p1, p2, is_parallel);
        // Now set the projection vector from the *current* points.
        let p1 = self.graphics.zp1().point(index2)?;
        let p2 = self.graphics.zp2().point(index1)?;
        self.graphics.proj_vector = line_vector(p1, p2, is_parallel);
        self.graphics.update_projection_state();
        Ok(())
    }

    /// Set projection vector from stack.
    ///
    /// SPVFS[] (0x0A)
    ///
    /// Pops: y, x (2.14 fixed point numbers padded with zeroes)
    ///
    /// Sets the direction of the projection_vector, using values x and y taken
    /// from the stack, so that its projections onto the x and y-axes are x and
    /// y, which are specified as signed (two’s complement) fixed-point (2.14)
    /// numbers. The square root of (x2 + y2) must be equal to 0x4000 (hex)
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-projection_vector-from-stack>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4142>
    pub(super) fn op_spvfs(&mut self) -> OpResult {
        let y = self.value_stack.pop()? as i16 as i32;
        let x = self.value_stack.pop()? as i16 as i32;
        let vector = if (x, y) == (0, 0) {
            self.graphics.proj_vector
        } else {
            math::normalize14(x, y)
        };
        self.graphics.proj_vector = vector;
        self.graphics.dual_proj_vector = vector;
        self.graphics.update_projection_state();
        Ok(())
    }

    /// Set freedom vector from stack.
    ///
    /// SFVFS[] (0x0B)
    ///
    /// Pops: y, x (2.14 fixed point numbers padded with zeroes)
    ///
    /// Sets the direction of the freedom_vector, using values x and y taken
    /// from the stack, so that its projections onto the x and y-axes are x and
    /// y, which are specified as signed (two’s complement) fixed-point (2.14)
    /// numbers. The square root of (x2 + y2) must be equal to 0x4000 (hex)
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-freedom_vector-from-stack>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4169>
    pub(super) fn op_sfvfs(&mut self) -> OpResult {
        let y = self.value_stack.pop()? as i16 as i32;
        let x = self.value_stack.pop()? as i16 as i32;
        let vector = if (x, y) == (0, 0) {
            self.graphics.freedom_vector
        } else {
            math::normalize14(x, y)
        };
        self.graphics.freedom_vector = vector;
        self.graphics.update_projection_state();
        Ok(())
    }

    /// Get projection vector.
    ///
    /// GPV[] (0x0C)
    ///
    /// Pushes: x, y (2.14 fixed point numbers padded with zeroes)
    ///
    /// Pushes the x and y components of the projection_vector onto the stack
    /// as two 2.14 numbers.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#get-projection_vector>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4194>
    pub(super) fn op_gpv(&mut self) -> OpResult {
        let vector = self.graphics.proj_vector;
        self.value_stack.push(vector.x)?;
        self.value_stack.push(vector.y)
    }

    /// Get freedom vector.
    ///
    /// GFV[] (0x0D)
    ///
    /// Pushes: x, y (2.14 fixed point numbers padded with zeroes)
    ///
    /// Pushes the x and y components of the freedom_vector onto the stack as
    /// two 2.14 numbers.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#get-freedom_vector>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4209>
    pub(super) fn op_gfv(&mut self) -> OpResult {
        let vector = self.graphics.freedom_vector;
        self.value_stack.push(vector.x)?;
        self.value_stack.push(vector.y)
    }

    /// Set reference point 0.
    ///
    /// SRP0[] (0x10)
    ///
    /// Pops: p (point number)
    ///
    /// Pops a point number from the stack and sets rp0 to that point number.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-reference-point-0>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4224>
    pub(super) fn op_srp0(&mut self) -> OpResult {
        let p = self.value_stack.pop_usize()?;
        self.graphics.rp0 = p;
        Ok(())
    }

    /// Set reference point 1.
    ///
    /// SRP1[] (0x11)
    ///
    /// Pops: p (point number)
    ///
    /// Pops a point number from the stack and sets rp1 to that point number.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-reference-point-1>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4238>
    pub(super) fn op_srp1(&mut self) -> OpResult {
        let p = self.value_stack.pop_usize()?;
        self.graphics.rp1 = p;
        Ok(())
    }

    /// Set reference point 2.
    ///
    /// SRP2[] (0x12)
    ///
    /// Pops: p (point number)
    ///
    /// Pops a point number from the stack and sets rp2 to that point number.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-reference-point-2>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4252>
    pub(super) fn op_srp2(&mut self) -> OpResult {
        let p = self.value_stack.pop_usize()?;
        self.graphics.rp2 = p;
        Ok(())
    }

    /// Set zone pointer 0.
    ///
    /// SZP0[] (0x13)
    ///
    /// Pops: n (zone number)
    ///
    /// Pops a zone number, n, from the stack and sets zp0 to the zone with
    /// that number. If n is 0, zp0 points to zone 0. If n is 1, zp0 points
    /// to zone 1. Any other value for n is an error.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-zone-pointer-0>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4746>
    pub(super) fn op_szp0(&mut self) -> OpResult {
        let n = self.value_stack.pop()?;
        self.graphics.zp0 = n.try_into()?;
        Ok(())
    }

    /// Set zone pointer 1.
    ///
    /// SZP1[] (0x14)
    ///
    /// Pops: n (zone number)
    ///
    /// Pops a zone number, n, from the stack and sets zp0 to the zone with
    /// that number. If n is 0, zp1 points to zone 0. If n is 1, zp0 points
    /// to zone 1. Any other value for n is an error.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-zone-pointer-1>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4776>
    pub(super) fn op_szp1(&mut self) -> OpResult {
        let n = self.value_stack.pop()?;
        self.graphics.zp1 = n.try_into()?;
        Ok(())
    }

    /// Set zone pointer 2.
    ///
    /// SZP2[] (0x15)
    ///
    /// Pops: n (zone number)
    ///
    /// Pops a zone number, n, from the stack and sets zp0 to the zone with
    /// that number. If n is 0, zp2 points to zone 0. If n is 1, zp0 points
    /// to zone 1. Any other value for n is an error.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-zone-pointer-2>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4806>
    pub(super) fn op_szp2(&mut self) -> OpResult {
        let n = self.value_stack.pop()?;
        self.graphics.zp2 = n.try_into()?;
        Ok(())
    }

    /// Set zone pointers.
    ///
    /// SZPS[] (0x16)
    ///
    /// Pops: n (zone number)
    ///
    /// Pops a zone number from the stack and sets all of the zone pointers to
    /// point to the zone with that number. If n is 0, all three zone pointers
    /// will point to zone 0. If n is 1, all three zone pointers will point to
    /// zone 1. Any other value for n is an error.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-zone-pointers>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4836>
    pub(super) fn op_szps(&mut self) -> OpResult {
        let n = self.value_stack.pop()?;
        let zp = n.try_into()?;
        self.graphics.zp0 = zp;
        self.graphics.zp1 = zp;
        self.graphics.zp2 = zp;
        Ok(())
    }

    /// Round to half grid.
    ///
    /// RTHG[] (0x19)
    ///
    /// Sets the round_state variable to state 0 (hg). In this state, the
    /// coordinates of a point are rounded to the nearest half grid line.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#round-to-half-grid>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4393>
    pub(super) fn op_rthg(&mut self) -> OpResult {
        self.graphics.round_state.mode = RoundMode::HalfGrid;
        Ok(())
    }

    /// Round to grid.
    ///
    /// RTG[] (0x18)
    ///
    /// Sets the round_state variable to state 1 (g). In this state, distances
    /// are rounded to the closest grid line.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#round-to-grid>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4407>
    pub(super) fn op_rtg(&mut self) -> OpResult {
        self.graphics.round_state.mode = RoundMode::Grid;
        Ok(())
    }

    /// Round to double grid.
    ///
    /// RTDG[] (0x3D)
    ///
    /// Sets the round_state variable to state 2 (dg). In this state, distances
    /// are rounded to the closest half or integer pixel.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#round-to-double-grid>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4420>
    pub(super) fn op_rtdg(&mut self) -> OpResult {
        self.graphics.round_state.mode = RoundMode::DoubleGrid;
        Ok(())
    }

    /// Round down to grid.
    ///
    /// RDTG[] (0x7D)
    ///
    /// Sets the round_state variable to state 3 (dtg). In this state, distances
    /// are rounded down to the closest integer grid line.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#round-down-to-grid>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4447>
    pub(super) fn op_rdtg(&mut self) -> OpResult {
        self.graphics.round_state.mode = RoundMode::DownToGrid;
        Ok(())
    }

    /// Round up to grid.
    ///
    /// RUTG[] (0x7C)
    ///
    /// Sets the round_state variable to state 4 (utg). In this state distances
    /// are rounded up to the closest integer pixel boundary.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#round-up-to-grid>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4433>
    pub(super) fn op_rutg(&mut self) -> OpResult {
        self.graphics.round_state.mode = RoundMode::UpToGrid;
        Ok(())
    }

    /// Round off.
    ///
    /// ROFF[] (0x7A)
    ///
    /// Sets the round_state variable to state 5 (off). In this state rounding
    /// is turned off.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#round-off>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4461>
    pub(super) fn op_roff(&mut self) -> OpResult {
        self.graphics.round_state.mode = RoundMode::Off;
        Ok(())
    }

    /// Super round.
    ///
    /// SROUND[] (0x76)
    ///
    /// Pops: n (number decomposed to obtain period, phase threshold)
    ///
    /// SROUND allows you fine control over the effects of the round_state
    /// variable by allowing you to set the values of three components of
    /// the round_state: period, phase, and threshold.
    ///
    /// More formally, SROUND maps the domain of 26.6 fixed point numbers into
    /// a set of discrete values that are separated by equal distances.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#super-round>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4475>
    pub(super) fn op_sround(&mut self) -> OpResult {
        let n = self.value_stack.pop()?;
        self.super_round(0x4000, n);
        self.graphics.round_state.mode = RoundMode::Super;
        Ok(())
    }

    /// Super round 45 degrees.
    ///
    /// S45ROUND[] (0x77)
    ///
    /// Pops: n (number decomposed to obtain period, phase threshold)
    ///
    /// S45ROUND is analogous to SROUND. The gridPeriod is SQRT(2)/2 pixels
    /// rather than 1 pixel. It is useful for measuring at a 45 degree angle
    /// with the coordinate axes.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#super-round-45-degrees>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4492>
    pub(super) fn op_s45round(&mut self) -> OpResult {
        let n = self.value_stack.pop()?;
        self.super_round(0x2D41, n);
        self.graphics.round_state.mode = RoundMode::Super45;
        Ok(())
    }

    /// Helper function for decomposing period, phase and threshold for
    /// the SROUND[] and SROUND45[] instructions.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2299>
    fn super_round(&mut self, grid_period: i32, selector: i32) {
        let round_state = &mut self.graphics.round_state;
        let period = match selector & 0xC0 {
            0 => grid_period / 2,
            0x40 => grid_period,
            0x80 => grid_period * 2,
            0xC0 => grid_period,
            _ => round_state.period,
        };
        let phase = match selector & 0x30 {
            0 => 0,
            0x10 => period / 4,
            0x20 => period / 2,
            0x30 => period * 3 / 4,
            _ => round_state.phase,
        };
        let threshold = if (selector & 0x0F) == 0 {
            period - 1
        } else {
            ((selector & 0x0F) - 4) * period / 8
        };
        round_state.period = period >> 8;
        round_state.phase = phase >> 8;
        round_state.threshold = threshold >> 8;
    }

    /// Set loop variable.
    ///
    /// SLOOP[] (0x17)
    ///
    /// Pops: n (value for loop Graphics State variable (integer))
    ///
    /// Pops a value, n, from the stack and sets the loop variable count to
    /// that value. The loop variable works with the SHP\[a\], SHPIX[], IP[],
    /// FLIPPT[], and ALIGNRP[]. The value n indicates the number of times
    /// the instruction is to be repeated. After the instruction executes,
    /// the loop variable is reset to 1.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-loop-variable>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3287>
    pub(super) fn op_sloop(&mut self) -> OpResult {
        let n = self.value_stack.pop()?;
        if n < 0 {
            return Err(HintErrorKind::NegativeLoopCounter);
        }
        // As in FreeType, heuristically limit the number of loops to 16 bits.
        self.graphics.loop_counter = (n as u32).min(0xFFFF);
        Ok(())
    }

    /// Set minimum distance.
    ///
    /// SMD[] (0x1A)
    ///
    /// Pops: distance: value for minimum_distance (F26Dot6)
    ///
    /// Pops a value from the stack and sets the minimum_distance variable
    /// to that value. The distance is assumed to be expressed in sixty-fourths
    /// of a pixel.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-minimum_distance>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4266>
    pub(super) fn op_smd(&mut self) -> OpResult {
        let distance = self.value_stack.pop_f26dot6()?;
        self.graphics.min_distance = distance;
        Ok(())
    }

    /// Instruction execution control.
    ///
    /// INSTCTRL[] (0x8E)
    ///
    /// Pops: s: selector flag (int32)
    ///       value: used to set value of instruction_control (uint16 padded)
    ///
    /// Sets the instruction control state variable making it possible to turn
    /// on or off the execution of instructions and to regulate use of
    /// parameters set in the CVT program. INSTCTRL[ ] can only be executed in
    /// the CVT program.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#instruction-execution-control>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4871>
    pub(super) fn op_instctrl(&mut self) -> OpResult {
        let selector = self.value_stack.pop()? as u32;
        let value = self.value_stack.pop()? as u32;
        // Selectors are indices starting with 1; not flags.
        // Avoid potential subtract with overflow below.
        // See <https://bugs.chromium.org/p/oss-fuzz/issues/detail?id=70426>
        // and <https://oss-fuzz.com/testcase?key=6243979511791616>
        if !(1..=3).contains(&selector) {
            return Ok(());
        }
        // Convert index to flag.
        let selector_flag = 1 << (selector - 1);
        if value != 0 && value != selector_flag {
            return Ok(());
        }
        // If preserving linear metrics, prevent modification of the backward
        // compatibility flag.
        if selector == 3 && self.graphics.target.preserve_linear_metrics() {
            return Ok(());
        }
        match (self.program.initial, selector) {
            // Typically, this instruction can only be executed in the prep table.
            (Program::ControlValue, _) => {
                self.graphics.instruct_control &= !(selector_flag as u8);
                self.graphics.instruct_control |= value as u8;
            }
            // Allow an exception in the glyph program for selector 3 which can
            // temporarily disable backward compatibility mode.
            (Program::Glyph, 3) => {
                self.graphics.backward_compatibility = value != 4;
            }
            _ => {}
        }
        Ok(())
    }

    /// Scan conversion control.
    ///
    /// SCANCTRL[] (0x85)
    ///
    /// Pops: n: flags indicating when to turn on dropout control mode
    ///
    /// SCANCTRL is used to set the value of the Graphics State variable
    /// scan_control which in turn determines whether the scan converter
    /// will activate dropout control for this glyph.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#scan-conversion-control>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4933>
    pub(super) fn op_scanctrl(&mut self) -> OpResult {
        let n = self.value_stack.pop()?;
        // Bits 0-7 represent the threshold value for ppem.
        let threshold = n & 0xFF;
        match threshold {
            // A value of FF in bits 0-7 means invoke scan_control for all
            // sizes.
            0xFF => self.graphics.scan_control = true,
            // A value of 0 in bits 0-7 means never invoke scan_control.
            0 => self.graphics.scan_control = false,
            _ => {
                let ppem = self.graphics.ppem;
                let is_rotated = self.graphics.is_rotated;
                let is_stretched = self.graphics.is_stretched;
                let scan_control = &mut self.graphics.scan_control;
                // Bits 8-13 are used to turn on scan_control in cases where
                // the specified conditions are met. Bits 8, 9 and 10 are used
                // to turn on the scan_control mode (assuming other
                // conditions do not block it). Bits 11, 12, and 13 are used to
                // turn off the dropout mode unless other conditions force it
                if (n & 0x100) != 0 && ppem <= threshold {
                    // Bit 8: Set scan_control to TRUE if other conditions
                    // do not block and ppem is less than or equal to the
                    // threshold value.
                    *scan_control = true;
                }
                if (n & 0x200) != 0 && is_rotated {
                    // Bit 9: Set scan_control to TRUE if other conditions
                    // do not block and the glyph is rotated
                    *scan_control = true;
                }
                if (n & 0x400) != 0 && is_stretched {
                    // Bit 10: Set scan_control to TRUE if other conditions
                    // do not block and the glyph is stretched.
                    *scan_control = true;
                }
                if (n & 0x800) != 0 && ppem > threshold {
                    // Bit 11: Set scan_control to FALSE unless ppem is less
                    // than or equal to the threshold value.
                    *scan_control = false;
                }
                if (n & 0x1000) != 0 && is_rotated {
                    // Bit 12: Set scan_control to FALSE based on rotation
                    // state.
                    *scan_control = false;
                }
                if (n & 0x2000) != 0 && is_stretched {
                    // Bit 13: Set scan_control to FALSE based on stretched
                    // state.
                    *scan_control = false;
                }
            }
        }
        Ok(())
    }

    /// Scan type.
    ///
    /// SCANTYPE[] (0x8D)
    ///
    /// Pops: n: 16 bit integer
    ///
    /// Pops a 16-bit integer whose value is used to determine which rules the
    /// scan converter will use.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#scantype>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4980>
    pub(super) fn op_scantype(&mut self) -> OpResult {
        let n = self.value_stack.pop()?;
        self.graphics.scan_type = n & 0xFFFF;
        Ok(())
    }

    /// Set control value table cut in.
    ///
    /// SCVTCI[] (0x1D)
    ///
    /// Pops: n: value for cut_in (F26Dot6)
    ///
    /// Sets the control_value_cut_in in the Graphics State. The value n is
    /// expressed in sixty-fourths of a pixel.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-control-value-table-cut-in>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4280>
    pub(super) fn op_scvtci(&mut self) -> OpResult {
        let n = self.value_stack.pop_f26dot6()?;
        self.graphics.control_value_cutin = n;
        Ok(())
    }

    /// Set single width cut in.
    ///
    /// SSWCI[] (0x1E)
    ///
    /// Pops: n: value for single_width_cut_in (F26Dot6)
    ///
    /// Sets the single_width_cut_in in the Graphics State. The value n is
    /// expressed in sixty-fourths of a pixel.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-single_width_cut_in>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4294>
    pub(super) fn op_sswci(&mut self) -> OpResult {
        let n = self.value_stack.pop_f26dot6()?;
        self.graphics.single_width_cutin = n;
        Ok(())
    }

    /// Set single width.
    ///
    /// SSW[] (0x1F)
    ///
    /// Pops: n: value for single_width_value (FUnits)
    ///
    /// Sets the single_width_value in the Graphics State. The
    /// single_width_value is expressed in FUnits, which the
    /// interpreter converts to pixels (F26Dot6).
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-single-width>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4308>
    pub(super) fn op_ssw(&mut self) -> OpResult {
        let n = self.value_stack.pop()?;
        self.graphics.single_width = F26Dot6::from_bits(math::mul(n, self.graphics.scale));
        Ok(())
    }

    /// Set auto flip on.
    ///
    /// FLIPON[] (0x4D)
    ///
    /// Sets the auto_flip Boolean in the Graphics State to TRUE causing the
    /// MIRP instructions to ignore the sign of Control Value Table entries.
    /// The default auto_flip Boolean value is TRUE.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-the-auto_flip-boolean-to-on>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4323>
    pub(super) fn op_flipon(&mut self) -> OpResult {
        self.graphics.auto_flip = true;
        Ok(())
    }

    /// Set auto flip off.
    ///
    /// FLIPOFF[] (0x4E)
    ///
    /// Set the auto_flip Boolean in the Graphics State to FALSE causing the
    /// MIRP instructions to use the sign of Control Value Table entries.
    /// The default auto_flip Boolean value is TRUE.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-the-auto_flip-boolean-to-off>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4336>
    pub(super) fn op_flipoff(&mut self) -> OpResult {
        self.graphics.auto_flip = false;
        Ok(())
    }

    /// Set angle weight.
    ///
    /// SANGW[] (0x7E)
    ///
    /// Pops: weight: value for angle_weight
    ///
    /// SANGW is no longer needed because of dropped support to the AA
    /// (Adjust Angle) instruction. AA was the only instruction that used
    /// angle_weight in the global graphics state.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-angle_weight>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4349>
    pub(super) fn op_sangw(&mut self) -> OpResult {
        // totally unsupported but we still need to pop the stack value
        let _weight = self.value_stack.pop()?;
        Ok(())
    }

    /// Set delta base in graphics state.
    ///
    /// SDB[] (0x5E)
    ///
    /// Pops: n: value for delta_base
    ///
    /// Pops a number, n, and sets delta_base to the value n. The default for
    /// delta_base is 9.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-delta_base-in-the-graphics-state>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4362>
    pub(super) fn op_sdb(&mut self) -> OpResult {
        let n = self.value_stack.pop()?;
        self.graphics.delta_base = n as u16;
        Ok(())
    }

    /// Set delta shift in graphics state.
    ///
    /// SDS[] (0x5F)
    ///
    /// Pops: n: value for delta_shift
    ///
    /// Sets delta_shift to the value n. The default for delta_shift is 3.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#set-delta_shift-in-the-graphics-state>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4376>
    pub(super) fn op_sds(&mut self) -> OpResult {
        let n = self.value_stack.pop()?;
        if n as u32 > 6 {
            Err(HintErrorKind::InvalidStackValue(n))
        } else {
            self.graphics.delta_shift = n as u16;
            Ok(())
        }
    }
}

/// Computes a parallel or perpendicular normalized vector for the line
/// between the two given points.
///
/// This is common code for the "set vector to line" instructions.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4009>
fn line_vector(p1: Point<F26Dot6>, p2: Point<F26Dot6>, is_parallel: bool) -> Point<i32> {
    let mut a = (p1.x - p2.x).to_bits();
    let mut b = (p1.y - p2.y).to_bits();
    if a == 0 && b == 0 {
        // If the points are equal, set to the x axis.
        a = 0x4000;
    } else if !is_parallel {
        // Perform a counter-clockwise rotation by 90 degrees to form a
        // perpendicular line.
        let c = b;
        b = a;
        a = -c;
    }
    math::normalize14(a, b)
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            super::zone::{Zone, ZonePointer},
            math, F2Dot14, MockEngine,
        },
        F26Dot6, HintErrorKind, Point, Program, RoundMode,
    };

    // Some helpful constants for testing vectors
    const ONE: i32 = F2Dot14::ONE.to_bits() as i32;

    const X_AXIS: Point<i32> = Point::new(ONE, 0);
    const Y_AXIS: Point<i32> = Point::new(0, ONE);

    #[test]
    fn set_vectors_to_coord_axis() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // freedom and projection vector to y axis
        engine.op_svtca(0x00).unwrap();
        assert_eq!(engine.graphics.freedom_vector, Y_AXIS);
        assert_eq!(engine.graphics.proj_vector, Y_AXIS);
        // freedom and projection vector to x axis
        engine.op_svtca(0x01).unwrap();
        assert_eq!(engine.graphics.freedom_vector, X_AXIS);
        assert_eq!(engine.graphics.proj_vector, X_AXIS);
        // projection vector to y axis
        engine.op_svtca(0x02).unwrap();
        assert_eq!(engine.graphics.proj_vector, Y_AXIS);
        // projection vector to x axis
        engine.op_svtca(0x03).unwrap();
        assert_eq!(engine.graphics.proj_vector, X_AXIS);
        // freedom vector to y axis
        engine.op_svtca(0x04).unwrap();
        assert_eq!(engine.graphics.freedom_vector, Y_AXIS);
        // freedom vector to x axis
        engine.op_svtca(0x05).unwrap();
        assert_eq!(engine.graphics.freedom_vector, X_AXIS);
    }

    #[test]
    fn set_get_vectors_from_stack() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // projection vector
        engine.value_stack.push(X_AXIS.x).unwrap();
        engine.value_stack.push(X_AXIS.y).unwrap();
        engine.op_spvfs().unwrap();
        assert_eq!(engine.graphics.proj_vector, X_AXIS);
        engine.op_gpv().unwrap();
        let y = engine.value_stack.pop().unwrap();
        let x = engine.value_stack.pop().unwrap();
        assert_eq!(Point::new(x, y), X_AXIS);
        // freedom vector
        engine.value_stack.push(Y_AXIS.x).unwrap();
        engine.value_stack.push(Y_AXIS.y).unwrap();
        engine.op_sfvfs().unwrap();
        assert_eq!(engine.graphics.freedom_vector, Y_AXIS);
        engine.op_gfv().unwrap();
        let y = engine.value_stack.pop().unwrap();
        let x = engine.value_stack.pop().unwrap();
        assert_eq!(Point::new(x, y), Y_AXIS);
    }

    #[test]
    fn set_vectors_to_line() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // Set up a zone for testing and set all the zone pointers to it.
        let points = &mut [Point::new(0, 0), Point::new(64, 0)].map(|p| p.map(F26Dot6::from_bits));
        let original =
            &mut [Point::new(0, 64), Point::new(0, -64)].map(|p| p.map(F26Dot6::from_bits));
        engine.graphics.zones[1] = Zone {
            points,
            original,
            unscaled: &mut [],
            flags: &mut [],
            contours: &[],
        };
        engine.value_stack.push(1).unwrap();
        engine.op_szps().unwrap();
        // First, push point indices (a few times for reuse)
        for _ in 0..6 {
            engine.value_stack.push(1).unwrap();
            engine.value_stack.push(0).unwrap();
        }
        // SPVTL: set projection vector to line:
        {
            // (parallel)
            engine.op_svtl(0x6).unwrap();
            assert_eq!(engine.graphics.proj_vector, X_AXIS);
            // (perpendicular)
            engine.op_svtl(0x7).unwrap();
            assert_eq!(engine.graphics.proj_vector, Point::new(0, ONE));
        }
        // SFVTL: set freedom vector to line:
        {
            // (parallel)
            engine.op_svtl(0x8).unwrap();
            assert_eq!(engine.graphics.freedom_vector, X_AXIS);
            // (perpendicular)
            engine.op_svtl(0x9).unwrap();
            assert_eq!(engine.graphics.freedom_vector, Point::new(0, ONE));
        }
        // SDPVTL: set dual projection vector to line:
        {
            // (parallel)
            engine.op_sdpvtl(0x86).unwrap();
            assert_eq!(engine.graphics.dual_proj_vector, Point::new(0, -ONE));
            // (perpendicular)
            engine.op_sdpvtl(0x87).unwrap();
            assert_eq!(engine.graphics.dual_proj_vector, Point::new(ONE, 0));
        }
    }

    /// Lots of little tests for instructions that just set fields on
    /// the graphics state.
    #[test]
    fn simple_state_setting() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // srp0
        engine.value_stack.push(111).unwrap();
        engine.op_srp0().unwrap();
        assert_eq!(engine.graphics.rp0, 111);
        // srp1
        engine.value_stack.push(222).unwrap();
        engine.op_srp1().unwrap();
        assert_eq!(engine.graphics.rp1, 222);
        // srp2
        engine.value_stack.push(333).unwrap();
        engine.op_srp2().unwrap();
        assert_eq!(engine.graphics.rp2, 333);
        // zp0
        engine.value_stack.push(1).unwrap();
        engine.op_szp0().unwrap();
        assert_eq!(engine.graphics.zp0, ZonePointer::Glyph);
        // zp1
        engine.value_stack.push(0).unwrap();
        engine.op_szp1().unwrap();
        assert_eq!(engine.graphics.zp1, ZonePointer::Twilight);
        // zp2
        engine.value_stack.push(1).unwrap();
        engine.op_szp2().unwrap();
        assert_eq!(engine.graphics.zp2, ZonePointer::Glyph);
        // zps
        engine.value_stack.push(0).unwrap();
        engine.op_szps().unwrap();
        assert_eq!(
            [
                engine.graphics.zp0,
                engine.graphics.zp1,
                engine.graphics.zp2
            ],
            [ZonePointer::Twilight; 3]
        );
        // zp failure
        engine.value_stack.push(2).unwrap();
        assert!(matches!(
            engine.op_szps(),
            Err(HintErrorKind::InvalidZoneIndex(2))
        ));
        // rtg
        engine.op_rtg().unwrap();
        assert_eq!(engine.graphics.round_state.mode, RoundMode::Grid);
        // rtdg
        engine.op_rtdg().unwrap();
        assert_eq!(engine.graphics.round_state.mode, RoundMode::DoubleGrid);
        // rdtg
        engine.op_rdtg().unwrap();
        assert_eq!(engine.graphics.round_state.mode, RoundMode::DownToGrid);
        // rutg
        engine.op_rutg().unwrap();
        assert_eq!(engine.graphics.round_state.mode, RoundMode::UpToGrid);
        // roff
        engine.op_roff().unwrap();
        assert_eq!(engine.graphics.round_state.mode, RoundMode::Off);
        // sround
        engine.value_stack.push(0).unwrap();
        engine.op_sround().unwrap();
        assert_eq!(engine.graphics.round_state.mode, RoundMode::Super);
        // s45round
        engine.value_stack.push(0).unwrap();
        engine.op_s45round().unwrap();
        assert_eq!(engine.graphics.round_state.mode, RoundMode::Super45);
        // sloop
        engine.value_stack.push(10).unwrap();
        engine.op_sloop().unwrap();
        assert_eq!(engine.graphics.loop_counter, 10);
        // loop variable cannot be negative
        engine.value_stack.push(-10).unwrap();
        assert!(matches!(
            engine.op_sloop(),
            Err(HintErrorKind::NegativeLoopCounter)
        ));
        // smd
        engine.value_stack.push(64).unwrap();
        engine.op_smd().unwrap();
        assert_eq!(engine.graphics.min_distance, F26Dot6::from_bits(64));
        // scantype
        engine.value_stack.push(50).unwrap();
        engine.op_scantype().unwrap();
        assert_eq!(engine.graphics.scan_type, 50);
        // scvtci
        engine.value_stack.push(128).unwrap();
        engine.op_scvtci().unwrap();
        assert_eq!(engine.graphics.control_value_cutin, F26Dot6::from_bits(128));
        // sswci
        engine.value_stack.push(100).unwrap();
        engine.op_sswci().unwrap();
        assert_eq!(engine.graphics.single_width_cutin, F26Dot6::from_bits(100));
        // ssw
        engine.graphics.scale = 64;
        engine.value_stack.push(100).unwrap();
        engine.op_ssw().unwrap();
        assert_eq!(
            engine.graphics.single_width,
            F26Dot6::from_bits(math::mul(100, engine.graphics.scale))
        );
        // flipoff
        engine.op_flipoff().unwrap();
        assert!(!engine.graphics.auto_flip);
        // flipon
        engine.op_flipon().unwrap();
        assert!(engine.graphics.auto_flip);
        // sdb
        engine.value_stack.push(172).unwrap();
        engine.op_sdb().unwrap();
        assert_eq!(engine.graphics.delta_base, 172);
        // sds
        engine.value_stack.push(4).unwrap();
        engine.op_sds().unwrap();
        assert_eq!(engine.graphics.delta_shift, 4);
        // delta_shift has a max value of 6
        engine.value_stack.push(7).unwrap();
        assert!(matches!(
            engine.op_sds(),
            Err(HintErrorKind::InvalidStackValue(7))
        ));
    }

    #[test]
    fn instctrl() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        engine.program.initial = Program::ControlValue;
        // selectors 1..=3 are valid and values for each selector
        // can be 0, which disables the field, or 1 << (selector - 1) to
        // enable it
        for selector in 1..=3 {
            // enable first
            let enable_mask = (1 << (selector - 1)) as u8;
            engine.value_stack.push(enable_mask as i32).unwrap();
            engine.value_stack.push(selector).unwrap();
            engine.op_instctrl().unwrap();
            assert!(engine.graphics.instruct_control & enable_mask != 0);
            // now disable
            engine.value_stack.push(0).unwrap();
            engine.value_stack.push(selector).unwrap();
            engine.op_instctrl().unwrap();
            assert!(engine.graphics.instruct_control & enable_mask == 0);
        }
        // in glyph programs, selector 3 can be used to toggle
        // backward_compatibility
        engine.program.initial = Program::Glyph;
        // enabling this flag opts into "native ClearType mode"
        // which disables backward compatibility
        engine.value_stack.push((3 - 1) << 1).unwrap();
        engine.value_stack.push(3).unwrap();
        engine.op_instctrl().unwrap();
        assert!(!engine.graphics.backward_compatibility);
        // and disabling it enables backward compatibility
        engine.value_stack.push(0).unwrap();
        engine.value_stack.push(3).unwrap();
        engine.op_instctrl().unwrap();
        assert!(engine.graphics.backward_compatibility);
    }

    // Subtract with overflow caught by fuzzing when selector == 0
    // See <https://bugs.chromium.org/p/oss-fuzz/issues/detail?id=70426>
    // and <https://oss-fuzz.com/testcase?key=6243979511791616>
    #[test]
    fn instctrl_avoid_overflow() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        engine.program.initial = Program::ControlValue;
        engine.value_stack.push(0).unwrap();
        engine.value_stack.push(0).unwrap();
        engine.op_instctrl().unwrap();
    }

    #[test]
    fn scanctrl() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // Example modes from specification:
        // 0x0000   No dropout control is invoked
        engine.value_stack.push(0x0000).unwrap();
        engine.op_scanctrl().unwrap();
        assert!(!engine.graphics.scan_control);
        // 0x01FF   Always do dropout control
        engine.value_stack.push(0x01FF).unwrap();
        engine.op_scanctrl().unwrap();
        assert!(engine.graphics.scan_control);
        // 0x0A10   Do dropout control if the glyph is rotated and has less than 16 pixels per-em
        engine.value_stack.push(0x0A10).unwrap();
        engine.graphics.is_rotated = true;
        engine.graphics.ppem = 12;
        engine.op_scanctrl().unwrap();
        assert!(engine.graphics.scan_control);
    }
}
