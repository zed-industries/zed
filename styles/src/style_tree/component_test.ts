
import { useTheme } from "../common"
import { text_button } from "../component/text_button"
import { icon_button } from "../component/icon_button"
import { text } from "./components"
import { toggleable } from "../element"

export default function contacts_panel(): any {
    const theme = useTheme()

    return {
        button: text_button({}),
        toggle: toggleable({
            base: text_button({}),
            state: {
                active: {
                    ...text_button({ color: "accent" })
                }
            }
        }),
        disclosure: {
            ...text(theme.lowest, "sans", "base"),
            button: icon_button({ variant: "ghost" }),
            spacing: 4,
        }
    }
}
