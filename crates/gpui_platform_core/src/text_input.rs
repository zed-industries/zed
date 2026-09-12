//! Text input session configuration shared by `gpui` and its platform backends.

/// The configuration of a focused text region's input session.
///
/// These are hints to the platform's IME / software keyboard rather than
/// input-session attributes (on web, DOM attributes of the hidden editable
/// element such as `autocorrect` and `enterkeyhint`).
///
/// The default disables all text assistance and requests no particular action
/// key presentation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextInputConfiguration {
    /// Whether the platform may automatically correct entered text.
    pub autocorrect: bool,
    /// How software keyboards automatically capitalize entered text.
    pub autocapitalize: Autocapitalize,
    /// Whether software keyboards may offer word suggestions and spellcheck.
    pub suggestions: bool,
    /// The action advertised on a software keyboard's confirm ("enter") key.
    pub input_action: TextInputAction,
}

/// Automatic capitalization applied by software keyboards.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Autocapitalize {
    /// No automatic capitalization.
    #[default]
    None,
    /// Capitalize the first letter of each word.
    Words,
    /// Capitalize the first letter of each sentence.
    Sentences,
    /// Capitalize every letter.
    Characters,
}

/// The action a software keyboard advertises on its confirm ("enter") key.
///
/// This affects only how the key is presented (icon or label); pressing it is
/// still delivered as ordinary input.
///
/// The variants are the HTML `enterkeyhint` attribute's value set
/// (<https://html.spec.whatwg.org/multipage/interaction.html#input-modalities:-the-enterkeyhint-attribute>),
/// which also maps onto Android's `IME_ACTION_*` constants and iOS's
/// `UIReturnKeyType`; [`TextInputAction::Unspecified`] means "emit no hint".
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextInputAction {
    /// Let the platform choose its default presentation.
    #[default]
    Unspecified,
    /// Inserting a line break.
    Enter,
    /// Committing the field's value.
    Done,
    /// Navigating to the typed target.
    Go,
    /// Moving to the next field.
    Next,
    /// Moving to the previous field.
    Previous,
    /// Executing a search.
    Search,
    /// Sending a message.
    Send,
}
