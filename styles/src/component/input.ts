import { useTheme } from "../common"
import { background, border, text } from "../style_tree/components"

export const input = () => {
    const theme = useTheme()

    return {
        background: background(theme.highest),
        corner_radius: 8,
        min_width: 200,
        max_width: 500,
        placeholder_text: text(theme.highest, "mono", "disabled"),
        selection: theme.players[0],
        text: text(theme.highest, "mono", "default"),
        border: border(theme.highest),
        padding: {
            top: 3,
            bottom: 3,
            left: 12,
            right: 8,
        },
    }
}
