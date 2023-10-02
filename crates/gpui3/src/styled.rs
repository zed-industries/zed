use crate::{Refineable, RefinementCascade};

pub trait Styled {
    type Style: Refineable + Default;

    fn style_cascade(&mut self) -> &mut RefinementCascade<Self::Style>;
    fn declared_style(&mut self) -> &mut <Self::Style as Refineable>::Refinement;

    fn computed_style(&mut self) -> Self::Style {
        Self::Style::from_refinement(&self.style_cascade().merged())
    }

    // fn hover(self) -> Hoverable<Self>
    // where
    //     Self: Sized,
    // {
    //     hoverable(self)
    // }

    // fn active(self) -> Pressable<Self>
    // where
    //     Self: Sized,
    // {
    //     pressable(self)
    // }
}
