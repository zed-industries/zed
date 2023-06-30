import { ColorScheme } from "../theme/color_scheme"
import { with_opacity } from "../theme/color"
import {
    border,
    background,
    foreground,
    text,
    TextProperties,
    svg,
} from "./components"
import { interactive } from "../element"

export default function welcome(theme: ColorScheme): any {
    const checkbox_base = {
        corner_radius: 4,
        padding: {
            left: 3,
            right: 3,
            top: 3,
            bottom: 3,
        },
        // shadow: theme.popover_shadow,
        border: border(theme.highest),
        margin: {
            right: 8,
            top: 5,
            bottom: 5,
        },
    }

    const interactive_text_size: TextProperties = { size: "sm" }

    return {
        page_width: 320,
        logo: svg(
            foreground(theme.highest, "default"),
            "icons/logo_96.svg",
            64,
            64
        ),
        logo_subheading: {
            ...text(theme.highest, "sans", "variant", { size: "md" }),
            margin: {
                top: 10,
                bottom: 7,
            },
        },
        button_group: {
            margin: {
                top: 8,
                bottom: 16,
            },
        },
        heading_group: {
            margin: {
                top: 8,
                bottom: 12,
            },
        },
        checkbox_group: {
            border: border(theme.highest, "variant"),
            background: with_opacity(
                background(theme.highest, "hovered"),
                0.25
            ),
            corner_radius: 4,
            padding: {
                left: 12,
                top: 2,
                bottom: 2,
            },
        },
        button: interactive({
            base: {
                background: background(theme.highest),
                border: border(theme.highest, "active"),
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
                ...text(
                    theme.highest,
                    "sans",
                    "default",
                    interactive_text_size
                ),
            },
            state: {
                hovered: {
                    ...text(
                        theme.highest,
                        "sans",
                        "default",
                        interactive_text_size
                    ),
                    background: background(theme.highest, "hovered"),
                },
            },
        }),

        usage_note: {
            ...text(theme.highest, "sans", "variant", { size: "2xs" }),
            padding: {
                top: -4,
            },
        },
        checkbox_container: {
            margin: {
                top: 4,
            },
            padding: {
                bottom: 8,
            },
        },
        checkbox: {
            label: {
                ...text(theme.highest, "sans", interactive_text_size),
                // Also supports margin, container, border, etc.
            },
            icon: svg(
                foreground(theme.highest, "on"),
                "icons/check_12.svg",
                12,
                12
            ),
            default: {
                ...checkbox_base,
                background: background(theme.highest, "default"),
                border: border(theme.highest, "active"),
            },
            checked: {
                ...checkbox_base,
                background: background(theme.highest, "hovered"),
                border: border(theme.highest, "active"),
            },
            hovered: {
                ...checkbox_base,
                background: background(theme.highest, "hovered"),
                border: border(theme.highest, "active"),
            },
            hovered_and_checked: {
                ...checkbox_base,
                background: background(theme.highest, "hovered"),
                border: border(theme.highest, "active"),
            },
        },
    }
}
