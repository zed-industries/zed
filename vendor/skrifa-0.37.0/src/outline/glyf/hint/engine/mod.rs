//! TrueType bytecode interpreter.

mod arith;
mod control_flow;
mod cvt;
mod data;
mod definition;
mod delta;
mod dispatch;
mod graphics;
mod logical;
mod misc;
mod outline;
mod round;
mod stack;
mod storage;

use read_fonts::{
    tables::glyf::bytecode::Instruction,
    types::{F26Dot6, F2Dot14, Point},
};

use super::{
    super::Outlines,
    cvt::Cvt,
    definition::DefinitionState,
    error::{HintError, HintErrorKind},
    graphics::{GraphicsState, RetainedGraphicsState},
    math,
    program::ProgramState,
    storage::Storage,
    value_stack::ValueStack,
    zone::Zone,
};

pub type OpResult = Result<(), HintErrorKind>;

/// TrueType bytecode interpreter.
pub struct Engine<'a> {
    program: ProgramState<'a>,
    graphics: GraphicsState<'a>,
    definitions: DefinitionState<'a>,
    cvt: Cvt<'a>,
    storage: Storage<'a>,
    value_stack: ValueStack<'a>,
    loop_budget: LoopBudget,
    axis_count: u16,
    coords: &'a [F2Dot14],
}

impl<'a> Engine<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        outlines: &Outlines,
        program: ProgramState<'a>,
        graphics: RetainedGraphicsState,
        definitions: DefinitionState<'a>,
        cvt: impl Into<Cvt<'a>>,
        storage: impl Into<Storage<'a>>,
        value_stack: ValueStack<'a>,
        twilight: Zone<'a>,
        glyph: Zone<'a>,
        axis_count: u16,
        coords: &'a [F2Dot14],
        is_composite: bool,
    ) -> Self {
        let point_count = if glyph.points.is_empty() {
            None
        } else {
            Some(glyph.points.len())
        };
        let graphics = GraphicsState {
            retained: graphics,
            zones: [twilight, glyph],
            is_composite,
            ..Default::default()
        };
        Self {
            program,
            graphics,
            definitions,
            cvt: cvt.into(),
            storage: storage.into(),
            value_stack,
            loop_budget: LoopBudget::new(outlines, point_count),
            axis_count,
            coords,
        }
    }

    pub fn backward_compatibility(&self) -> bool {
        self.graphics.backward_compatibility
    }

    pub fn retained_graphics_state(&self) -> &RetainedGraphicsState {
        &self.graphics.retained
    }
}

/// Tracks budgets for loops to limit execution time.
struct LoopBudget {
    /// Maximum number of times we can do backward jumps or
    /// loop calls.
    limit: usize,
    /// Current number of backward jumps executed.
    backward_jumps: usize,
    /// Current number of loop call iterations executed.
    loop_calls: usize,
}

impl LoopBudget {
    fn new(outlines: &Outlines, point_count: Option<usize>) -> Self {
        let cvt_len = outlines.cvt_len as usize;
        // Compute limits for loop calls and backward jumps.
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6955>
        let limit = if let Some(point_count) = point_count {
            (point_count * 10).max(50) + (cvt_len / 10).max(50)
        } else {
            300 + 22 * cvt_len
        };
        // FreeType has two variables for neg_jump_counter_max and
        // loopcall_counter_max but sets them to the same value so
        // we'll just use a single limit.
        Self {
            limit,
            backward_jumps: 0,
            loop_calls: 0,
        }
    }

    fn reset(&mut self) {
        self.backward_jumps = 0;
        self.loop_calls = 0;
    }

    fn doing_backward_jump(&mut self) -> Result<(), HintErrorKind> {
        self.backward_jumps += 1;
        if self.backward_jumps > self.limit {
            Err(HintErrorKind::ExceededExecutionBudget)
        } else {
            Ok(())
        }
    }

    fn doing_loop_call(&mut self, count: usize) -> Result<(), HintErrorKind> {
        self.loop_calls += count;
        if self.loop_calls > self.limit {
            Err(HintErrorKind::ExceededExecutionBudget)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
use mock::MockEngine;

#[cfg(test)]
mod mock {
    use super::{
        super::{
            cow_slice::CowSlice,
            definition::{Definition, DefinitionMap, DefinitionState},
            program::{Program, ProgramState},
            zone::Zone,
            Point, PointFlags,
        },
        Engine, F26Dot6, GraphicsState, LoopBudget, ValueStack,
    };

    /// Mock engine for testing.
    pub(super) struct MockEngine {
        cvt_storage: Vec<i32>,
        value_stack: Vec<i32>,
        definitions: Vec<Definition>,
        unscaled: Vec<Point<i32>>,
        points: Vec<Point<F26Dot6>>,
        point_flags: Vec<PointFlags>,
        contours: Vec<u16>,
        twilight: Vec<Point<F26Dot6>>,
        twilight_flags: Vec<PointFlags>,
    }

    impl MockEngine {
        pub fn new() -> Self {
            Self {
                cvt_storage: vec![0; 32],
                value_stack: vec![0; 32],
                definitions: vec![Default::default(); 8],
                unscaled: vec![Default::default(); 32],
                points: vec![Default::default(); 64],
                point_flags: vec![Default::default(); 32],
                contours: vec![31],
                twilight: vec![Default::default(); 32],
                twilight_flags: vec![Default::default(); 32],
            }
        }

        pub fn engine(&mut self) -> Engine {
            let font_code = &[];
            let cv_code = &[];
            let glyph_code = &[];
            let (cvt, storage) = self.cvt_storage.split_at_mut(16);
            let (function_defs, instruction_defs) = self.definitions.split_at_mut(5);
            let definition = DefinitionState::new(
                DefinitionMap::Mut(function_defs),
                DefinitionMap::Mut(instruction_defs),
            );
            for (i, point) in self.unscaled.iter_mut().enumerate() {
                let i = i as i32;
                point.x = 57 + i * 2;
                point.y = -point.x * 3;
            }
            let (points, original) = self.points.split_at_mut(32);
            let glyph_zone = Zone::new(
                &self.unscaled,
                original,
                points,
                &mut self.point_flags,
                &self.contours,
            );
            let (points, original) = self.twilight.split_at_mut(16);
            let twilight_zone = Zone::new(&[], original, points, &mut self.twilight_flags, &[]);
            let mut graphics_state = GraphicsState {
                zones: [twilight_zone, glyph_zone],
                ..Default::default()
            };
            graphics_state.update_projection_state();
            Engine {
                graphics: graphics_state,
                cvt: CowSlice::new_mut(cvt).into(),
                storage: CowSlice::new_mut(storage).into(),
                value_stack: ValueStack::new(&mut self.value_stack, false),
                program: ProgramState::new(font_code, cv_code, glyph_code, Program::Font),
                loop_budget: LoopBudget {
                    limit: 10,
                    backward_jumps: 0,
                    loop_calls: 0,
                },
                definitions: definition,
                axis_count: 0,
                coords: &[],
            }
        }
    }

    impl Default for MockEngine {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Engine<'_> {
        /// Helper to push values to the stack, invoke a callback and check
        /// the expected result.    
        pub(super) fn test_exec(
            &mut self,
            push: &[i32],
            expected_result: impl Into<i32>,
            mut f: impl FnMut(&mut Engine),
        ) {
            for &val in push {
                self.value_stack.push(val).unwrap();
            }
            f(self);
            assert_eq!(self.value_stack.pop().ok(), Some(expected_result.into()));
        }
    }
}
