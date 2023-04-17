import { iconButton } from "@components/button"
import { Theme } from "@/theme"
import { toggleButton } from "@components/toggleButton"

export const find = (theme: Theme) => {
    return {
        next_button: iconButton(theme),
        previous_button: iconButton(theme),
        case_button: toggleButton(theme),
        word_button: toggleButton(theme),
        regex_button: toggleButton(theme),
    }
}
