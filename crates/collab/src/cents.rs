/// A number of cents.
#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
)]
pub struct Cents(pub u32);

impl Cents {
    pub const ZERO: Self = Self(0);

    pub const fn new(cents: u32) -> Self {
        Self(cents)
    }

    pub const fn from_dollars(dollars: u32) -> Self {
        Self(dollars * 100)
    }

    pub fn saturating_sub(self, other: Cents) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_cents_new() {
        assert_eq!(Cents::new(50), Cents(50));
    }

    #[test]
    fn test_cents_from_dollars() {
        assert_eq!(Cents::from_dollars(1), Cents(100));
        assert_eq!(Cents::from_dollars(5), Cents(500));
    }

    #[test]
    fn test_cents_zero() {
        assert_eq!(Cents::ZERO, Cents(0));
    }

    #[test]
    fn test_cents_add() {
        assert_eq!(Cents(50) + Cents(30), Cents(80));
    }

    #[test]
    fn test_cents_add_assign() {
        let mut cents = Cents(50);
        cents += Cents(30);
        assert_eq!(cents, Cents(80));
    }

    #[test]
    fn test_cents_saturating_sub() {
        assert_eq!(Cents(50).saturating_sub(Cents(30)), Cents(20));
        assert_eq!(Cents(30).saturating_sub(Cents(50)), Cents(0));
    }

    #[test]
    fn test_cents_ordering() {
        assert!(Cents(50) > Cents(30));
        assert!(Cents(30) < Cents(50));
        assert_eq!(Cents(50), Cents(50));
    }
}
