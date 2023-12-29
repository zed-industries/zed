use gpui::Rgba;

/// `Theme` struct represents a collection of surfaces with their color states.
/// It defines the visual appearance of various UI elements in the IDE.
///
/// Each `Surface` in the `Theme` struct has a default, hovered, pressed, active, disabled, and inverted state.
/// These states determine the colors of the UI elements in different conditions.
#[derive(Default, Clone)]
pub struct FabricTheme {
    pub name: String,

    /// `cotton` is the base surface layer used for primary content areas and the main UI canvas.
    /// It provides a clean and neutral backdrop for content.
    pub cotton: FabricSurface,
    /// `linen` represents elevated UI components like active panels or dialogs.
    /// It suggests a layer above the base content.
    pub linen: FabricSurface,
    /// `denim` is used for the title bar and status elements; it provides a strong visual structure.
    pub denim: FabricSurface,
    /// `silk` is used for the most prominent, interactive UI elements like buttons and menus.
    /// It maintains consistency among top-level surfaces.
    pub silk: FabricSurface,
    /// `satin` is the accent surface for interactive elements like buttons, links, or highlighted text.
    /// It encourages interaction and focus.
    pub satin: FabricSurface,
    /// `positive` indicates positive or successful statuses, such as alerts and confirmation messages.
    /// It signals approval or completion.
    pub positive: FabricSurface,
    /// `warning` indicates potential issues or important notices that the user should not overlook.
    /// It alerts without causing alarm.
    pub warning: FabricSurface,
    /// `negative` indicates error states or negative actions, like incorrect inputs or destructive operations.
    /// It clearly signals an error but in a non-threatening way.
    pub negative: FabricSurface,
}

struct DebugInto<T>(T);

impl<T: std::fmt::Debug> std::fmt::Debug for DebugInto<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}.into()", self.0)
    }
}

impl std::fmt::Debug for FabricTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FabricTheme")
            .field("name", &DebugInto(&self.name))
            .field("cotton", &self.cotton)
            .field("linen", &self.linen)
            .field("denim", &self.denim)
            .field("silk", &self.silk)
            .field("satin", &self.satin)
            .field("positive", &self.positive)
            .field("warning", &self.warning)
            .field("negative", &self.negative)
            .finish()
    }
}

#[derive(Default, Debug, Clone)]
pub struct FabricSurface {
    pub default: FabricSurfaceState,
    pub hovered: FabricSurfaceState,
    pub pressed: FabricSurfaceState,
    pub active: FabricSurfaceState,
    pub disabled: FabricSurfaceState,
    pub inverted: FabricSurfaceState,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct FabricSurfaceState {
    pub background: Rgba,
    pub border: Rgba,
    pub foreground: Rgba,
    pub secondary_foreground: Option<Rgba>,
}
