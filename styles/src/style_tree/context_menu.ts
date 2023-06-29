import { ColorScheme } from "../theme/color_scheme"
import { background, border, border_color, text } from "./components"
import { interactive, toggleable } from "../element"

export default function context_menu(theme: ColorScheme): any {
    return {
        background: background(theme.middle),
        corner_radius: 10,
        padding: 4,
        shadow: theme.popover_shadow,
        border: border(theme.middle),
        keystroke_margin: 30,
        item: toggleable({
            base: interactive({
                base: {
                    icon_spacing: 8,
                    icon_width: 14,
                    padding: { left: 6, right: 6, top: 2, bottom: 2 },
                    corner_radius: 6,
                    label: text(theme.middle, "sans", { size: "sm" }),
                    keystroke: {
                        ...text(theme.middle, "sans", "variant", {
                            size: "sm",
                            weight: "bold",
                        }),
                        padding: { left: 3, right: 3 },
                    },
                },
                state: {
                    hovered: {
                        background: background(theme.middle, "hovered"),
                        label: text(theme.middle, "sans", "hovered", { size: "sm" }),
                        keystroke: {
                            ...text(theme.middle, "sans", "hovered", {
                                size: "sm",
                                weight: "bold",
                            }),
                            padding: { left: 3, right: 3 },
                        },
                    },
                    clicked: {
                        background: background(theme.middle, "pressed"),
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        background: background(theme.middle, "active"),
                    },
                    hovered: {
                        background: background(theme.middle, "hovered"),
                    },
                    clicked: {
                        background: background(theme.middle, "pressed"),
                    },
                },
            },
        }),

        separator: {
            background: border_color(theme.middle),
            margin: { top: 2, bottom: 2 },
        },
    }
}
