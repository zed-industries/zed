/*!
Generate SPIR-V conditional structures.

Builders for `if` structures with `and`s.

The types in this module track the information needed to emit SPIR-V code
for complex conditional structures, like those whose conditions involve
short-circuiting 'and' and 'or' structures. These track labels and can emit
`OpPhi` instructions to merge values produced along different paths.

This currently only supports exactly the forms Naga uses, so it doesn't
support `or` or `else`, and only supports zero or one merged values.

Naga needs to emit code roughly like this:

```ignore

    value = DEFAULT;
    if COND1 && COND2 {
        value = THEN_VALUE;
    }
    // use value

```

Assuming `ctx` and `block` are a mutable references to a [`BlockContext`]
and the current [`Block`], and `merge_type` is the SPIR-V type for the
merged value `value`, we can build SPIR-V for the code above like so:

```ignore

    let cond = Selection::start(block, merge_type);
        // ... compute `cond1` ...
    cond.if_true(ctx, cond1, DEFAULT);
        // ... compute `cond2` ...
    cond.if_true(ctx, cond2, DEFAULT);
        // ... compute THEN_VALUE
    let merged_value = cond.finish(ctx, THEN_VALUE);

```

After this, `merged_value` is either `DEFAULT` or `THEN_VALUE`, depending on
the path by which the merged block was reached.

This takes care of writing all branch instructions, including an
`OpSelectionMerge` annotation in the header block; starting new blocks and
assigning them labels; and emitting the `OpPhi` that gathers together the
right sources for the merged values, for every path through the selection
construct.

When there is no merged value to produce, you can pass `()` for `merge_type`
and the merge values. In this case no `OpPhi` instructions are produced, and
the `finish` method returns `()`.

To enforce proper nesting, a `Selection` takes ownership of the `&mut Block`
pointer for the duration of its lifetime. To obtain the block for generating
code in the selection's body, call the `Selection::block` method.
*/

use alloc::{vec, vec::Vec};

use spirv::Word;

use super::{Block, BlockContext, Instruction};

/// A private struct recording what we know about the selection construct so far.
pub(super) struct Selection<'b, M: MergeTuple> {
    /// The block pointer we're emitting code into.
    block: &'b mut Block,

    /// The label of the selection construct's merge block, or `None` if we
    /// haven't yet written the `OpSelectionMerge` merge instruction.
    merge_label: Option<Word>,

    /// A set of `(VALUES, PARENT)` pairs, used to build `OpPhi` instructions in
    /// the merge block. Each `PARENT` is the label of a predecessor block of
    /// the merge block. The corresponding `VALUES` holds the ids of the values
    /// that `PARENT` contributes to the merged values.
    ///
    /// We emit all branches to the merge block, so we know all its
    /// predecessors. And we refuse to emit a branch unless we're given the
    /// values the branching block contributes to the merge, so we always have
    /// everything we need to emit the correct phis, by construction.
    values: Vec<(M, Word)>,

    /// The types of the values in each element of `values`.
    merge_types: M,
}

impl<'b, M: MergeTuple> Selection<'b, M> {
    /// Start a new selection construct.
    ///
    /// The `block` argument indicates the selection's header block.
    ///
    /// The `merge_types` argument should be a `Word` or tuple of `Word`s, each
    /// value being the SPIR-V result type id of an `OpPhi` instruction that
    /// will be written to the selection's merge block when this selection's
    /// [`finish`] method is called. This argument may also be `()`, for
    /// selections that produce no values.
    ///
    /// (This function writes no code to `block` itself; it simply constructs a
    /// fresh `Selection`.)
    ///
    /// [`finish`]: Selection::finish
    pub(super) const fn start(block: &'b mut Block, merge_types: M) -> Self {
        Selection {
            block,
            merge_label: None,
            values: vec![],
            merge_types,
        }
    }

    pub(super) const fn block(&mut self) -> &mut Block {
        self.block
    }

    /// Branch to a successor block if `cond` is true, otherwise merge.
    ///
    /// If `cond` is false, branch to the merge block, using `values` as the
    /// merged values. Otherwise, proceed to a new block.
    ///
    /// The `values` argument must be the same shape as the `merge_types`
    /// argument passed to `Selection::start`.
    pub(super) fn if_true(&mut self, ctx: &mut BlockContext, cond: Word, values: M) {
        self.values.push((values, self.block.label_id));

        let merge_label = self.make_merge_label(ctx);
        let next_label = ctx.gen_id();
        ctx.function.consume(
            core::mem::replace(self.block, Block::new(next_label)),
            Instruction::branch_conditional(cond, next_label, merge_label),
        );
    }

