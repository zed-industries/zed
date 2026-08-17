/// Support for `i31ref` disabled at compile time because the `gc` cargo feature
/// was not enabled.
pub enum I31 {}

impl I31 {
    pub fn get_u32(&self) -> u32 {
        match *self {}
    }

    pub fn get_i32(&self) -> i32 {
        match *self {}
    }
}
