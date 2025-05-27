use std::ops::Range;
use std::str::FromStr as _;

use anyhow::{Context as _, Result};
use gpui::http_client::http::{HeaderMap, HeaderValue};
use gpui::{App, Context, Entity, SharedString};
use language::Buffer;
use project::Project;
use zed_llm_client::{
    EDIT_PREDICTIONS_USAGE_AMOUNT_HEADER_NAME, EDIT_PREDICTIONS_USAGE_LIMIT_HEADER_NAME, UsageLimit,
};

// TODO: Find a better home for `Direction`.
//
// This should live in an ancestor crate of `editor` and `inline_completion`,
// but at time of writing there isn't an obvious spot.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Prev,
    Next,
}

#[derive(Clone)]
pub struct InlineCompletion {
    /// The ID of the completion, if it has one.
    pub id: Option<SharedString>,
    pub edits: Vec<(Range<language::Anchor>, String)>,
    pub edit_preview: Option<language::EditPreview>,
}

pub enum DataCollectionState {
    /// The provider doesn't support data collection.
    Unsupported,
    /// Data collection is enabled.
    Enabled { is_project_open_source: bool },
    /// Data collection is disabled or unanswered.
    Disabled { is_project_open_source: bool },
}

impl DataCollectionState {
    pub fn is_supported(&self) -> bool {
        !matches!(self, DataCollectionState::Unsupported { .. })
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, DataCollectionState::Enabled { .. })
    }

    pub fn is_project_open_source(&self) -> bool {
        match self {
            Self::Enabled {
                is_project_open_source,
            }
            | Self::Disabled {
                is_project_open_source,
            } => *is_project_open_source,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EditPredictionUsage {
    pub limit: UsageLimit,
    pub amount: i32,
}

impl EditPredictionUsage {
    pub fn from_headers(headers: &HeaderMap<HeaderValue>) -> Result<Self> {
        let limit = headers
            .get(EDIT_PREDICTIONS_USAGE_LIMIT_HEADER_NAME)
            .with_context(|| {
                format!("missing {EDIT_PREDICTIONS_USAGE_LIMIT_HEADER_NAME:?} header")
            })?;
        let limit = UsageLimit::from_str(limit.to_str()?)?;

        let amount = headers
            .get(EDIT_PREDICTIONS_USAGE_AMOUNT_HEADER_NAME)
            .with_context(|| {
                format!("missing {EDIT_PREDICTIONS_USAGE_AMOUNT_HEADER_NAME:?} header")
            })?;
        let amount = amount.to_str()?.parse::<i32>()?;

        Ok(Self { limit, amount })
    }

    pub fn over_limit(&self) -> bool {
        match self.limit {
            UsageLimit::Limited(limit) => self.amount >= limit,
            UsageLimit::Unlimited => false,
        }
    }
}

pub trait EditPredictionProvider: 'static + Sized {
    fn name() -> &'static str;
    fn display_name() -> &'static str;
    fn show_completions_in_menu() -> bool;
    fn show_tab_accept_marker() -> bool {
        false
    }
    fn data_collection_state(&self, _cx: &App) -> DataCollectionState {
        DataCollectionState::Unsupported
    }

    fn usage(&self, _cx: &App) -> Option<EditPredictionUsage> {
        None
    }

    fn toggle_data_collection(&mut self, _cx: &mut App) {}
    fn is_enabled(
        &self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &App,
    ) -> bool;
    fn is_refreshing(&self) -> bool;
    fn refresh(
        &mut self,
        project: Option<Entity<Project>>,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    );
    fn needs_terms_acceptance(&self, _cx: &App) -> bool {
        false
    }
    fn cycle(
        &mut self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut Context<Self>,
    );
    fn accept(&mut self, cx: &mut Context<Self>);
    fn discard(&mut self, cx: &mut Context<Self>);
    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<InlineCompletion>;
}

pub trait InlineCompletionProviderHandle {
    fn name(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    fn is_enabled(
        &self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &App,
    ) -> bool;
    fn show_completions_in_menu(&self) -> bool;
    fn show_tab_accept_marker(&self) -> bool;
    fn data_collection_state(&self, cx: &App) -> DataCollectionState;
    fn usage(&self, cx: &App) -> Option<EditPredictionUsage>;
    fn toggle_data_collection(&self, cx: &mut App);
    fn needs_terms_acceptance(&self, cx: &App) -> bool;
    fn is_refreshing(&self, cx: &App) -> bool;
    fn refresh(
        &self,
        project: Option<Entity<Project>>,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut App,
    );
    fn cycle(
        &self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut App,
    );
    fn accept(&self, cx: &mut App);
    fn discard(&self, cx: &mut App);
    fn suggest(
        &self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut App,
    ) -> Option<InlineCompletion>;
}

impl<T> InlineCompletionProviderHandle for Entity<T>
where
    T: EditPredictionProvider,
{
    fn name(&self) -> &'static str {
        T::name()
    }

    fn display_name(&self) -> &'static str {
        T::display_name()
    }

    fn show_completions_in_menu(&self) -> bool {
        T::show_completions_in_menu()
    }

    fn show_tab_accept_marker(&self) -> bool {
        T::show_tab_accept_marker()
    }

    fn data_collection_state(&self, cx: &App) -> DataCollectionState {
        self.read(cx).data_collection_state(cx)
    }

    fn usage(&self, cx: &App) -> Option<EditPredictionUsage> {
        self.read(cx).usage(cx)
    }

    fn toggle_data_collection(&self, cx: &mut App) {
        self.update(cx, |this, cx| this.toggle_data_collection(cx))
    }

    fn is_enabled(
        &self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &App,
    ) -> bool {
        self.read(cx).is_enabled(buffer, cursor_position, cx)
    }

    fn needs_terms_acceptance(&self, cx: &App) -> bool {
        self.read(cx).needs_terms_acceptance(cx)
    }

    fn is_refreshing(&self, cx: &App) -> bool {
        self.read(cx).is_refreshing()
    }

    fn refresh(
        &self,
        project: Option<Entity<Project>>,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut App,
    ) {
        self.update(cx, |this, cx| {
            this.refresh(project, buffer, cursor_position, debounce, cx)
        })
    }

    fn cycle(
        &self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut App,
    ) {
        self.update(cx, |this, cx| {
            this.cycle(buffer, cursor_position, direction, cx)
        })
    }

    fn accept(&self, cx: &mut App) {
        self.update(cx, |this, cx| this.accept(cx))
    }

    fn discard(&self, cx: &mut App) {
        self.update(cx, |this, cx| this.discard(cx))
    }

    fn suggest(
        &self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut App,
    ) -> Option<InlineCompletion> {
        self.update(cx, |this, cx| this.suggest(buffer, cursor_position, cx))
    }
}
