import { useTheme } from "../common"
import { toggleable_icon_button } from "../component/icon_button"
import { interactive } from "../element"
import { background, border, foreground, text } from "./components"

export const toolbar = () => {
    const theme = useTheme()

    return {
        height: 42,
        padding: { left: 8, right: 8 },
        background: background(theme.highest),
        border: border(theme.highest, { bottom: true }),
        item_spacing: 4,
        toggleable_tool: toggleable_icon_button({
            margin: { left: 4 },
            variant: "ghost",
            active_color: "accent",
        }),
        breadcrumb_height: 24,
        breadcrumbs: interactive({
            base: {
                ...text(theme.highest, "sans", "variant"),
                corner_radius: 6,
                padding: {
                    left: 4,
                    right: 4,
                }
            },
            state: {
                hovered: {
                    color: foreground(theme.highest, "on", "hovered"),
                    background: background(theme.highest, "on", "hovered"),
                },
            },
        }),
    }
}
