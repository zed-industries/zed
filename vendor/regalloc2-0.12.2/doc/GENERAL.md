# regalloc2 Design Overview

This document describes the basic architecture of the regalloc2
register allocator. It describes the externally-visible interface:
input CFG, instructions, operands, with their invariants; meaning of
various parts of the output.
`ION.md` and `FASTALLOC.md` describe the specifics of the main Ion
allocator and the fast allocator, respectively.

# API, Input IR and Invariants

The toplevel API to regalloc2 consists of a single entry point `run()`
that takes a register environment, which specifies all physical
registers, and the input program. The function returns either an error
or an `Output` struct that provides allocations for each operand and a
vector of additional instructions (moves, loads, stores) to insert.

## Register Environment

The allocator takes a `MachineEnv` which specifies, for each of the
two register classes `Int` and `Float`, a vector of `PReg`s by index. A
`PReg` is nothing more than the class and index within the class; the
allocator does not need to know anything more.

The `MachineEnv` provides a vector of preferred and non-preferred
physical registers per class. Any register not in either vector will
not be allocated. Usually, registers that do not need to be saved in
the prologue if used (i.e., caller-save registers) are given in the
"preferred" vector. The environment also provides exactly one scratch
register per class. This register must not be in the preferred or
non-preferred vectors, and is used whenever a set of moves that need
to occur logically in parallel have a cycle (for a simple example,
consider a swap `r0, r1 := r1, r0`).

With some more work, we could potentially remove the need for the
scratch register by requiring support for an additional edit type from
the client ("swap"), but we have not pursued this.

## CFG and Instructions

The allocator operates on an input program that is in a standard CFG
representation: the function body is a sequence of basic blocks, and
each block has a sequence of instructions and zero or more
successors. The allocator also requires the client to provide
predecessors for each block, and these must be consistent with the
successors.

Instructions are opaque to the allocator except for a few important
bits: (1) `is_ret` (is a return instruction); (2) `is_branch` (is a
branch instruction); and (3) a vector of Operands, covered below.
Every block must end in a return or branch.

Both instructions and blocks are named by indices in contiguous index
spaces. A block's instructions must be a contiguous range of
instruction indices, and block i's first instruction must come
immediately after block i-1's last instruction.

The CFG must have *no critical edges*. A critical edge is an edge from
block A to block B such that A has more than one successor *and* B has
more than one predecessor. For this definition, the entry block has an
implicit predecessor, and any block that ends in a return has an
implicit successor.

Note that there are *no* requirements related to the ordering of
blocks, and there is no requirement that the control flow be
reducible. Some *heuristics* used by the allocator will perform better
if the code is reducible and ordered in reverse postorder (RPO),
however: in particular, (1) this interacts better with the
contiguous-range-of-instruction-indices live range representation that
we use, and (2) the "approximate loop depth" metric will actually be
exact if both these conditions are met.

## Operands and VRegs

Every instruction operates on values by way of `Operand`s. An operand
consists of the following fields:

- VReg, or virtual register. *Every* operand mentions a virtual
  register, even if it is constrained to a single physical register in
  practice. This is because we track liveranges uniformly by vreg.

- Policy, or "constraint". Every reference to a vreg can apply some
  constraint to the vreg at that point in the program. Valid policies are:

  - Any location;
  - Any register of the vreg's class;
  - Any stack slot;
  - A particular fixed physical register; or
  - For a def (output), a *reuse* of an input register.

- The "kind" of reference to this vreg: Def, Use, Mod. A def
  (definition) writes to the vreg, and disregards any possible earlier
  value. A mod (modify) reads the current value then writes a new
  one. A use simply reads the vreg's value.

- The position: before or after the instruction.
  - Note that to have a def (output) register available in a way that
    does not conflict with inputs, the def should be placed at the
    "before" position. Similarly, to have a use (input) register
    available in a way that does not conflict with outputs, the use
    should be placed at the "after" position.

