# Elevation

Elevation in Zed applies to all surfaces and components. Elevation is categorized into levels.

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

## Elevation Levels

Zed integrates six unique elevation levels in its design system. The elevation of a surface is expressed as a whole number ranging from 0 to 5, both numbers inclusive. A component’s elevation is ascertained by combining the component’s resting elevation with any dynamic elevation offsets.

The levels are detailed as follows:

0. App Background
1. UI Surface
2. Elevated Elements
3. Wash
4. Focused Element
5. Dragged Element

### 0. App Background

The app background constitutes the lowest elevation layer, appearing behind all other surfaces and components. It is predominantly used for the background color of the app.

### 1. UI Surface

The UI Surface is the standard elevation for components and is placed above the app background. It is generally used for the background color of the app bar, card, and sheet.

### 2. Elevated Elements

Elevated elements appear above the UI surface layer surfaces and components. Elevated elements are predominantly used for creating popovers, context menus, and tooltips.

### 3. Wash

Wash denotes a distinct elevation reserved to isolate app UI layers from high elevation components such as modals, notifications, and overlaid panels. The wash may not consistently be visible when these components are active. This layer is often referred to as a scrim or overlay and the background color of the wash is typically deployed in its design.

### 4. Focused Element

Focused elements obtain a higher elevation above surfaces and components at wash elevation. They are often used for modals, notifications, and overlaid panels and indicate that they are the sole element the user is interacting with at the moment.

### 5. Dragged Element

Dragged elements gain the highest elevation, thus appearing above surfaces and components at the elevation of focused elements. These are typically used for elements that are being dragged, following the cursor
