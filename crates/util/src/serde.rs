pub const fn default_true() -> bool {
    true
}

pub fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
