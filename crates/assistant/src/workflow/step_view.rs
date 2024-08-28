use super::WorkflowStep;
use crate::{Assist, Context};
use editor::{
    display_map::{BlockDisposition, BlockProperties, BlockStyle},
    Editor, EditorEvent, ExcerptRange, MultiBuffer,
};
use gpui::{
    div, AnyElement, AppContext, Context as _, Empty, EventEmitter, FocusableView, IntoElement,
    Model, ParentElement as _, Render, SharedString, Styled as _, View, ViewContext,
    VisualContext as _, WeakModel, WindowContext,
};
use language::{language_settings::SoftWrap, Anchor, Buffer, LanguageRegistry};
use std::{ops::DerefMut, sync::Arc};
use text::OffsetRangeExt;
use theme::ActiveTheme as _;
use ui::{
    h_flex, v_flex, ButtonCommon as _, ButtonLike, ButtonStyle, Color, Icon, IconName,
    InteractiveElement as _, Label, LabelCommon as _,
};
use workspace::{
    item::{self, Item},
    pane,
    searchable::SearchableItemHandle,
};
