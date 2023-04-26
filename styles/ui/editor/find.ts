import { iconButton, labelButton } from "@components/button"
import { Theme } from "@/theme"

export const find = (theme: Theme) => {
    return {
        next_button: iconButton(theme),
        previous_button: iconButton(theme),
        case_button: labelButton(theme),
        word_button: labelButton(theme),
        regex_button: labelButton(theme),
    }
}
