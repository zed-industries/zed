import { iconButton } from "@components/button"
import { Theme } from "@/theme"

export const find = (theme: Theme) => {
    return {
        next_button: iconButton(theme),
        previous_button: iconButton(theme),
        case_button: iconButton(theme),
        word_button: iconButton(theme),
        regex_button: iconButton(theme),
    }
}
