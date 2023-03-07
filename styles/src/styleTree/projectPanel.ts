import { ColorScheme } from "../themes/common/colorScheme"
import { withOpacity } from "../utils/color"
import { background, border, foreground, text } from "./components"

export default function projectPanel(colorScheme: ColorScheme) {
    let layer = colorScheme.middle

    let baseEntry = {
        height: 24,
        iconColor: foreground(layer, "variant"),
        iconSize: 8,
        iconSpacing: 8,
    }

    let entry = {
        ...baseEntry,
        text: text(layer, "mono", "variant", { size: "sm" }),
        hover: {
            background: background(layer, "variant", "hovered"),
        },
        active: {
            background: background(layer, "active"),
            text: text(layer, "mono", "active", { size: "sm" }),
        },
        activeHover: {
            background: background(layer, "active"),
            text: text(layer, "mono", "active", { size: "sm" }),
        },
    }

    return {
        openProjectButton: {
            ...text(layer, "mono", "active", { size: "sm" }),
            background: background(layer, "on"),
            cornerRadius: 6,
            border: border(layer, "on"),
            margin: {
                top: 20,
                left: 10,
                right: 10
            },
            padding: {
                bottom: 2,
                left: 10,
                right: 10,
                top: 2,
            },
            active: {
                ...text(layer, "mono", "on", "inverted"),
                background: background(layer, "on", "inverted"),
                border: border(layer, "on", "inverted"),
            },
            clicked: {
                ...text(layer, "mono", "on", "pressed"),
                background: background(layer, "on", "pressed"),
                border: border(layer, "on", "pressed"),
            },
            hover: {
                ...text(layer, "mono", "on", "hovered"),
                background: background(layer, "on", "hovered"),
                border: border(layer, "on", "hovered"),
            },

        },
        background: background(layer),
        padding: { left: 12, right: 12, top: 6, bottom: 6 },
        indentWidth: 8,
        entry,
        draggedEntry: {
            ...baseEntry,
            text: text(layer, "mono", "on", { size: "sm" }),
            background: withOpacity(background(layer, "on"), 0.9),
            border: border(layer),
        },
        ignoredEntry: {
            ...entry,
            text: text(layer, "mono", "disabled"),
        },
        cutEntry: {
            ...entry,
            text: text(layer, "mono", "disabled"),
            active: {
                background: background(layer, "active"),
                text: text(layer, "mono", "disabled", { size: "sm" }),
            },
        },
        filenameEditor: {
            background: background(layer, "on"),
            text: text(layer, "mono", "on", { size: "sm" }),
            selection: colorScheme.players[0],
        },
    }
}
