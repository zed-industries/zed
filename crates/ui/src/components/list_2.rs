use crate::prelude::*;
use gpui::{uniform_list, Entity, Hsla};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum List2ItemHeight {
    Default = 27,
}

impl List2ItemHeight {
    pub fn f32(&self) -> f32 {
        match self {
            List2ItemHeight::Default => 27.0,
        }
    }
}

pub enum List2Item {
    InsetItem(List2InsetItem),
    SectionTitle(List2SectionTitle),
}

#[derive(Clone)]
pub struct List2SectionTitle {
    label: SharedString,
}

impl List2SectionTitle {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

#[derive(Clone)]
pub struct List2InsetItem {
    id: ElementId,
    icon: Option<IconName>,
    label: SharedString,
    selected: bool,
}

impl List2InsetItem {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: None,
            label: label.into(),
            selected: false,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

pub struct List2Builder {
    id: SharedString,
    items: Vec<List2Item>,
    internal_last_item_ix: usize,
}

impl List2Builder {
    pub fn new(
        id: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut App,
        f: impl FnOnce(Self, &mut Window, &mut Context<Self>) -> Self,
    ) -> Entity<Self> {
        cx.new(|cx| {
            window.refresh();
            f(
                Self {
                    id: id.into(),
                    items: Vec::new(),
                    internal_last_item_ix: 0,
                },
                window,
                cx,
            )
        })
    }

    /// Request a unique [`gpui::ElementId`] for a new item.
    pub fn next_id(&mut self) -> ElementId {
        self.internal_last_item_ix += 1;
        let id = format!("list2_{}_{}", self.id, self.internal_last_item_ix).into();
        ElementId::Name(id)
    }

    pub fn section_title(mut self, title: impl Into<SharedString>) -> Self {
        self.items
            .push(List2Item::SectionTitle(List2SectionTitle::new(title)));
        self
    }

    pub fn inset_item(mut self, item: List2InsetItem) -> Self {
        self.items.push(List2Item::InsetItem(item));
        self
    }

    pub fn render_item(&self, ix: usize, cx: &Context<Self>) -> AnyElement {
        let item = &self.items[ix];
        match item {
            List2Item::InsetItem(item) => {
                self.render_inset_item(item.clone(), cx).into_any_element()
            }
            List2Item::SectionTitle(item) => self
                .render_section_title(item.clone(), cx)
                .into_any_element(),
        }
    }

    pub fn render_section_title(
        &self,
        el: List2SectionTitle,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let height = px(List2ItemHeight::Default.f32());

        h_flex()
            .h(height)
            .w_full()
            .px_3()
            .pt(px(7.))
            .pb(px(5.))
            .overflow_hidden()
            .flex_none()
            .child(div().text_ui(cx).child(el.label))
    }

    pub fn render_inset_item(&self, el: List2InsetItem, cx: &Context<Self>) -> impl IntoElement {
        let height = px(List2ItemHeight::Default.f32());
        let color = List2::state_color(cx);

        h_flex()
            .id(el.id)
            .h(height)
            .w_full()
            .items_center()
            .px_1p5()
            .overflow_hidden()
            .flex_none()
            .when(el.selected, |el| el.bg(color.selected_bg))
            .child(
                h_flex()
                    .gap_2()
                    .p_1p5()
                    .rounded(px(4.))
                    .when_some(el.icon, |el, icon| el.child(Icon::new(icon)))
                    .child(div().text_ui(cx).child(el.label)),
            )
    }

    // pub fn icon(mut self, icon: ToastIcon) -> Self {
    //     self.icon = Some(icon);
    //     self
    // }

    // pub fn action(
    //     mut self,
    //     label: impl Into<SharedString>,
    //     f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    // ) -> Self {
    //     self.action = Some(ToastAction::new(label.into(), Some(Arc::new(f))));
    //     self
    // }
}

pub struct List2StateColor {
    selected_bg: Hsla,
    marked_bg: Hsla,
    active_bg: Hsla,
    hover_bg: Hsla,
}

#[derive(IntoComponent)]
#[component(scope = "Layout", description = "A list component")]
pub struct List2 {}

impl List2 {
    fn state_color(cx: &App) -> List2StateColor {
        let selected_bg_alpha = 0.08;
        let marked_bg_alpha = 0.12;
        let state_opacity_step = 0.04;

        let bg_color = |selected: bool, marked: bool| -> Hsla {
            match (selected, marked) {
                (true, true) => cx
                    .theme()
                    .status()
                    .info
                    .alpha(selected_bg_alpha + marked_bg_alpha),
                (true, false) => cx.theme().status().info.alpha(selected_bg_alpha),
                (false, true) => cx.theme().status().info.alpha(marked_bg_alpha),
                _ => cx.theme().colors().ghost_element_background,
            }
        };

        let hover_bg = if true {
            cx.theme()
                .status()
                .info
                .alpha(selected_bg_alpha + state_opacity_step)
        } else {
            cx.theme().colors().ghost_element_hover
        };

        let active_bg = if true {
            cx.theme()
                .status()
                .info
                .alpha(selected_bg_alpha + state_opacity_step * 2.0)
        } else {
            cx.theme().colors().ghost_element_active
        };

        List2StateColor {
            selected_bg: bg_color(true, false),
            marked_bg: bg_color(false, true),
            active_bg,
            hover_bg,
        }
    }
}

impl List2 {}

impl Render for List2Builder {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let list_id = ElementId::Name(format!("list_{}", self.id).into());
        let len = self.items.len();

        uniform_list(
            cx.entity().clone(),
            list_id,
            len,
            move |this, range, _window, cx| range.map(|ix| this.render_item(ix, cx)).collect(),
        )
        .border_1()
        .border_color(cx.theme().colors().border)
        .bg(cx.theme().colors().panel_background)
        .size_full()
        .flex_none()
    }
}

impl ComponentPreview for List2 {
    fn preview(window: &mut Window, cx: &mut App) -> AnyElement {
        let basic_list = List2Builder::new("basic-list", window, cx, |this, _, _| {
            this.section_title("Favorites")
                .inset_item(
                    List2InsetItem::new("1", "Recents")
                        .icon(IconName::HistoryRerun.into())
                        .selected(true),
                )
                .inset_item(List2InsetItem::new("2", "Desktop").icon(IconName::Folder.into()))
                .inset_item(List2InsetItem::new("3", "Folders").icon(IconName::Folder.into()))
        });

        v_flex()
            .gap_6()
            .p_4()
            .children(vec![example_group_with_title(
                "Basic List",
                vec![single_example(
                    "Basic List",
                    div()
                        .w_80()
                        .h(px(640.))
                        .child(basic_list)
                        .into_any_element(),
                )],
            )])
            .into_any_element()
    }
}