    /// Emit an unconditional branch to the merge block, and compute merged
    /// values.
    ///
    /// Use `final_values` as the merged values contributed by the current
    /// block, and transition to the merge block, emitting `OpPhi` instructions
    /// to produce the merged values. This must be the same shape as the
    /// `merge_types` argument passed to [`Selection::start`].
    ///
    /// Return the SPIR-V ids of the merged values. This value has the same
    /// shape as the `merge_types` argument passed to `Selection::start`.
    pub(super) fn finish(self, ctx: &mut BlockContext, final_values: M) -> M {
        match self {
            Selection {
                merge_label: None, ..
            } => {
                // We didn't actually emit any branches, so `self.values` must
                // be empty, and `final_values` are the only sources we have for
                // the merged values. Easy peasy.
                final_values
            }

            Selection {
                block,
                merge_label: Some(merge_label),
                mut values,
                merge_types,
            } => {
                // Emit the final branch and transition to the merge block.
                values.push((final_values, block.label_id));
                ctx.function.consume(
                    core::mem::replace(block, Block::new(merge_label)),
                    Instruction::branch(merge_label),
                );

                // Now that we're in the merge block, build the phi instructions.
                merge_types.write_phis(ctx, block, &values)
            }
        }
    }

    /// Return the id of the merge block, writing a merge instruction if needed.
    fn make_merge_label(&mut self, ctx: &mut BlockContext) -> Word {
        match self.merge_label {
            None => {
                let merge_label = ctx.gen_id();
                self.block.body.push(Instruction::selection_merge(
                    merge_label,
                    spirv::SelectionControl::NONE,
                ));
                self.merge_label = Some(merge_label);
                merge_label
            }
            Some(merge_label) => merge_label,
        }
    }
}

/// A trait to help `Selection` manage any number of merged values.
///
/// Some selection constructs, like a `ReadZeroSkipWrite` bounds check on a
/// [`Load`] expression, produce a single merged value. Others produce no merged
/// value, like a bounds check on a [`Store`] statement.
///
/// To let `Selection` work nicely with both cases, we let the merge type
/// argument passed to [`Selection::start`] be any type that implements this
/// `MergeTuple` trait. `MergeTuple` is then implemented for `()`, `Word`,
/// `(Word, Word)`, and so on.
///
/// A `MergeTuple` type can represent either a bunch of SPIR-V types or values;
/// the `merge_types` argument to `Selection::start` are type ids, whereas the
/// `values` arguments to the [`if_true`] and [`finish`] methods are value ids.
/// The set of merged value returned by `finish` is a tuple of value ids.
///
/// In fact, since Naga only uses zero- and single-valued selection constructs
/// at present, we only implement `MergeTuple` for `()` and `Word`. But if you
/// add more cases, feel free to add more implementations. Once const generics
/// are available, we could have a single implementation of `MergeTuple` for all
/// lengths of arrays, and be done with it.
///
/// [`Load`]: crate::Expression::Load
/// [`Store`]: crate::Statement::Store
/// [`if_true`]: Selection::if_true
/// [`finish`]: Selection::finish
pub(super) trait MergeTuple: Sized {
    /// Write OpPhi instructions for the given set of predecessors.
    ///
    /// The `predecessors` vector should be a vector of `(LABEL, VALUES)` pairs,
    /// where each `VALUES` holds the values contributed by the branch from
    /// `LABEL`, which should be one of the current block's predecessors.
    fn write_phis(
        self,
        ctx: &mut BlockContext,
        block: &mut Block,
        predecessors: &[(Self, Word)],
    ) -> Self;
}

/// Selections that produce a single merged value.
///
/// For example, `ImageLoad` with `BoundsCheckPolicy::ReadZeroSkipWrite` either
/// returns a texel value or zeros.
impl MergeTuple for Word {
    fn write_phis(
        self,
        ctx: &mut BlockContext,
        block: &mut Block,
        predecessors: &[(Word, Word)],
    ) -> Word {
        let merged_value = ctx.gen_id();
        block
            .body
            .push(Instruction::phi(self, merged_value, predecessors));
        merged_value
    }
}

/// Selections that produce no merged values.
///
/// For example, `ImageStore` under `BoundsCheckPolicy::ReadZeroSkipWrite`
/// either does the store or skips it, but in neither case does it produce a
/// value.
impl MergeTuple for () {
    /// No phis need to be generated.
    fn write_phis(self, _: &mut BlockContext, _: &mut Block, _: &[((), Word)]) {}
}
