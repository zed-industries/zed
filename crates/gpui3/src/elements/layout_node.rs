use crate::{
    AnyElement, AppContext, BorrowWindow, Bounds, Element, ElementId, IdentifiedElement,
    IntoAnyElement, LayoutId, ParentElement, Pixels, SharedString, Style, StyleCascade, Styled,
    ViewContext,
};
use collections::HashMap;
use refineable::Refineable;
use smallvec::SmallVec;

#[derive(Default)]
struct GroupBounds(HashMap<SharedString, SmallVec<[Bounds<Pixels>; 1]>>);

pub fn group_bounds(name: &SharedString, cx: &mut AppContext) -> Option<Bounds<Pixels>> {
    cx.default_global::<GroupBounds>()
        .0
        .get(name)
        .and_then(|bounds_stack| bounds_stack.last().cloned())
}

pub trait ElementKind: 'static + Send + Sync {
    fn id(&self) -> Option<ElementId>;
}

pub struct IdentifiedElementKind(ElementId);
pub struct AnonymousElementKind;

impl ElementKind for IdentifiedElementKind {
    fn id(&self) -> Option<ElementId> {
        Some(self.0.clone())
    }
}

impl ElementKind for AnonymousElementKind {
    fn id(&self) -> Option<ElementId> {
        None
    }
}

pub struct LayoutNodeElement<V: 'static + Send + Sync, K: ElementKind> {
    style_cascade: StyleCascade,
    computed_style: Option<Style>,
    children: SmallVec<[AnyElement<V>; 2]>,
    kind: K,
    group: Option<SharedString>,
}

impl<V: 'static + Send + Sync> LayoutNodeElement<V, AnonymousElementKind> {
    pub fn new() -> LayoutNodeElement<V, AnonymousElementKind> {
        LayoutNodeElement {
            style_cascade: StyleCascade::default(),
            computed_style: None,
            children: SmallVec::new(),
            kind: AnonymousElementKind,
            group: None,
        }
    }

    pub fn identify(self, id: impl Into<ElementId>) -> LayoutNodeElement<V, IdentifiedElementKind> {
        LayoutNodeElement {
            style_cascade: self.style_cascade,
            computed_style: self.computed_style,
            children: self.children,
            kind: IdentifiedElementKind(id.into()),
            group: self.group,
        }
    }
}

impl<V: 'static + Send + Sync, E: ElementKind> LayoutNodeElement<V, E> {
    pub fn set_group(&mut self, group: impl Into<SharedString>) {
        self.group = Some(group.into());
    }

    fn with_element_id<R>(
        &mut self,
        cx: &mut ViewContext<V>,
        f: impl FnOnce(&mut Self, &mut ViewContext<V>) -> R,
    ) -> R {
        if let Some(id) = self.id() {
            cx.with_element_id(id, |cx| f(self, cx))
        } else {
            f(self, cx)
        }
    }
}

impl<V: 'static + Send + Sync, K: ElementKind> Styled for LayoutNodeElement<V, K> {
    fn style_cascade(&mut self) -> &mut StyleCascade {
        &mut self.style_cascade
    }

    fn computed_style(&mut self) -> &Style {
        self.computed_style
            .get_or_insert_with(|| Style::default().refined(self.style_cascade.merged()))
    }
}

impl<V: 'static + Send + Sync> IdentifiedElement for LayoutNodeElement<V, IdentifiedElementKind> {
    fn id(&self) -> ElementId {
        self.kind.0.clone()
    }
}

impl<V, K> IntoAnyElement<V> for LayoutNodeElement<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V: 'static + Send + Sync, K: ElementKind> Element for LayoutNodeElement<V, K> {
    type ViewState = V;
    type ElementState = ();

    fn id(&self) -> Option<ElementId> {
        self.kind.id()
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState) {
        self.with_element_id(cx, |this, cx| {
            let layout_ids = this
                .children
                .iter_mut()
                .map(|child| child.layout(state, cx))
                .collect::<Vec<_>>();

            let style = this.computed_style();
            let layout_id = cx.request_layout(style, layout_ids);
            (layout_id, ())
        })
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::ViewState,
        _: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        self.with_element_id(cx, |this, cx| {
            if let Some(group) = this.group.clone() {
                cx.default_global::<GroupBounds>()
                    .0
                    .entry(group)
                    .or_default()
                    .push(bounds);
            }

            let style = this.computed_style().clone();
            let z_index = style.z_index.unwrap_or(0);
            cx.stack(z_index, |cx| style.paint(bounds, cx));

            // todo!("implement overflow")
            // let overflow = &style.overflow;

            style.apply_text_style(cx, |cx| {
                cx.stack(z_index + 1, |cx| {
                    style.apply_overflow(bounds, cx, |cx| {
                        for child in &mut this.children {
                            child.paint(state, None, cx);
                        }
                    })
                })
            });

            if let Some(group) = this.group.as_ref() {
                cx.default_global::<GroupBounds>()
                    .0
                    .get_mut(group)
                    .unwrap()
                    .pop();
            }
        })
    }
}

impl<V: 'static + Send + Sync, K: ElementKind> ParentElement for LayoutNodeElement<V, K> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        &mut self.children
    }

    fn group_mut(&mut self) -> &mut Option<SharedString> {
        &mut self.group
    }
}
