# Fastalloc Design Overview

Fastalloc is a register allocator made specifically for fast
compile times. It's based on the reverse linear scan register
allocation/SSRA algorithm.
This document describes the data structures used and the allocation steps.

# Data Structures

The main data structures that Fastalloc uses to track its state are
described below.

## Current VReg Allocations (`vreg_allocs`)

This is a vector that is used to hold the current allocation for every
VReg during execution.

## VReg Spillslots (`vreg_spillslots`)

Whenever a VReg needs a spillslot, a dedicated slot is allocated for it.
This vector is where all VReg's spillslots are stored.

## Live VRegs (`live_vregs`)

Live VReg information is kept in a `VRegSet`, a doubly linked list
based on a vector. This is used for quick insertion, removal, and
iteration.

## Least Recently Used Caches (`lrus`)

Every register class (int, float, and vector) has its own LRU and they
are stored together in an array: `lrus`. An LRU is represented similarly
to a `VRegSet`: it's a circular, doubly-linked list based on a vector.

The last PReg in an LRU is the least recently allocated PReg:

most recently used PReg (head) -> 2nd MRU PReg -> ... -> LRU PReg

## Current VReg In PReg Info (`vreg_in_preg`)

During allocation, it's necessary to determine which VReg is in a PReg
to generate the right move(s) for eviction.
`vreg_in_preg` is a vector that stores this information.

## Available PRegs For Use In Instruction (`available_pregs`)

This is a 2-tuple of `PRegSet`s, a bitset of physical registers, one for
the instruction's early phase and one for the late phase.
They are used to determine which registers are available for use in the
early/late phases of an instruction.

Prior to the beginning of any instruction's allocation, this set is reset
to include all allocatable physical registers, some of which may already
contain a VReg.

## VReg Liverange Location Info (`vreg_to_live_inst_range`)

This is a vector of 3-tuples containing the beginning and the end
of all VReg's liveranges, along with an allocation they are guaranteed
to be in throughout that liverange.
This is used to build the debug locations vector after allocation
is complete.

# Allocation Process Breakdown

Allocation proceeds in reverse: from the last block to the first block,
and in each block: from the last instruction to the first instruction.

The allocation for each operand in an instruction can be viewed to happen
in four phases: selection, assignment, eviction, and edit insertion.

## Allocation Phase: Selection

In this phase, a PReg is selected from `available_pregs` for the 
operand based on the operand constraints. Depending on the operand's 
position the selected PReg is removed from either the early or late 
phase or both, indicating that the PReg is no longer available for 
allocation by other operands in that phase.

## Allocation Phase: Assignment

In this phase, the selected PReg is set as the allocation for 
the operand in the final output.

## Allocation Phase: Eviction

In this phase, the previous VReg in the allocation assigned to 
an operand is evicted, if any.

During eviction, a dedicated spillslot is allocated for the evicted 
VReg and an edit is inserted after the instruction to move from the
slot to the allocation it's expected to be in after the instruction.

## Allocation Phase: Edit Insertion

In this phase, edits are inserted to ensure that the dataflow from
before the instruction to the selected allocation to after
the instruction remain correct.

# Invariants

Some invariants that remain true throughout execution:

1. During processing, the allocation of a VReg at any point in time
as indicated in `vreg_allocs` changes exactly twice or thrice.
Initially, it is set to none. When it's allocated, it is
changed to that allocation. After this, it doesn't change unless 
it's evicted or spilled across a block boundary;
if it is, then its current allocation will change to its dedicated 
spillslot. After this, it doesn't change again until its definition 
is reached and it's deallocated, during which its `vreg_allocs` 
entry is set to none. The only exception is block parameters that 
are never used: these are never allocated.

2. A virtual register that outlives the block it was defined in will 
be in its dedicated spillslot by the end of the block.

3. At the end of a block, before edits are inserted to move values 
from branch arguments to block parameters spillslots, all branch 
arguments will be in their dedicated spillslots.

