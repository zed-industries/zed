use collections::HashMap;
use futures::{FutureExt as _, future::Shared};
use gpui::{App, Entity, Subscription, Task, WeakEntity};
use language::OutlineItem;
use multi_buffer::Anchor;
use project::{bookmark_store::BookmarkStore, debugger::breakpoint_store::BreakpointStore};
use text::BufferId;
use util::debug_panic;

use crate::Editor;
use crate::code_lens::CodeLensState;
use crate::document_colors::LspColorData;
use crate::document_links::LspDocumentLinks;
use crate::git::{DiffReviewState, GitBlameState};
use crate::inlays::{InlineValueCache, inlay_hints::LspInlayHintData};
use crate::runnables::RunnableData;
use crate::semantic_tokens::SemanticTokenState;

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub enum SizingBehavior {
    /// The editor will layout itself using `size_full` and will include the vertical
    /// scroll margin as requested by user settings.
    #[default]
    Default,
    /// The editor will layout itself using `size_full`, but will not have any
    /// vertical overscroll.
    ExcludeOverscrollMargin,
    /// The editor will request a vertical size according to its content and will be
    /// layouted without a vertical scroll margin.
    SizeByContent,
}

/// A cloneable description of an editor's mode, without any of the state that
/// the editor keeps per mode: used to construct editors, to switch their modes,
/// and to describe the mode in [`crate::EditorSnapshot`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EditorModeConfig {
    SingleLine,
    AutoHeight {
        min_lines: usize,
        max_lines: Option<usize>,
    },
    Full {
        /// When set to `true`, the editor will scale its UI elements with the buffer font size.
        scale_ui_elements_with_buffer_font_size: bool,
        /// When set to `true`, the editor will render a background for the active line.
        show_active_line_background: bool,
        /// Determines the sizing behavior for this editor
        sizing_behavior: SizingBehavior,
    },
    Minimap {
        parent: WeakEntity<Editor>,
    },
}

impl EditorModeConfig {
    pub fn full() -> Self {
        Self::Full {
            scale_ui_elements_with_buffer_font_size: true,
            show_active_line_background: true,
            sizing_behavior: SizingBehavior::Default,
        }
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        matches!(self, Self::Full { .. })
    }

    #[inline]
    pub fn is_single_line(&self) -> bool {
        matches!(self, Self::SingleLine)
    }

    #[inline]
    pub(crate) fn is_minimap(&self) -> bool {
        matches!(self, Self::Minimap { .. })
    }
}

/// The mode of an editor, owning all state that only exists in that mode.
pub enum EditorMode {
    SingleLine,
    AutoHeight {
        min_lines: usize,
        max_lines: Option<usize>,
    },
    Full(Box<FullEditorMode>),
    Minimap {
        parent: WeakEntity<Editor>,
    },
}

/// Configuration and state that only a [`EditorMode::Full`] editor has.
pub struct FullEditorMode {
    pub scale_ui_elements_with_buffer_font_size: bool,
    pub show_active_line_background: bool,
    pub sizing_behavior: SizingBehavior,
    pub(crate) minimap: Option<Entity<Editor>>,
    pub(crate) git_blame: GitBlameState,
    pub(crate) diff_review: DiffReviewState,
    pub(crate) runnables: RunnableData,
    pub(crate) runnables_for_selection_toggle: Task<()>,
    pub(crate) bookmark_store: Option<Entity<BookmarkStore>>,
    pub(crate) breakpoint_store: Option<Entity<BreakpointStore>>,
    pub(crate) inline_value_cache: InlineValueCache,
    pub(crate) subscriptions: Vec<Subscription>,
    pub(crate) lsp_data: Option<LspData>,
}

/// LSP-derived state that only a full mode editor fetches and renders.
pub(crate) struct LspData {
    pub(crate) next_color_inlay_id: usize,
    pub(crate) colors: Option<LspColorData>,
    pub(crate) code_lens: Option<CodeLensState>,
    pub(crate) refresh_colors_task: Task<()>,
    pub(crate) refresh_code_lens_task: Task<()>,
    pub(crate) use_document_folding_ranges: bool,
    pub(crate) refresh_folding_ranges_task: Task<()>,
    pub(crate) inlay_hints: Option<LspInlayHintData>,
    pub(crate) semantic_token_state: SemanticTokenState,
    pub(crate) refresh_document_symbols_task: Shared<Task<()>>,
    pub(crate) lsp_document_links: LspDocumentLinks,
    pub(crate) lsp_document_symbols: HashMap<BufferId, Vec<OutlineItem<text::Anchor>>>,
    pub(crate) refresh_outline_symbols_at_cursor_at_cursor_task: Task<()>,
    pub(crate) outline_symbols_at_cursor: Option<(BufferId, Vec<OutlineItem<Anchor>>)>,
    pub(crate) sticky_headers_task: Task<()>,
    pub(crate) sticky_headers: Option<Vec<OutlineItem<Anchor>>>,
}

