Much of element styling is now handled by an external engine.

How do I make an element hover.

There's a hover style.

Hoverable needs to wrap another element. That element can be styled.

```rs
struct Hoverable<E: Element> {

}

impl<V> Element<V> for Hoverable {

}
```

```rs
#[derive(Styled, Interactive)]
pub struct Div {
    declared_style: StyleRefinement,
    interactions: Interactions
}

pub trait Styled {
    fn declared_style(&mut self) -> &mut StyleRefinement;
    fn compute_style(&mut self) -> Style {
        Style::default().refine(self.declared_style())
    }

    // All the tailwind classes, modifying self.declared_style()
}

impl Style {
    pub fn paint_background<V>(layout: Layout, cx: &mut PaintContext<V>);
    pub fn paint_foreground<V>(layout: Layout, cx: &mut PaintContext<V>);
}

pub trait Interactive<V> {
    fn interactions(&mut self) -> &mut Interactions<V>;

    fn on_click(self, )
}

struct Interactions<V> {
    click: SmallVec<[<Rc<dyn Fn(&mut V, &dyn Any, )>; 1]>,
}
```

```rs
trait Stylable {
    type Style;

    fn with_style(self, style: Self::Style) -> Self;
}
```
