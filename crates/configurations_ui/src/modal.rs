use std::sync::Arc;

use configuration::ConfigurationTemplate;
use fuzzy::StringMatch;
use gpui::{
    rems, App, AppContext as _, Context, DismissEvent, Entity, EventEmitter, Focusable, ParentElement, Render, Styled, Subscription, Task, WeakEntity, Window,
};
use picker::{Picker, PickerDelegate};
use project::{ConfigurationSourceKind, ConfigurationStore};
use ui::{prelude::*, ListItem, ListItemSpacing};
use workspace::{ModalView, Workspace};

pub struct ConfigurationsModalDelegate {
    configuration_store: Entity<ConfigurationStore>,
    pub candidates: Option<Vec<(ConfigurationSourceKind, ConfigurationTemplate)>>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    workspace: WeakEntity<Workspace>,
    prompt: String,
}

impl ConfigurationsModalDelegate {
    fn new(
        configuration_store: Entity<ConfigurationStore>,
        workspace: WeakEntity<Workspace>,
    ) -> Self {
        Self {
            configuration_store,
            workspace,
            candidates: None,
            matches: Vec::new(),
            selected_index: 0,
            prompt: String::default(),
        }
    }
}

pub struct ConfigurationsModal {
    pub picker: Entity<Picker<ConfigurationsModalDelegate>>,
    _subscription: Subscription,
}

impl ConfigurationsModal {
    pub fn new(
        configuration_store: Entity<ConfigurationStore>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut delegate = ConfigurationsModalDelegate::new(configuration_store.clone(), workspace);
        
        // Load configurations from the store
        if let Some(inventory) = configuration_store.read(cx).configuration_inventory() {
            delegate.candidates = Some(inventory.read(cx).list_configurations(None));
        }
        
        let picker = cx.new(|cx| {
            Picker::uniform_list(
                delegate,
                window,
                cx,
            )
            .modal(true)
        });
        let _subscription = cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        });
        Self {
            picker,
            _subscription,
        }
    }
}

impl EventEmitter<DismissEvent> for ConfigurationsModal {}

impl ModalView for ConfigurationsModal {}

impl Render for ConfigurationsModal {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl Focusable for ConfigurationsModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.read(cx).focus_handle(cx)
    }
}

impl PickerDelegate for ConfigurationsModalDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(
        &self,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Arc<str> {
        "Select a configuration...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.prompt = query.clone();
        
        let candidates = self.candidates.clone().unwrap_or_default();
        let query_lower = query.to_lowercase();
        
        let matches = if query.is_empty() {
            candidates
                .iter()
                .enumerate()
                .map(|(index, (_, config))| StringMatch {
                    candidate_id: index,
                    score: 0.0,
                    positions: Vec::new(),
                    string: config.label.clone(),
                })
                .collect()
        } else {
            candidates
                .iter()
                .enumerate()
                .filter_map(|(index, (_, config))| {
                    let label_lower = config.label.to_lowercase();
                    if label_lower.contains(&query_lower) {
                        Some(StringMatch {
                            candidate_id: index,
                            score: 1.0,
                            positions: Vec::new(),
                            string: config.label.clone(),
                        })
                    } else {
                        None
                    }
                })
                .collect()
        };
        
        self.matches = matches;
        cx.notify();
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            if let Some(candidates) = &self.candidates {
                if let Some((source, template)) = candidates.get(mat.candidate_id) {
                    log::info!("Configuration confirmed: '{}'", template.label);
                    
                    // Update global state
                    crate::set_selected_configuration(source.clone(), template.clone(), cx);
                }
            }
        }
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = self.matches.get(ix)?;
        let candidates = self.candidates.as_ref()?;
        let (_source, template) = candidates.get(mat.candidate_id)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .gap_2()
                        .child(Label::new(template.label.clone()))
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().colors().text_muted)
                                .child(format!("Recipe: {}", template.recipe)),
                        ),
                )
        )
    }
}
