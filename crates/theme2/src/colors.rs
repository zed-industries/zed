use gpui2::{FontWeight, Hsla};
use indexmap::IndexMap;

use crate::generate_struct_with_overrides;

pub struct StaticColors {
    pub transparent: Hsla,
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerColor {
    pub cursor: Hsla,
    pub background: Hsla,
    pub selection: Hsla,
}

type PlayerColors = Vec<PlayerColor>;

#[derive(Debug, Clone, Copy)]
pub enum StatusColorName {
    Conflict,
    Created,
    Deleted,
    Error,
    Hidden,
    Ignored,
    Info,
    Modified,
    Renamed,
    Success,
    Warning,
}

type StatusColors = IndexMap<StatusColorName, Hsla>;

#[derive(Debug, Clone, Copy)]
pub enum GitStatusColorName {
    Conflict,
    Created,
    Deleted,
    Ignored,
    Modified,
    Renamed,
}

type GitStatusColors = IndexMap<GitStatusColorName, Hsla>;

#[derive(Debug, Clone, Copy)]
pub enum SyntaxColorName {
    Comment,
    CommentDoc,
    Primary,
    Predictive,
    Hint,
    Emphasis,
    EmphasisStrong,
    Title,
    LinkUri,
    LinkText,
    TextLiteral,
    Punctuation,
    PunctuationBracket,
    PunctuationDelimiter,
    PunctuationSpecial,
    PunctuationListMarker,
    String,
    StringSpecial,
    StringSpecialSymbol,
    StringEscape,
    StringRegex,
    Constructor,
    Variant,
    Type,
    TypeBuiltin,
    Variable,
    VariableSpecial,
    Label,
    Tag,
    Attribute,
    Property,
    Constant,
    Keyword,
    Enum,
    Operator,
    Number,
    Boolean,
    ConstantBuiltin,
    Function,
    FunctionBuiltin,
    FunctionDefinition,
    FunctionSpecialDefinition,
    FunctionMethod,
    FunctionMethodBuiltin,
    Preproc,
    Embedded,
}

#[derive(Debug, Clone, Copy)]
pub struct SyntaxStyle {
    pub color: Hsla,
    pub weight: FontWeight,
    pub underline: bool,
    pub italic: bool,
    // Nate: In the future I'd like to enable using background highlights for syntax highlighting
    // pub highlight: Hsla,
}

type SyntaxStyles = IndexMap<SyntaxColorName, SyntaxStyle>;

pub enum ThemeStyleName {
    Border,
    BorderVariant,
    BorderFocused,
    BorderTransparent,
    ElevatedSurface,
    Surface,
    Background,
    Element,
    ElementHover,
    ElementActive,
    ElementSelected,
    ElementDisabled,
    ElementPlaceholder,
    GhostElement,
    GhostElementHover,
    GhostElementActive,
    GhostElementSelected,
    GhostElementDisabled,
    Text,
    TextMuted,
    TextPlaceholder,
    TextDisabled,
    TextAccent,
    Icon,
    IconMuted,
    IconDisabled,
    IconPlaceholder,
    IconAccent,
    Syntax,
    StatusBar,
    TitleBar,
    Toolbar,
    TabBar,
    Editor,
    EditorSubheader,
    EditorActiveLine,
}

type ThemeColor = IndexMap<ThemeStyleName, Hsla>;

generate_struct_with_overrides! {
    ThemeStyle,
    ThemeStyleOverrides,
    color: ThemeColor,
    status: StatusColors,
    git: GitStatusColors,
    player: PlayerColors
}
