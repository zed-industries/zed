pub use derive_refineable::Refineable;

pub trait Refineable: Clone {
    type Refinement: Refineable<Refinement = Self::Refinement> + Default;

    fn refine(&mut self, refinement: &Self::Refinement);
    fn refined(mut self, refinement: &Self::Refinement) -> Self
    where
        Self: Sized,
    {
        self.refine(refinement);
        self
    }
    fn from_refinement(refinement: &Self::Refinement) -> Self
    where
        Self: Default + Sized,
    {
        Self::default().refined(refinement)
    }
    fn from_cascade(cascade: &RefinementCascade<Self>) -> Self
    where
        Self: Default + Sized,
    {
        Self::default().refined(&cascade.merged())
    }
}

pub struct RefinementCascade<S: Refineable>(Vec<Option<S::Refinement>>);

impl<S: Refineable + Default> Default for RefinementCascade<S> {
    fn default() -> Self {
        Self(vec![Some(Default::default())])
    }
}

#[derive(Copy, Clone)]
pub struct CascadeSlot(usize);

impl<S: Refineable + Default> RefinementCascade<S> {
    pub fn reserve(&mut self) -> CascadeSlot {
        self.0.push(None);
        return CascadeSlot(self.0.len() - 1);
    }

    pub fn base(&mut self) -> &mut S::Refinement {
        self.0[0].as_mut().unwrap()
    }

    pub fn set(&mut self, slot: CascadeSlot, refinement: Option<S::Refinement>) {
        self.0[slot.0] = refinement
    }

    pub fn merged(&self) -> S::Refinement {
        let mut merged = self.0[0].clone().unwrap();
        for refinement in self.0.iter().skip(1) {
            if let Some(refinement) = refinement {
                merged.refine(refinement);
            }
        }
        merged
    }
}
