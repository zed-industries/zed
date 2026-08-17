pub trait SpecifiedValueInfo {
    const SUPPORTED_TYPES: u8 = 0;
}

impl<T: SpecifiedValueInfo> SpecifiedValueInfo for [T] {
    const SUPPORTED_TYPES: u8 = T::SUPPORTED_TYPES;
}
