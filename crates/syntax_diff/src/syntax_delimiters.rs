#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SyntaxDelimiterCursor {
    // TODO: SmallVec<[(u64, u64)]; 16> to avoid limits
    lhs_depths: u128,
    rhs_depths: u128,
    both_depth: u8,
}

impl SyntaxDelimiterCursor {
    const LAYER_SIZE: u8 = 8;

    pub fn is_empty(&self) -> bool {
        self.both_depth == 0 && self.lhs_depths == 0 && self.rhs_depths == 0
    }

    pub fn push_both(self) -> Self {
        Self {
            lhs_depths: self.lhs_depths,
            rhs_depths: self.rhs_depths,
            both_depth: self.both_depth + 1,
        }
    }

    pub fn push_lhs(self) -> Self {
        Self {
            lhs_depths: self.lhs_depths + self.shift(),
            rhs_depths: self.rhs_depths,
            both_depth: self.both_depth,
        }
    }

    pub fn push_rhs(self) -> Self {
        Self {
            rhs_depths: self.rhs_depths + self.shift(),
            lhs_depths: self.lhs_depths,
            both_depth: self.both_depth,
        }
    }

    pub fn pop_lhs(self) -> Option<Self> {
        if self.lhs_depth() == 0 {
            return None;
        }

        Some(Self {
            lhs_depths: self.lhs_depths - self.shift(),
            rhs_depths: self.rhs_depths,
            both_depth: self.both_depth,
        })
    }

    pub fn pop_rhs(self) -> Option<Self> {
        if self.rhs_depth() == 0 {
            return None;
        }

        Some(Self {
            rhs_depths: self.rhs_depths - self.shift(),
            lhs_depths: self.lhs_depths,
            both_depth: self.both_depth,
        })
    }

    pub fn pop_both(self) -> Option<Self> {
        if self.both_depth == 0 || self.lhs_depth() != 0 || self.rhs_depth() != 0 {
            return None;
        }

        Some(Self {
            both_depth: self.both_depth - 1,
            lhs_depths: self.lhs_depths,
            rhs_depths: self.rhs_depths,
        })
    }

    pub fn can_pop_lhs(&self) -> bool {
        self.lhs_depth() > 0
    }

    pub fn can_pop_rhs(&self) -> bool {
        self.rhs_depth() > 0
    }

    fn shift(&self) -> u128 {
        1 << (self.both_depth * Self::LAYER_SIZE)
    }

    fn lhs_depth(&self) -> u8 {
        ((self.lhs_depths >> (self.both_depth * Self::LAYER_SIZE)) & 0xFF) as u8
    }

    fn rhs_depth(&self) -> u8 {
        ((self.rhs_depths >> (self.both_depth * Self::LAYER_SIZE)) & 0xFF) as u8
    }
}
