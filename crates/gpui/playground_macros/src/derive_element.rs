// type Result = ();
// type LayoutContext<> = ();

// trait Element<V: 'static>: 'static + Clone {
//     type Layout: 'static;

//     fn style_mut(&mut self) -> &mut Style;
//     fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>)
//         -> Result<(NodeId, Self::Layout)>;
//     fn paint<'a>(
//         &mut self,
//         layout: Layout<'a, Self::Layout>,
//         view: &mut V,
//         cx: &mut PaintContext<V>,
//     ) -> Result<()>;
// }

// struct Button {
//     style: Style,
// }

// type Style = ();

// impl Button {
//     fn render<V>() -> impl Element<V> {
//         todo!()
//     }
// }

// impl<V: 'static> Element<V> for Foo {
//     type Layout = ();

//     fn style_mut(&mut self) -> &mut Style {
//         unimplemented!()
//     }

//     fn layout(
//         &mut self,
//         view: &mut V,
//         cx: &mut LayoutContext<V>,
//     ) -> Result<(NodeId, Self::Layout)> {
//         unimplemented!()
//     }

//     fn paint(&mut self, layout: Layout<()>, view: &mut V, cx: &mut PaintContext<V>) -> Result<()> {
//         unimplemented!()
//     }
// }
