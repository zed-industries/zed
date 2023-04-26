import * as themes from "../../themes"
import { buildUI } from "../../ui"

export function useTheme() {
    const dark = themes.zedDark
    const ui = buildUI(dark)

    const theme = {
        config: dark,
        ui,
    }

    return theme
}
