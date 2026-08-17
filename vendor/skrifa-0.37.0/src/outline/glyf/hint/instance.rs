//! Instance state for TrueType hinting.

use super::{
    super::Outlines,
    cow_slice::CowSlice,
    definition::{Definition, DefinitionMap, DefinitionState},
    engine::Engine,
    error::HintError,
    graphics::RetainedGraphicsState,
    program::{Program, ProgramState},
    value_stack::ValueStack,
    zone::Zone,
    HintOutline, PointFlags, Target,
};
use alloc::vec::Vec;
use raw::{
    types::{F26Dot6, F2Dot14, Fixed, Point},
    TableProvider,
};

#[derive(Clone, Default)]
pub struct HintInstance {
    functions: Vec<Definition>,
    instructions: Vec<Definition>,
    cvt: Vec<i32>,
    storage: Vec<i32>,
    graphics: RetainedGraphicsState,
    twilight_scaled: Vec<Point<F26Dot6>>,
    twilight_original_scaled: Vec<Point<F26Dot6>>,
    twilight_flags: Vec<PointFlags>,
    axis_count: u16,
    max_stack: usize,
}

impl HintInstance {
    pub fn reconfigure(
        &mut self,
        outlines: &Outlines,
        scale: i32,
        ppem: i32,
        target: Target,
        coords: &[F2Dot14],
    ) -> Result<(), HintError> {
        self.setup(outlines, scale, coords);
        let twilight_contours = [self.twilight_scaled.len() as u16];
        let twilight = Zone::new(
            &[],
            &mut self.twilight_original_scaled,
            &mut self.twilight_scaled,
            &mut self.twilight_flags,
            &twilight_contours,
        );
        let glyph = Zone::default();
        let mut stack_buf = vec![0; self.max_stack];
        let value_stack = ValueStack::new(&mut stack_buf, false);
        let graphics = RetainedGraphicsState::new(scale, ppem, target);
        let mut engine = Engine::new(
            outlines,
            ProgramState::new(outlines.fpgm, outlines.prep, &[], Program::Font),
            graphics,
            DefinitionState::new(
                DefinitionMap::Mut(&mut self.functions),
                DefinitionMap::Mut(&mut self.instructions),
            ),
            CowSlice::new_mut(&mut self.cvt),
            CowSlice::new_mut(&mut self.storage),
            value_stack,
            twilight,
            glyph,
            self.axis_count,
            coords,
            false,
        );
        // Run the font program (fpgm)
        engine.run_program(Program::Font, false)?;
        // Run the control value program (prep)
        engine.run_program(Program::ControlValue, false)?;
        // Save the retained state from the CV program
        self.graphics = *engine.retained_graphics_state();
        Ok(())
    }

    /// Returns true if we should actually apply hinting.
    ///
    /// Hinting can be completely disabled by the control value program.
    pub fn is_enabled(&self) -> bool {
        // If bit 0 is set, disables hinting entirely
        self.graphics.instruct_control & 1 == 0
    }

    /// Returns true if backward compatibility mode has been activated
    /// by the hinter settings or the `prep` table.
    pub fn backward_compatibility(&self) -> bool {
        // Set backward compatibility mode
        if self.graphics.target.preserve_linear_metrics() {
            true
        } else if self.graphics.target.is_smooth() {
            (self.graphics.instruct_control & 0x4) == 0
        } else {
            false
        }
    }

    pub fn hint(
        &self,
        outlines: &Outlines,
        outline: &mut HintOutline,
        is_pedantic: bool,
    ) -> Result<(), HintError> {
        // Twilight zone
        let twilight_count = outline.twilight_scaled.len();
        let twilight_contours = [twilight_count as u16];
        outline
            .twilight_original_scaled
            .copy_from_slice(&self.twilight_original_scaled);
        outline
            .twilight_scaled
            .copy_from_slice(&self.twilight_scaled);
        outline.twilight_flags.copy_from_slice(&self.twilight_flags);
        let twilight = Zone::new(
            &[],
            outline.twilight_original_scaled,
            outline.twilight_scaled,
            outline.twilight_flags,
            &twilight_contours,
        );
        // Glyph zone
        let glyph = Zone::new(
            outline.unscaled,
            outline.original_scaled,
            outline.scaled,
            outline.flags,
            outline.contours,
        );
        let value_stack = ValueStack::new(outline.stack, is_pedantic);
        let cvt = CowSlice::new(&self.cvt, outline.cvt).unwrap();
        let storage = CowSlice::new(&self.storage, outline.storage).unwrap();
        let mut engine = Engine::new(
            outlines,
            ProgramState::new(
                outlines.fpgm,
                outlines.prep,
                outline.bytecode,
                Program::Glyph,
            ),
            self.graphics,
            DefinitionState::new(
                DefinitionMap::Ref(&self.functions),
                DefinitionMap::Ref(&self.instructions),
            ),
            cvt,
            storage,
            value_stack,
            twilight,
            glyph,
            self.axis_count,
            outline.coords,
            outline.is_composite,
        );
        engine
            .run_program(Program::Glyph, is_pedantic)
            .map_err(|mut e| {
                e.glyph_id = Some(outline.glyph_id);
                e
            })?;
        // If we're not running in backward compatibility mode, capture
        // modified phantom points.
        if !engine.backward_compatibility() {
            for (i, p) in (outline.scaled[outline.scaled.len() - 4..])
                .iter()
                .enumerate()
            {
                outline.phantom[i] = *p;
            }
        }
        Ok(())
    }

