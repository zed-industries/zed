pub use derive_refineable::Refineable;

pub trait Refineable {
    type Refinement;

    fn refine(&self, refinement: &Self::Refinement) -> Self;
    fn from_refinement(refinement: &Self::Refinement) -> Self
    where
        Self: Sized + Default,
    {
        Self::default().refine(refinement)
    }
}