VRegs, or virtual registers, are specified by an index and a register
class (Float or Int). The classes are not given separately; they are
encoded on every mention of the vreg. (In a sense, the class is an
extra index bit, or part of the register name.) The input function
trait does require the client to provide the exact vreg count,
however.

Implementation note: both vregs and operands are bit-packed into
u32s. This is essential for memory-efficiency. As a result of the
operand bit-packing in particular (including the policy constraints!),
the allocator supports up to 2^21 (2M) vregs per function, and 2^6
(64) physical registers per class. Later we will also see a limit of
2^20 (1M) instructions per function. These limits are considered
sufficient for the anticipated use-cases (e.g., compiling Wasm, which
also has function-size implementation limits); for larger functions,
it is likely better to use a simpler register allocator in any case.

## Reuses and Two-Address ISAs

Some instruction sets primarily have instructions that name only two
registers for a binary operator, rather than three: both registers are
inputs, and the result is placed in one of the registers, clobbering
its original value. The most well-known modern example is x86. It is
thus imperative that we support this pattern well in the register
allocator.

This instruction-set design is somewhat at odds with an SSA
representation, where a value cannot be redefined.

Thus, the allocator supports a useful fiction of sorts: the
instruction can be described as if it has three register mentions --
two inputs and a separate output -- and neither input will be
clobbered. The output, however, is special: its register-placement
policy is "reuse input i" (where i == 0 or 1). The allocator
guarantees that the register assignment for that input and the output
will be the same, so the instruction can use that register as its
"modifies" operand. If the input is needed again later, the allocator
will take care of the necessary copying.

We will see below how the allocator makes this work by doing some
preprocessing so that the core allocation algorithms do not need to
worry about this constraint.

## SSA

regalloc2 takes an SSA IR as input, where the usual definitions apply:
every vreg is defined exactly once, and every vreg use is dominated by
its one def. (Using blockparams means that we do not need additional
conditions for phi-nodes.)

## Block Parameters

Every block can have *block parameters*, and a branch to a block with
block parameters must provide values for those parameters via
operands. When a branch has more than one successor, it provides
separate operands for each possible successor. These block parameters
are equivalent to phi-nodes; we chose this representation because they
are in many ways a more consistent representation of SSA.

To see why we believe block parameters are a slightly nicer design
choice than use of phi nodes, consider: phis are special
pseudoinstructions that must come first in a block, are all defined in
parallel, and whose uses occur on the edge of a particular
predecessor. All of these facts complicate any analysis that scans
instructions and reasons about uses and defs. It is much closer to the
truth to actually put those uses *in* the predecessor, on the branch,
and put all the defs at the top of the block as a separate kind of
def. The tradeoff is that a vreg's def now has two possibilities --
ordinary instruction def or blockparam def -- but this is fairly
reasonable to handle.

## Output

The allocator produces two main data structures as output: an array of
`Allocation`s and a sequence of edits. Some other miscellaneous data is also
provided.

### Allocations

The allocator provides an array of `Allocation` values, one per
`Operand`. Each `Allocation` has a kind and an index. The kind may
indicate that this is a physical register or a stack slot, and the
index gives the respective register or slot. All allocations will
conform to the constraints given, and will faithfully preserve the
dataflow of the input program.

### Inserted Moves

In order to implement the necessary movement of data between
allocations, the allocator needs to insert moves at various program
points.

The vector of inserted moves contains tuples that name a program point
and an "edit". The edit is either a move, from one `Allocation` to
another, or else a kind of metadata used by the checker to know which
VReg is live in a given allocation at any particular time. The latter
sort of edit can be ignored by a backend that is just interested in
generating machine code.

Note that the allocator will never generate a move from one stackslot
directly to another, by design. Instead, if it needs to do so, it will
make use of the scratch register. (Sometimes such a move occurs when
the scratch register is already holding a value, e.g. to resolve a
cycle of moves; in this case, it will allocate another spillslot and
spill the original scratch value around the move.)

Thus, the single "edit" type can become either a register-to-register
move, a load from a stackslot into a register, or a store from a
register into a stackslot.

