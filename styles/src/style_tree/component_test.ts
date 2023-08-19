import { toggle_label_button_style } from "../component/label_button"
import { useTheme } from "../common"
import { text_button } from "../component/text_button"
import { toggleable_icon_button } from "../component/icon_button"
import { text } from "./components"

export default function contacts_panel(): any {
    const theme = useTheme()

    return {
        button: text_button({}),
        toggle: toggle_label_button_style({ active_color: "accent" }),
        disclosure: {
            ...text(theme.lowest, "sans", "base"),
            button: toggleable_icon_button(theme, {}),
            spacing: 4,
        }
    }
}
