pub use derive_refineable::Refineable;

pub trait Refineable {
    type Refinement: Default;

    fn refine(&mut self, refinement: &Self::Refinement);
    fn refined(mut self, refinement: &Self::Refinement) -> Self
    where
        Self: Sized,
    {
        self.refine(refinement);
        self
    }
}