    /// Captures limits, resizes buffers and scales the CVT.
    fn setup(&mut self, outlines: &Outlines, scale: i32, coords: &[F2Dot14]) {
        let axis_count = outlines
            .gvar
            .as_ref()
            .map(|gvar| gvar.axis_count())
            .unwrap_or_default();
        self.functions.clear();
        self.functions
            .resize(outlines.max_function_defs as usize, Definition::default());
        self.instructions.resize(
            outlines.max_instruction_defs as usize,
            Definition::default(),
        );
        self.cvt.clear();
        let cvt = outlines.font.cvt().unwrap_or_default();
        if let Ok(cvar) = outlines.font.cvar() {
            // First accumulate all the deltas in 16.16
            self.cvt.resize(cvt.len(), 0);
            let _ = cvar.deltas(axis_count, coords, &mut self.cvt);
            // Now add the base CVT values
            for (value, base_value) in self.cvt.iter_mut().zip(cvt.iter()) {
                // Deltas are converted from 16.16 to 26.6
                // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttgxvar.c#L3822>
                let delta = Fixed::from_bits(*value).to_f26dot6().to_bits();
                let base_value = base_value.get() as i32 * 64;
                *value = base_value + delta;
            }
        } else {
            // CVT values are converted to 26.6 on load
            // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttpload.c#L350>
            self.cvt
                .extend(cvt.iter().map(|value| (value.get() as i32) * 64));
        }
        // More weird scaling. This is due to the fact that CVT values are
        // already in 26.6
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttobjs.c#L996>
        let scale = Fixed::from_bits(scale >> 6);
        for value in &mut self.cvt {
            *value = (Fixed::from_bits(*value) * scale).to_bits();
        }
        self.storage.clear();
        self.storage.resize(outlines.max_storage as usize, 0);
        let max_twilight_points = outlines.max_twilight_points as usize;
        self.twilight_scaled.clear();
        self.twilight_scaled
            .resize(max_twilight_points, Default::default());
        self.twilight_original_scaled.clear();
        self.twilight_original_scaled
            .resize(max_twilight_points, Default::default());
        self.twilight_flags.clear();
        self.twilight_flags
            .resize(max_twilight_points, Default::default());
        self.axis_count = axis_count;
        self.max_stack = outlines.max_stack_elements as usize;
        self.graphics = RetainedGraphicsState::default();
    }
}

#[cfg(test)]
impl HintInstance {
    /// Enable instruct control bit 1 which effectively disables hinting.
    ///
    /// This mimics what the `prep` table might do for various configurations
    /// and font sizes. Used for testing.    
    pub fn simulate_prep_flag_suppress_hinting(&mut self) {
        self.graphics.instruct_control |= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{super::super::Outlines, HintInstance};
    use read_fonts::{types::F2Dot14, FontRef};

    #[test]
    fn scaled_cvar_cvt() {
        let font = FontRef::new(font_test_data::CVAR).unwrap();
        let outlines = Outlines::new(&font).unwrap();
        let mut instance = HintInstance::default();
        let coords = [0.5, -0.5].map(F2Dot14::from_f32);
        let ppem = 16;
        // ppem * 64 / upem
        let scale = 67109;
        instance
            .reconfigure(&outlines, scale, ppem, Default::default(), &coords)
            .unwrap();
        let expected = [
            778, 10, 731, 0, 731, 10, 549, 10, 0, 0, 0, -10, 0, -10, -256, -10, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 95, 137, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 60, 0, 81, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(&instance.cvt, &expected);
    }
}
