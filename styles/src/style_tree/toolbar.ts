import { useTheme } from "../common"
import { toggleable_icon_button } from "../component/icon_button"
import { background, border } from "./components"

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
    }
}
