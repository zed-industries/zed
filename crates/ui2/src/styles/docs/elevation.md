TODO: Originally sourced from Material Design 3, Rewrite to be more Zed specific

# Elevation

Zed applies elevation to all surfaces and components, which are categorized into levels.

Elevation accomplishes the following:
- Allows surfaces to move in front of or behind others, such as content scrolling beneath app top bars.
- Reflects spatial relationships, for instance, how a floating action button’s shadow intimates its disconnection from a collection of cards.
- Directs attention to structures at the highest elevation, like a temporary dialog arising in front of other surfaces.

Elevations are the initial elevation values assigned to components by default.

Components may transition to a higher elevation in some cases, like user interations.

On such occasions, components transition to predetermined dynamic elevation offsets. These are the typical elevations to which components move when they are not at rest.

## Understanding Elevation

Elevation can be thought of as the physical closeness of an element to the user. Elements with lower elevations are physically further away from the user on the z-axis and appear to be underneath elements with higher elevations.

Material Design 3 has a some great visualizations of elevation that may be helpful to understanding the mental modal of elevation. [Material Design – Elevation](https://m3.material.io/styles/elevation/overview)

## Elevation

1. App Background (e.x.: Workspace, system window)
1. UI Surface (e.x.: Title Bar, Panel, Tab Bar)
1. Elevated Surface (e.x.: Palette, Notification, Floating Window)
1. Wash
1. Modal Surfaces (e.x.: Modal)
1. Dragged Element (This is a special case, see Layer section below)

### App Background

The app background constitutes the lowest elevation layer, appearing behind all other surfaces and components. It is predominantly used for the background color of the app.

### Surface

The Surface elevation level, located above the app background, is the standard level for all elements

Example Elements: Title Bar, Panel, Tab Bar, Editor

### Elevated Surface

Non-Modal Elevated Surfaces appear above the UI surface layer and is used for things that should appear above most UI elements like an editor or panel, but not elements like popovers, context menus, modals, etc.

Examples: Notifications, Palettes, Detached/Floating Windows, Detached/Floating Panels

You could imagine a variant of the assistant that floats in a window above the editor on this elevation, or a floating terminal window that becomes less opaque when not focused.

### Wash

Wash denotes a distinct elevation reserved to isolate app UI layers from high elevation components such as modals, notifications, and overlaid panels. The wash may not consistently be visible when these components are active. This layer is often referred to as a scrim or overlay and the background color of the wash is typically deployed in its design.

### Modal Surfaces

Modal Surfaces are used for elements that should appear above all other UI elements and are located above the wash layer. This is the maximum elevation at which UI elements can be rendered

Elements rendered at this layer have an enforced behavior: Any interaction outside of the modal will either dismiss the modal or prompt an action (Save your progress, etc) then dismiss the modal.

If the element does not have this behavior, it should be rendered at the Elevated Surface layer.

## Layer
Each elevation that can contain elements has its own set of layers that are nested within the elevations.

1. TBD (Z -1 layer)
1. Element (Text, button, surface, etc)
1. Elevated Element (Popover, Context Menu, Tooltip)
999. Dragged Element -> Highest Elevation

Dragged elements jump to the highest elevation the app can render. An active drag should _always_ be the most foreground element in the app at any time.

🚧 Work in Progress 🚧

## Element
Each elevation that can contain elements has it's own set of layers:

1. Effects
1. Background
1. Tint
1. Highlight
1. Content
1. Overlay

🚧 Work in Progress 🚧
