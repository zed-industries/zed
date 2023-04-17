import { iconButton, toggleButton } from "@/components/button"
import { Theme } from "@/theme"

export const find = (theme: Theme) => {
    return {
        next_button: iconButton(theme),
        previous_button: iconButton(theme),
        case_button: toggleButton(theme),
        word_button: toggleButton(theme),
        regex_button: toggleButton(theme),
    }
}
