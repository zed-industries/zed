pub const fn default_true() -> bool {
    true
}

pub const fn default_false() -> bool {
    false
}

pub fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
