import { ColorScheme } from "../themes/common/colorScheme"
import { withOpacity } from "../utils/color"
import { background, border, foreground, text } from "./components"

export default function projectPanel(colorScheme: ColorScheme) {
    const { isLight } = colorScheme

    let layer = colorScheme.middle

    let baseEntry = {
        height: 24,
        iconColor: foreground(layer, "variant"),
        iconSize: 8,
        iconSpacing: 8,
    }

    let status = {
        git: {
            modified: isLight
                ? colorScheme.ramps.yellow(0.6).hex()
                : colorScheme.ramps.yellow(0.5).hex(),
            inserted: isLight
                ? colorScheme.ramps.green(0.45).hex()
                : colorScheme.ramps.green(0.5).hex(),
            conflict: isLight
                ? colorScheme.ramps.red(0.6).hex()
                : colorScheme.ramps.red(0.5).hex()
        }
    }

    let entry = {
        ...baseEntry,
        text: text(layer, "mono", "variant", { size: "sm" }),
        hover: {
            background: background(layer, "variant", "hovered"),
        },
        active: {
            background: colorScheme.isLight
                ? withOpacity(background(layer, "active"), 0.5)
                : background(layer, "active"),
            text: text(layer, "mono", "active", { size: "sm" }),
        },
        activeHover: {
            background: background(layer, "active"),
            text: text(layer, "mono", "active", { size: "sm" }),
        },
        status
    }

    return {
        openProjectButton: {
            background: background(layer),
            border: border(layer, "active"),
            cornerRadius: 4,
            margin: {
                top: 16,
                left: 16,
                right: 16,
            },
            padding: {
                top: 3,
                bottom: 3,
                left: 7,
                right: 7,
            },
            ...text(layer, "sans", "default", { size: "sm" }),
            hover: {
                ...text(layer, "sans", "default", { size: "sm" }),
                background: background(layer, "hovered"),
                border: border(layer, "active"),
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
            status
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