impl LspData {
    pub(crate) fn new(
        code_lens: Option<CodeLensState>,
        inlay_hints: LspInlayHintData,
        cx: &App,
    ) -> Self {
        Self {
            next_color_inlay_id: 0,
            colors: Some(LspColorData::new(cx)),
            code_lens,
            refresh_colors_task: Task::ready(()),
            refresh_code_lens_task: Task::ready(()),
            use_document_folding_ranges: true,
            refresh_folding_ranges_task: Task::ready(()),
            inlay_hints: Some(inlay_hints),
            semantic_token_state: SemanticTokenState::new(cx, true),
            refresh_document_symbols_task: Task::ready(()).shared(),
            lsp_document_links: LspDocumentLinks::new(cx),
            lsp_document_symbols: HashMap::default(),
            refresh_outline_symbols_at_cursor_at_cursor_task: Task::ready(()),
            outline_symbols_at_cursor: None,
            sticky_headers_task: Task::ready(()),
            sticky_headers: None,
        }
    }
}

impl EditorMode {
    pub(crate) fn new(config: EditorModeConfig) -> Self {
        match config {
            EditorModeConfig::SingleLine => Self::SingleLine,
            EditorModeConfig::AutoHeight {
                min_lines,
                max_lines,
            } => Self::AutoHeight {
                min_lines,
                max_lines,
            },
            EditorModeConfig::Full {
                scale_ui_elements_with_buffer_font_size,
                show_active_line_background,
                sizing_behavior,
            } => Self::Full(Box::new(FullEditorMode {
                scale_ui_elements_with_buffer_font_size,
                show_active_line_background,
                sizing_behavior,
                minimap: None,
                git_blame: GitBlameState::default(),
                diff_review: DiffReviewState::default(),
                runnables: RunnableData::new(),
                runnables_for_selection_toggle: Task::ready(()),
                bookmark_store: None,
                breakpoint_store: None,
                inline_value_cache: InlineValueCache::new(false),
                subscriptions: Vec::new(),
                lsp_data: None,
            })),
            EditorModeConfig::Minimap { parent } => Self::Minimap { parent },
        }
    }

    pub fn config(&self) -> EditorModeConfig {
        match self {
            Self::SingleLine => EditorModeConfig::SingleLine,
            Self::AutoHeight {
                min_lines,
                max_lines,
            } => EditorModeConfig::AutoHeight {
                min_lines: *min_lines,
                max_lines: *max_lines,
            },
            Self::Full(full) => EditorModeConfig::Full {
                scale_ui_elements_with_buffer_font_size: full
                    .scale_ui_elements_with_buffer_font_size,
                show_active_line_background: full.show_active_line_background,
                sizing_behavior: full.sizing_behavior,
            },
            Self::Minimap { parent } => EditorModeConfig::Minimap {
                parent: parent.clone(),
            },
        }
    }

    #[inline]
    pub fn full(&self) -> Option<&FullEditorMode> {
        match self {
            Self::Full(full) => Some(full),
            _ => None,
        }
    }

    #[inline]
    pub fn full_mut(&mut self) -> Option<&mut FullEditorMode> {
        match self {
            Self::Full(full) => Some(full),
            _ => None,
        }
    }

    /// Same as [`Self::full`], but `debug_panic!`s when the editor is not in
    /// full mode: for callers that must not be reachable otherwise.
    #[track_caller]
    pub fn expect_full(&self) -> Option<&FullEditorMode> {
        let full = self.full();
        if full.is_none() {
            debug_panic!("expected a full mode editor");
        }
        full
    }

    /// Same as [`Self::full_mut`], but `debug_panic!`s when the editor is not
    /// in full mode: for callers that must not be reachable otherwise.
    #[track_caller]
    pub fn expect_full_mut(&mut self) -> Option<&mut FullEditorMode> {
        let full = match self {
            Self::Full(full) => Some(full.as_mut()),
            _ => None,
        };
        if full.is_none() {
            debug_panic!("expected a full mode editor");
        }
        full
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        matches!(self, Self::Full { .. })
    }

    #[inline]
    pub fn is_single_line(&self) -> bool {
        matches!(self, Self::SingleLine)
    }

    #[inline]
    pub(crate) fn is_minimap(&self) -> bool {
        matches!(self, Self::Minimap { .. })
    }
}
