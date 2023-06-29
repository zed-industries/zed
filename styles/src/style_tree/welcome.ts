import { ColorScheme } from "../theme/color_scheme"
import { withOpacity } from "../theme/color"
import {
    border,
    background,
    foreground,
    text,
    TextProperties,
    svg,
} from "./components"
import { interactive } from "../element"

export default function welcome(colorScheme: ColorScheme): any {
    const layer = colorScheme.highest

    const checkboxBase = {
        corner_radius: 4,
        padding: {
            left: 3,
            right: 3,
            top: 3,
            bottom: 3,
        },
        // shadow: colorScheme.popoverShadow,
        border: border(layer),
        margin: {
            right: 8,
            top: 5,
            bottom: 5,
        },
    }

    const interactive_text_size: TextProperties = { size: "sm" }

    return {
        pageWidth: 320,
        logo: svg(foreground(layer, "default"), "icons/logo_96.svg", 64, 64),
        logoSubheading: {
            ...text(layer, "sans", "variant", { size: "md" }),
            margin: {
                top: 10,
                bottom: 7,
            },
        },
        buttonGroup: {
            margin: {
                top: 8,
                bottom: 16,
            },
        },
        headingGroup: {
            margin: {
                top: 8,
                bottom: 12,
            },
        },
        checkboxGroup: {
            border: border(layer, "variant"),
            background: withOpacity(background(layer, "hovered"), 0.25),
            corner_radius: 4,
            padding: {
                left: 12,
                top: 2,
                bottom: 2,
            },
        },
        button: interactive({
            base: {
                background: background(layer),
                border: border(layer, "active"),
                corner_radius: 4,
                margin: {
                    top: 4,
                    bottom: 4,
                },
                padding: {
                    top: 3,
                    bottom: 3,
                    left: 7,
                    right: 7,
                },
                ...text(layer, "sans", "default", interactive_text_size),
            },
            state: {
                hovered: {
                    ...text(layer, "sans", "default", interactive_text_size),
                    background: background(layer, "hovered"),
                },
            },
        }),

        usageNote: {
            ...text(layer, "sans", "variant", { size: "2xs" }),
            padding: {
                top: -4,
            },
        },
        checkboxContainer: {
            margin: {
                top: 4,
            },
            padding: {
                bottom: 8,
            },
        },
        checkbox: {
            label: {
                ...text(layer, "sans", interactive_text_size),
                // Also supports margin, container, border, etc.
            },
            icon: svg(foreground(layer, "on"), "icons/check_12.svg", 12, 12),
            default: {
                ...checkboxBase,
                background: background(layer, "default"),
                border: border(layer, "active"),
            },
            checked: {
                ...checkboxBase,
                background: background(layer, "hovered"),
                border: border(layer, "active"),
            },
            hovered: {
                ...checkboxBase,
                background: background(layer, "hovered"),
                border: border(layer, "active"),
            },
            hoveredAndChecked: {
                ...checkboxBase,
                background: background(layer, "hovered"),
                border: border(layer, "active"),
            },
        },
    }
}
