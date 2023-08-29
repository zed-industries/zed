import { useTheme } from "../common"
import { toggleable_icon_button } from "../component/icon_button"
import { background, border } from "./components"

export const toolbar = () => {
    const theme = useTheme()

    return {
        height: 42,
        background: background(theme.highest),
        border: border(theme.highest, { bottom: true }),
        item_spacing: 8,
        toggleable_tool: toggleable_icon_button(theme, {
            margin: { left: 8 },
            variant: "ghost",
            active_color: "accent",
        }),
        padding: { left: 8, right: 8 },
    }
}
