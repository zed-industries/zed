use crate::{Operand, OperandConstraint, OperandKind};

pub struct Operands<'a>(pub &'a [Operand]);

impl<'a> Operands<'a> {
    pub fn new(operands: &'a [Operand]) -> Self {
        Self(operands)
    }

    pub fn matches<F: Fn(Operand) -> bool + 'a>(
        &self,
        predicate: F,
    ) -> impl Iterator<Item = (usize, Operand)> + 'a {
        self.0
            .iter()
            .cloned()
            .enumerate()
            .filter(move |(_, op)| predicate(*op))
    }

    pub fn def_ops(&self) -> impl Iterator<Item = (usize, Operand)> + 'a {
        self.matches(|op| op.kind() == OperandKind::Def)
    }

    pub fn use_ops(&self) -> impl Iterator<Item = (usize, Operand)> + 'a {
        self.matches(|op| op.kind() == OperandKind::Use)
    }

    pub fn reuse(&self) -> impl Iterator<Item = (usize, Operand)> + 'a {
        self.matches(|op| matches!(op.constraint(), OperandConstraint::Reuse(_)))
    }

    pub fn fixed(&self) -> impl Iterator<Item = (usize, Operand)> + 'a {
        self.matches(|op| matches!(op.constraint(), OperandConstraint::FixedReg(_)))
    }
}

impl<'a> core::ops::Index<usize> for Operands<'a> {
    type Output = Operand;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
