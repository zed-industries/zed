use gpui::{
    div, AnyElement, Div, IntoElement as _, ParentElement as _, Render, RenderOnce, Styled,
    ViewContext, WindowContext,
};
use ui::Avatar;

#[derive(Default)]
pub(crate) struct FacePile {
    faces: Vec<AnyElement>,
}

impl RenderOnce for FacePile {
    type Rendered = Div;

    fn render(self, _: &mut WindowContext) -> Self::Rendered {
        let player_count = self.faces.len();
        let player_list = self.faces.into_iter().enumerate().map(|(ix, player)| {
            let isnt_last = ix < player_count - 1;

            div().when(isnt_last, |div| div.neg_mr_1()).child(player)
        });
        div().p_1().flex().items_center().children(player_list)
    }
}

// impl Element for FacePile {
//     type State = ();
//     fn layout(
//         &mut self,
//         state: Option<Self::State>,
//         cx: &mut WindowContext,
//     ) -> (LayoutId, Self::State) {
//         let mut width = 0.;
//         let mut max_height = 0.;
//         let mut faces = Vec::with_capacity(self.faces.len());
//         for face in &mut self.faces {
//             let layout = face.layout(cx);
//             width += layout.x();
//             max_height = f32::max(max_height, layout.y());
//             faces.push(layout);
//         }
//         width -= self.overlap * self.faces.len().saturating_sub(1) as f32;
//         (cx.request_layout(&Style::default(), faces), ())
//         // (
//         //     Vector2F::new(width, max_height.clamp(1., constraint.max.y())),
//         //     (),
//         // ))
//     }

//     fn paint(
//         &mut self,
//         bounds: RectF,
//         visible_bounds: RectF,
//         _layout: &mut Self::LayoutState,
//         view: &mut V,
//         cx: &mut ViewContext<V>,
//     ) -> Self::PaintState {
//         let visible_bounds = bounds.intersection(visible_bounds).unwrap_or_default();

//         let origin_y = bounds.upper_right().y();
//         let mut origin_x = bounds.upper_right().x();

//         for face in self.faces.iter_mut().rev() {
//             let size = face.size();
//             origin_x -= size.x();
//             let origin_y = origin_y + (bounds.height() - size.y()) / 2.0;

//             cx.scene().push_layer(None);
//             face.paint(vec2f(origin_x, origin_y), visible_bounds, view, cx);
//             cx.scene().pop_layer();
//             origin_x += self.overlap;
//         }

//         ()
//     }
// }

impl Extend<AnyElement> for FacePile {
    fn extend<T: IntoIterator<Item = AnyElement>>(&mut self, children: T) {
        self.faces.extend(children);
    }
}
