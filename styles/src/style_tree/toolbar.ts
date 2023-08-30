import { useTheme } from "../common"
import { toggleable_icon_button } from "../component/icon_button"
import { interactive, toggleable } from "../element"
import { background, border, foreground, text } from "./components"
import { text_button } from "../component";

export const toolbar = () => {
    const theme = useTheme()

    return {
        height: 32,
        padding: { left: 4, right: 4, top: 4, bottom: 4 },
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
                    left: 6,
                    right: 6,
                },
            },
            state: {
                hovered: {
                    color: foreground(theme.highest, "on", "hovered"),
                    background: background(theme.highest, "on", "hovered"),
                },
            },
        }),
        toggleable_text_tool: toggleable({
            state: {
                inactive: text_button({ variant: "ghost", layer: theme.highest, disabled: true, margin: { right: 4 }, text_properties: { size: "sm" } }),
                active: text_button({ variant: "ghost", layer: theme.highest, margin: { right: 4 }, text_properties: { size: "sm" } })
            }
        }),
    }
}