4. At the beginning of a block, all branch parameters and livein 
virtual registers will be in their dedicated spillslots.

# Instruction Allocation

To allocate a single instruction, the first step is to reset the
`available_pregs` sets to all allocatable PRegs.

Next, the selection phase is carried out for all operands with
fixed register constraints: the registers they are constrained to use are
marked as unavailable in the `available_pregs` set, depending on the
phase that they are valid in. If the operand is an early use or late
def operand, then the register will be marked as unavailable in the
early set or late set, respectively. Otherwise, the PReg is marked
as unavailable in both the early and late sets, because a PReg
assigned to an early def or late use operand cannot be reused by another
operand in the same instruction.

After selection for fixed register operands, the eviction phase is 
carried out for fixed register operands. Any VReg in their selected
registers, indicated by `vreg_in_preg`, is evicted: a dedicated 
spillslot is allocated for the VReg (if it doesn't have one already),
an edit is inserted to move from the slot to the PReg, which is where
the VReg expected to be after the instruction, and its current
allocation in `vreg_allocs` is set to the spillslot.

Next, all clobbers are removed from the early and late `available_pregs` 
sets to avoid allocating a clobber to a def.

Next, the selection, assignment, eviction, and edit insertion phases are 
carried out for all def operands. When each def operand's allocation is
complete, the def operands is immediately freed, marking the end of the
VReg's liverange. It is removed from the  `live_vregs` set, its allocation
in `vreg_allocs` is set to none, and if it was in a PReg, that PReg's
entry in `vreg_in_preg` is set to none. The selection and eviction phases
are omitted if the operand has a fixed constraint, as those phases have
already been carried out.

Next, the selection, assignment, and eviction phases are carried out for all
use operands. As with def operands, the selection and eviction phases are 
omitted if the operand has a fixed constraint, as those phases have already
been carried out.

Then the edit insertion phase is carried out for all use operands.

Lastly, if the instruction being processed is a branch instruction, the
parallel move resolver is used to insert edits before the instruction
to move from the branch arguments spillslots to the block parameter
spillslots.

## Operand Allocation

During the allocation of an operand, a check is first made to 
see if the VReg's current allocation as indicated in 
`vreg_allocs` is within the operand constraints.

If it is, the assignment phase is carried out, setting the final
allocation output's entry for that operand to the allocation.
The selection phase is carried out, marking the PReg 
(if the allocation is a PReg) as unavailable in the respective
early/late sets. The state of the LRUs is also updated to reflect 
the new most recently used PReg.
No eviction needs to be done since the VReg is already in the 
allocation and no edit insertion needs to be done either.

On the other hand, if the VReg's current allocation is not within
constraints, the selection and eviction phases are carried out for
non-fixed operands. First, a set of PRegs that can be drawn from is
created from `available_pregs`. For early uses and late defs,
this draw-from set is the early set or late set, respectively.
For late uses and early defs, the draw-from set is an intersection
of the available early and late sets (because a PReg used for a late
use can't be reassigned to another operand in the early phase;
likewise, a PReg used for an early def can't be reassigned to another
operand in the late phase).
The LRU for the VReg's regclass is then traversed from the end to find
the least recently used PReg in the draw-from set. Once a PReg is found,
it is marked as the most recently used in the LRU, unavailable in the
`available_pregs` sets, and whatever VReg was in it before is evicted.

The assignment phase is carried out next. The final allocation for the
operand is set to the selected register.

If the newly allocated operand has not been allocated before, that is,
this is the first use/def of the VReg encountered; the VReg is
inserted into `live_vregs` and marked as the value in the allocated
PReg in `vreg_in_preg`.

Otherwise, if the VReg has been allocated before, then an edit will need
to be inserted to ensure that the dataflow remains correct.
The edit insertion phase is now carried out if the operand is a def
operand: an edit is inserted after the instruction to move from the
new allocation to the allocation it's expected to be in after the
instruction.

The edit insertion phase for use operands is done after all operands
have been processed. Edits are inserted to move from the current
allocations in `vreg_allocs` to the final allocated position before
the instruction. This is to account for the possibility of multiple
uses of the same operand in the instruction.

## Reuse Operands

Reuse def operands are handled by creating a new operand identical to the
reuse def, except that its constraints are the constraints of the
reused input and allocating that in its place.

Reused inputs are handled by creating a new operand with a fixed register
constraint to use whatever register was assigned to the reuse def.

Because of the way reuse operands and reused inputs are handled, when
selecting a register for an early-use operand with a fixed constraint,
the PReg is also marked as unavailable in the `available_pregs` late 
set if the operand is a reused input. And when selecting a register 
for reuse def operands, the selected register is marked as unavailable 
in the `available_pregs` early set.

## VReg Spillslots

Whenever a VReg needs a spillslot, a suitable one is allocated and
marked as the VReg's dedicated spillslot in `vreg_spillslots`.
If a VReg never needs a spillslot, none is allocated for it.
To ensure that a VReg will always be in its spillslot when expected,
during the processing of a def operand, before it's deallocated,
an edit is inserted to move from its current allocation as indicated
in `vreg_allocs` to its dedicated spillslot, if one is present in
`vreg_spillslots`.

## Branch Instructions

As an invariant, all branch arguments will be in their dedicated
spillslots at the end of the block before edits are inserted to
move from those spillslots to the block parameter spillslots
of the successor blocks.

If a branch argument is already in an allocation that isn't
its spillslot (this could happen if the branch argument is used
as an operand in the same instruction, because all normal
instruction processing is completed before branch-specific
processing), then an edit is inserted
to move from the spillslot to that allocation and its current
allocation in `vreg_allocs` is set to the spillslot.

It's after these edits have been inserted that the parallel move
resolver is then used to generate and insert edits to move from
those spillslots to the spillslots of the block parameters.

# Across Blocks

When a block completes processing, some VRegs will still be live.
These VRegs are either block parameters or livein VRegs.
As an invariant, prior to the first instruction in a block, all
block parameters and livein VRegs will be in their dedicated spillslots.

To maintain this invariant, after a block completes processing, edits
are inserted at the beginning of the block to move from the block
parameter and livein spillslots to the allocation they are expected
to be in from the first instruction.
All block parameters are freed, just like defs, and liveins' current
allocations in `vreg_allocs` are set to their spillslots.

# Edits Order

`regalloc2`'s outward interface guarantees that edits are in
sorted order. Since allocation proceeds in reverse, all edits
are also added in reverse. After all blocks have completed
processing the edits are simply reversed to put it in the
correct order.

One of the reasons why the allocation order proceeds the way it
does is because of this edit-order constraint. All edits that
occur after the instruction must be inserted before all edits
that occur before the instruction.

# Debug Info

After all blocks have completed processing, the debug locations
vector is built.
The information it's built from is assembled from liverange info 
that is tracked throughout the allocation.
Whenever a VReg is allocated for the first time, its liverange end
is saved in the VReg's slot in the `vreg_to_live_inst_range`
vector. Whenever a VReg's definition is encountered, its liverange
beginning is saved, too. And the allocation it will be in
throughout that range is also saved alongside.

To determine the allocation the VReg will be in throughout the 
liverange, the first invariant is used: the first time a VReg
is allocated, its current allocation in `vreg_allocs` doesn't
change unless its evicted or spilled across block boundaries.
Using this info, if by the time the def of a VReg is allocated,
that VReg has no dedicated spillslot,
that implies that the VReg was never evicted or spilled, so whatever
value its `vreg_allocs` entry says is the location it will be in
throughout its liverange. Otherwise, if it has a spillslot
allocated to it, that implies that the VReg was either evicted
at some point or it was a livein of a predecessor or a block parameter.
Either way, since all spillslots are dedicated to their respective VRegs,
it is safe to record the spillslot as the allocation for the
`vreg_to_live_inst_range` info.
