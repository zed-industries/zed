//! Cost functions for egraph representation.

use crate::ir::Opcode;

/// A cost of computing some value in the program.
///
/// Costs are measured in an arbitrary union that we represent in a
/// `u32`. The ordering is meant to be meaningful, but the value of a
/// single unit is arbitrary (and "not to scale"). We use a collection
/// of heuristics to try to make this approximation at least usable.
///
/// We start by defining costs for each opcode (see `pure_op_cost`
/// below). The cost of computing some value, initially, is the cost
/// of its opcode, plus the cost of computing its inputs.
///
/// We then adjust the cost according to loop nests: for each
/// loop-nest level, we multiply by 1024. Because we only have 32
/// bits, we limit this scaling to a loop-level of two (i.e., multiply
/// by 2^20 ~= 1M).
///
/// Arithmetic on costs is always saturating: we don't want to wrap
/// around and return to a tiny cost when adding the costs of two very
/// expensive operations. It is better to approximate and lose some
/// precision than to lose the ordering by wrapping.
///
/// Finally, we reserve the highest value, `u32::MAX`, as a sentinel
/// that means "infinite". This is separate from the finite costs and
/// not reachable by doing arithmetic on them (even when overflowing)
/// -- we saturate just *below* infinity. (This is done by the
/// `finite()` method.) An infinite cost is used to represent a value
/// that cannot be computed, or otherwise serve as a sentinel when
/// performing search for the lowest-cost representation of a value.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Cost(u32);

impl core::fmt::Debug for Cost {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if *self == Cost::infinity() {
            write!(f, "Cost::Infinite")
        } else {
            f.debug_struct("Cost::Finite")
                .field("op_cost", &self.op_cost())
                .field("depth", &self.depth())
                .finish()
        }
    }
}

impl Ord for Cost {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We make sure that the high bits are the op cost and the low bits are
        // the depth. This means that we can use normal integer comparison to
        // order by op cost and then depth.
        //
        // We want to break op cost ties with depth (rather than the other way
        // around). When the op cost is the same, we prefer shallow and wide
        // expressions to narrow and deep expressions and breaking ties with
        // `depth` gives us that. For example, `(a + b) + (c + d)` is preferred
        // to `((a + b) + c) + d`. This is beneficial because it exposes more
        // instruction-level parallelism and shortens live ranges.
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Cost {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Cost {
    const DEPTH_BITS: u8 = 8;
    const DEPTH_MASK: u32 = (1 << Self::DEPTH_BITS) - 1;
    const OP_COST_MASK: u32 = !Self::DEPTH_MASK;
    const MAX_OP_COST: u32 = Self::OP_COST_MASK >> Self::DEPTH_BITS;

    pub(crate) fn infinity() -> Cost {
        // 2^32 - 1 is, uh, pretty close to infinite... (we use `Cost`
        // only for heuristics and always saturate so this suffices!)
        Cost(u32::MAX)
    }

    pub(crate) fn zero() -> Cost {
        Cost(0)
    }

    /// Construct a new `Cost` from the given parts.
    ///
    /// If the opcode cost is greater than or equal to the maximum representable
    /// opcode cost, then the resulting `Cost` saturates to infinity.
    fn new(opcode_cost: u32, depth: u8) -> Cost {
        if opcode_cost >= Self::MAX_OP_COST {
            Self::infinity()
        } else {
            Cost(opcode_cost << Self::DEPTH_BITS | u32::from(depth))
        }
    }

    fn depth(&self) -> u8 {
        let depth = self.0 & Self::DEPTH_MASK;
        u8::try_from(depth).unwrap()
    }

    fn op_cost(&self) -> u32 {
        (self.0 & Self::OP_COST_MASK) >> Self::DEPTH_BITS
    }

    /// Return the cost of an opcode.
    fn of_opcode(op: Opcode) -> Cost {
        match op {
            // Constants.
            Opcode::Iconst | Opcode::F32const | Opcode::F64const => Cost::new(1, 0),

            // Extends/reduces.
            Opcode::Uextend
            | Opcode::Sextend
            | Opcode::Ireduce
            | Opcode::Iconcat
            | Opcode::Isplit => Cost::new(1, 0),

            // "Simple" arithmetic.
            Opcode::Iadd
            | Opcode::Isub
            | Opcode::Band
            | Opcode::Bor
            | Opcode::Bxor
            | Opcode::Bnot
            | Opcode::Ishl
            | Opcode::Ushr
            | Opcode::Sshr => Cost::new(3, 0),

            // Everything else.
            _ => {
                let mut c = Cost::new(4, 0);
                if op.can_trap() || op.other_side_effects() {
                    c = c + Cost::new(5, 0);
                }
                if op.can_load() {
                    c = c + Cost::new(10, 0);
                }
                if op.can_store() {
                    c = c + Cost::new(20, 0);
                }
                c
            }
        }
    }

    /// Compute the cost of the operation and its given operands.
    ///
    /// Caller is responsible for checking that the opcode came from an instruction
    /// that satisfies `inst_predicates::is_pure_for_egraph()`.
    pub(crate) fn of_pure_op(op: Opcode, operand_costs: impl IntoIterator<Item = Self>) -> Self {
        let c = Self::of_opcode(op) + operand_costs.into_iter().sum();
        Cost::new(c.op_cost(), c.depth().saturating_add(1))
    }

    /// Compute the cost of an operation in the side-effectful skeleton.
    pub(crate) fn of_skeleton_op(op: Opcode, arity: usize) -> Self {
        Cost::of_opcode(op) + Cost::new(u32::try_from(arity).unwrap(), (arity != 0) as _)
    }
}

impl std::iter::Sum<Cost> for Cost {
    fn sum<I: Iterator<Item = Cost>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl std::default::Default for Cost {
    fn default() -> Cost {
        Cost::zero()
    }
}

impl std::ops::Add<Cost> for Cost {
    type Output = Cost;

    fn add(self, other: Cost) -> Cost {
        let op_cost = self.op_cost().saturating_add(other.op_cost());
        let depth = std::cmp::max(self.depth(), other.depth());
        Cost::new(op_cost, depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_cost() {
        let a = Cost::new(5, 2);
        let b = Cost::new(37, 3);
        assert_eq!(a + b, Cost::new(42, 3));
        assert_eq!(b + a, Cost::new(42, 3));
    }

    #[test]
    fn add_infinity() {
        let a = Cost::new(5, 2);
        let b = Cost::infinity();
        assert_eq!(a + b, Cost::infinity());
        assert_eq!(b + a, Cost::infinity());
    }

    #[test]
    fn op_cost_saturates_to_infinity() {
        let a = Cost::new(Cost::MAX_OP_COST - 10, 2);
        let b = Cost::new(11, 2);
        assert_eq!(a + b, Cost::infinity());
        assert_eq!(b + a, Cost::infinity());
    }

    #[test]
    fn depth_saturates_to_max_depth() {
        let a = Cost::new(10, u8::MAX);
        let b = Cost::new(10, 1);
        assert_eq!(
            Cost::of_pure_op(Opcode::Iconst, [a, b]),
            Cost::new(21, u8::MAX)
        );
        assert_eq!(
            Cost::of_pure_op(Opcode::Iconst, [b, a]),
            Cost::new(21, u8::MAX)
        );
    }
}
