use crate::ItemHandle;
use convex::ConvexClient;
use convex::Value;
use gpui::{
    AnyView, App, Context, Entity, EntityId, EventEmitter, ParentElement as _, Render, Styled,
    Window,
};
use gpui_tokio::Tokio;
use repo_name::RepoName;
use std::env;
use ui::prelude::*;
use ui::{h_flex, v_flex};

pub enum ToolbarItemEvent {
    ChangeLocation(ToolbarItemLocation),
}

pub trait ToolbarItemView: Render + EventEmitter<ToolbarItemEvent> {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn crate::ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation;

    fn pane_focus_update(
        &mut self,
        _pane_focused: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

trait ToolbarItemViewHandle: Send {
    fn id(&self) -> EntityId;
    fn to_any(&self) -> AnyView;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut App,
    ) -> ToolbarItemLocation;
    fn focus_changed(&mut self, pane_focused: bool, window: &mut Window, cx: &mut App);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ToolbarItemLocation {
    Hidden,
    PrimaryLeft,
    PrimaryRight,
    Secondary,
}

pub struct Toolbar {
    active_item: Option<Box<dyn ItemHandle>>,
    hidden: bool,
    can_navigate: bool,
    items: Vec<(Box<dyn ToolbarItemViewHandle>, ToolbarItemLocation)>,
}

impl Toolbar {
    fn has_any_visible_items(&self) -> bool {
        self.items
            .iter()
            .any(|(_item, location)| *location != ToolbarItemLocation::Hidden)
    }

    fn left_items(&self) -> impl Iterator<Item = &dyn ToolbarItemViewHandle> {
        self.items.iter().filter_map(|(item, location)| {
            if *location == ToolbarItemLocation::PrimaryLeft {
                Some(item.as_ref())
            } else {
                None
            }
        })
    }

    fn right_items(&self) -> impl Iterator<Item = &dyn ToolbarItemViewHandle> {
        self.items.iter().filter_map(|(item, location)| {
            if *location == ToolbarItemLocation::PrimaryRight {
                Some(item.as_ref())
            } else {
                None
            }
        })
    }

    fn secondary_items(&self) -> impl Iterator<Item = &dyn ToolbarItemViewHandle> {
        self.items.iter().rev().filter_map(|(item, location)| {
            if *location == ToolbarItemLocation::Secondary {
                Some(item.as_ref())
            } else {
                None
            }
        })
    }
}

impl Render for Toolbar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.has_any_visible_items() {
            return div();
        }

        let secondary_items = self.secondary_items().map(|item| item.to_any());

        let has_left_items = self.left_items().count() > 0;
        let has_right_items = self.right_items().count() > 0;

        v_flex()
            .group("toolbar")
            .relative()
            .p(DynamicSpacing::Base08.rems(cx))
            .when(has_left_items || has_right_items, |this| {
                this.gap(DynamicSpacing::Base08.rems(cx))
            })
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .bg(cx.theme().colors().toolbar_background)
            .when(has_left_items || has_right_items, |this| {
                this.child(
                    h_flex()
                        .min_h_6()
                        .justify_between()
                        .gap(DynamicSpacing::Base08.rems(cx))
                        .when(has_left_items, |this| {
                            this.child(
                                h_flex()
                                    .flex_auto()
                                    .justify_start()
                                    .overflow_x_hidden()
                                    .children(self.left_items().map(|item| item.to_any())),
                            )
                        })
                        .when(has_right_items, |this| {
                            this.child(
                                h_flex()
                                    .h_full()
                                    .flex_row_reverse()
                                    .map(|el| {
                                        if has_left_items {
                                            // We're using `flex_none` here to prevent some flickering that can occur when the
                                            // size of the left items container changes.
                                            el.flex_none()
                                        } else {
                                            el.flex_auto()
                                        }
                                    })
                                    .justify_end()
                                    .children(self.right_items().map(|item| item.to_any())),
                            )
                        }),
                )
            })
            .children(secondary_items)
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            active_item: None,
            items: Default::default(),
            hidden: false,
            can_navigate: true,
        }
    }

    pub fn set_can_navigate(&mut self, can_navigate: bool, cx: &mut Context<Self>) {
        self.can_navigate = can_navigate;
        cx.notify();
    }

    pub fn add_item<T>(&mut self, item: Entity<T>, window: &mut Window, cx: &mut Context<Self>)
    where
        T: 'static + ToolbarItemView,
    {
        let location = item.set_active_pane_item(self.active_item.as_deref(), window, cx);
        cx.subscribe(&item, |this, item, event, cx| {
            if let Some((_, current_location)) = this
                .items
                .iter_mut()
                .find(|(i, _)| i.id() == item.entity_id())
            {
                match event {
                    ToolbarItemEvent::ChangeLocation(new_location) => {
                        if new_location != current_location {
                            *current_location = *new_location;
                            cx.notify();
                        }
                    }
                }
            }
        })
        .detach();
        self.items.push((Box::new(item), location));
        cx.notify();
    }

    async fn update_current_file(
        file_path: String,
        function_name: String,
        class_name: String,
        repo_name: String,
    ) -> anyhow::Result<()> {
        let url = env::var("CONVEX_URL")?;
        let convex_user = env::var("CONVEX_USER")?;
        let mut client = ConvexClient::new(&url).await?;

        let result = client
            .mutation(
                "activity:update",
                maplit::btreemap! {
                    String::from("name") => Value::from(convex_user),
                    String::from("file_name") => Value::from(file_path),
                    String::from("function_name") => Value::from(function_name),
                    String::from("class_name") => Value::from(class_name),
                    String::from("repo_name") => Value::from(repo_name),
                },
            )
            .await?;
        println!("{result:#?}");

        Ok(())
    }

    pub fn set_active_item(
        &mut self,
        item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.active_item = item.map(|item| item.boxed_clone());
        self.hidden = self
            .active_item
            .as_ref()
            .map(|item| !item.show_toolbar(cx))
            .unwrap_or(false);

        let Some(active_item) = self.active_item.as_ref() else {
            return;
        };
        let Some(segments) = active_item.as_ref().breadcrumbs(cx.theme(), cx) else {
            return;
        };
        let absolute_file_name = String::from(active_item.suggested_filename(cx));
        let repo_name = cx.global::<RepoName>().0.clone();

        let mut class_name = String::new();
        let mut function_name = String::new();
        if segments.len() == 2 {
            function_name = String::from(segments[1].text.clone());
        }
        if segments.len() == 3 {
            function_name = String::from(segments[2].text.clone());
            class_name = String::from(segments[1].text.clone());
        }

        Tokio::spawn(cx, async move {
            let _ = Toolbar::update_current_file(
                absolute_file_name,
                function_name,
                class_name,
                repo_name,
            )
            .await;
        })
        .detach();

        for (toolbar_item, current_location) in self.items.iter_mut() {
            let new_location = toolbar_item.set_active_pane_item(item, window, cx);
            if new_location != *current_location {
                *current_location = new_location;
                cx.notify();
            }
        }
    }

    pub fn focus_changed(&mut self, focused: bool, window: &mut Window, cx: &mut Context<Self>) {
        for (toolbar_item, _) in self.items.iter_mut() {
            toolbar_item.focus_changed(focused, window, cx);
        }
    }

    pub fn item_of_type<T: ToolbarItemView>(&self) -> Option<Entity<T>> {
        self.items
            .iter()
            .find_map(|(item, _)| item.to_any().downcast().ok())
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }
}

impl<T: ToolbarItemView> ToolbarItemViewHandle for Entity<T> {
    fn id(&self) -> EntityId {
        self.entity_id()
    }

    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut App,
    ) -> ToolbarItemLocation {
        self.update(cx, |this, cx| {
            this.set_active_pane_item(active_pane_item, window, cx)
        })
    }

    fn focus_changed(&mut self, pane_focused: bool, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| {
            this.pane_focus_update(pane_focused, window, cx);
            cx.notify();
        });
    }
}
