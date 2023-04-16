import { buttonWithIconStyle } from "@/components"
import { Theme } from "@/theme"

export const find = (theme: Theme) => {
    return {
        button: buttonWithIconStyle(theme),
    }
}
