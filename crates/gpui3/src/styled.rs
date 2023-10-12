use crate::{Cascade, Hoverable, Pressable, Refineable, SharedString};

pub trait Styled {
    type Style: 'static + Refineable + Send + Sync + Default;

    fn style_cascade(&mut self) -> &mut Cascade<Self::Style>;
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
        Hoverable::new(self, None)
    }

    fn group_hover(self, group_name: impl Into<SharedString>) -> Hoverable<Self>
    where
        Self: 'static + Sized + Send + Sync,
        Self::Style: 'static + Refineable + Default + Send + Sync,
        <Self::Style as Refineable>::Refinement: 'static + Default + Send + Sync,
    {
        Hoverable::new(self, Some(group_name.into()))
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
