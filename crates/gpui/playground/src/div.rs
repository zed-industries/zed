use crate::style::StyleRefinement;
use playground_macros::styleable_trait;
use refineable::Refineable;

trait Element<V> {
    type Style;

    fn hover(self) -> Hover<V, Self>
    where
        Self: Sized,
        Self::Style: Refineable,
        <Self::Style as Refineable>::Refinement: Default,
    {
        Hover {
            child: self,
            style: <<Self as Element<V>>::Style as Refineable>::Refinement::default(),
        }
    }
}

use crate as playground;
styleable_trait!();

struct Hover<V, E: Element<V>>
where
    E::Style: Refineable,
{
    child: E,
    style: <E::Style as Refineable>::Refinement,
}

struct Div {
    style: StyleRefinement,
}

impl Styleable for Div {
    fn declared_style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn div() -> Div {
    Div {
        style: Default::default(),
    }
}

impl<V> Element<V> for Div {
    type Style = StyleRefinement;
}

#[test]
fn test() {
    let elt = div().w_auto();
}

// trait Element<V: 'static> {
//     type Style;

//     fn layout()
// }

// trait Stylable<V: 'static>: Element<V> {
//     type Style;

//     fn with_style(self, style: Self::Style) -> Self;
// }

// pub struct HoverStyle<S> {
//     default: S,
//     hovered: S,
// }

// struct Hover<V: 'static, C: Stylable<V>> {
//     child: C,
//     style: HoverStyle<C::Style>,
// }

// impl<V: 'static, C: Stylable<V>> Hover<V, C> {
//     fn new(child: C, style: HoverStyle<C::Style>) -> Self {
//         Self { child, style }
//     }
// }
