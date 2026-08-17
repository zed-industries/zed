/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Kleen logic: https://en.wikipedia.org/wiki/Three-valued_logic#Kleene_and_Priest_logics

/// A "trilean" value based on Kleen logic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KleeneValue {
    /// False
    False = 0,
    /// True
    True = 1,
    /// Either true or false, but we’re not sure which yet.
    Unknown,
}

impl From<bool> for KleeneValue {
    fn from(b: bool) -> Self {
        if b {
            Self::True
        } else {
            Self::False
        }
    }
}

impl KleeneValue {
    /// Turns this Kleene value to a bool, taking the unknown value as an
    /// argument.
    pub fn to_bool(self, unknown: bool) -> bool {
        match self {
            Self::True => true,
            Self::False => false,
            Self::Unknown => unknown,
        }
    }

    /// Return true if any result of f() is definitely true.
    /// Otherwise, return the `or` of all values.
    /// Returns false if empty, like that of `Iterator`.
    #[inline(always)]
    pub fn any<T>(iter: impl Iterator<Item = T>, f: impl FnMut(T) -> Self) -> Self {
        Self::any_value(iter, Self::True, Self::False, |a, b| a | b, f)
    }

    /// Return false if any results of f() is definitely false.
    /// Otherwise, return the `and` of all values.
    /// Returns true if empty, opposite of `Iterator`.
    #[inline(always)]
    pub fn any_false<T>(iter: impl Iterator<Item = T>, f: impl FnMut(T) -> Self) -> Self {
        Self::any_value(iter, Self::False, Self::True, |a, b| a & b, f)
    }

    #[inline(always)]
    fn any_value<T>(
        iter: impl Iterator<Item = T>,
        value: Self,
        on_empty: Self,
        op: impl Fn(Self, Self) -> Self,
        mut f: impl FnMut(T) -> Self,
    ) -> Self {
        let mut result = None;
        for item in iter {
            let r = f(item);
            if r == value {
                return r;
            }
            if let Some(v) = result.as_mut() {
                *v = op(*v, r);
            } else {
                result = Some(r);
            }
        }
        result.unwrap_or(on_empty)
    }
}

impl std::ops::Not for KleeneValue {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Unknown => Self::Unknown,
        }
    }
}

// Implements the logical and operation.
impl std::ops::BitAnd for KleeneValue {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        if self == Self::False || other == Self::False {
            return Self::False;
        }
        if self == Self::Unknown || other == Self::Unknown {
            return Self::Unknown;
        }
        Self::True
    }
}

// Implements the logical or operation.
impl std::ops::BitOr for KleeneValue {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        if self == Self::True || other == Self::True {
            return Self::True;
        }
        if self == Self::Unknown || other == Self::Unknown {
            return Self::Unknown;
        }
        Self::False
    }
}

impl std::ops::BitOrAssign for KleeneValue {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}

impl std::ops::BitAndAssign for KleeneValue {
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
