use crate::{Hoverable, Pressable, Refineable, RefinementCascade};

pub trait Styled {
    type Style: 'static + Refineable + Send + Sync + Default;

    fn style_cascade(&mut self) -> &mut RefinementCascade<Self::Style>;
    fn declared_style(&mut self) -> &mut <Self::Style as Refineable>::Refinement;

    fn computed_style(&mut self) -> Self::Style {
        Self::Style::from_refinement(&self.style_cascade().merged())
    }

    fn hover(self) -> Hoverable<Self>
    where
        Self: 'static + Sized + Send + Sync,
        Self::Style: 'static + Refineable + Default + Send + Sync,
        <Self::Style as Refineable>::Refinement: 'static + Default + Send + Sync,
    {
        Hoverable::new(self)
    }

    fn active(self) -> Pressable<Self>
    where
        Self: 'static + Sized + Send + Sync,
        Self::Style: 'static + Refineable + Default + Send + Sync,
        <Self::Style as Refineable>::Refinement: 'static + Default + Send + Sync,
    {
        Pressable::new(self)
    }
}
