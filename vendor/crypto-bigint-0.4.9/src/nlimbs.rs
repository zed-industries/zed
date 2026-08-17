/// Calculate the number of limbs required to represent the given number of bits.
// TODO(tarcieri): replace with `generic_const_exprs` (rust-lang/rust#76560) when stable
#[macro_export]
macro_rules! nlimbs {
    ($bits:expr) => {
        $bits / $crate::Limb::BIT_SIZE
    };
}

#[cfg(test)]
mod tests {
    #[cfg(target_pointer_width = "32")]
    #[test]
    fn nlimbs_for_bits_macro() {
        assert_eq!(nlimbs!(64), 2);
        assert_eq!(nlimbs!(128), 4);
        assert_eq!(nlimbs!(192), 6);
        assert_eq!(nlimbs!(256), 8);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn nlimbs_for_bits_macro() {
        assert_eq!(nlimbs!(64), 1);
        assert_eq!(nlimbs!(128), 2);
        assert_eq!(nlimbs!(192), 3);
        assert_eq!(nlimbs!(256), 4);
    }
}
