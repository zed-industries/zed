import { ColorScheme } from "../theme/color_scheme"
import { background, border, text } from "./components"
import { interactive, toggleable } from "../element"
export default function dropdown_menu(theme: ColorScheme): any {
    return {
        row_height: 30,
        background: background(theme.middle),
        border: border(theme.middle),
        shadow: theme.popover_shadow,
        header: interactive({
            base: {
                ...text(theme.middle, "sans", { size: "sm" }),
                secondary_text: text(theme.middle, "sans", {
                    size: "sm",
                    color: "#aaaaaa",
                }),
                secondary_text_spacing: 10,
                padding: { left: 8, right: 8, top: 2, bottom: 2 },
                corner_radius: 6,
                background: background(theme.middle, "on"),
            },
            state: {
                hovered: {
                    background: background(theme.middle, "hovered"),
                },
                clicked: {
                    background: background(theme.middle, "pressed"),
                },
            },
        }),
        section_header: {
            ...text(theme.middle, "sans", { size: "sm" }),
            padding: { left: 8, right: 8, top: 8, bottom: 8 },
        },
        item: toggleable({
            base: interactive({
                base: {
                    ...text(theme.middle, "sans", { size: "sm" }),
                    secondary_text_spacing: 10,
                    secondary_text: text(theme.middle, "sans", { size: "sm" }),
                    padding: { left: 18, right: 18, top: 2, bottom: 2 },
                },
                state: {
                    hovered: {
                        background: background(theme.middle, "hovered"),
                        ...text(theme.middle, "sans", "hovered", { size: "sm" }),
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
                },
            },
        }),
    }
}
