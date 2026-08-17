//! A stub for `std::sync::Barrier`.

#[derive(Debug)]
/// `std::sync::Barrier` is not supported yet in Loom. This stub is provided just
/// to make the code to compile.
pub struct Barrier {}

impl Barrier {
    /// `std::sync::Barrier` is not supported yet in Loom. This stub is provided just
    /// to make the code to compile.
    pub fn new(_n: usize) -> Self {
        unimplemented!("std::sync::Barrier is not supported yet in Loom.")
    }
    /// `std::sync::Barrier` is not supported yet in Loom. This stub is provided just
    /// to make the code to compile.
    pub fn wait(&self) -> std::sync::BarrierWaitResult {
        unimplemented!("std::sync::Barrier is not supported yet in Loom.")
    }
}
