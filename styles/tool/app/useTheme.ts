import { zedLight } from "../../themes"
import { buildTheme } from "../../theme/buildTheme"
import { buildUI } from "../../ui"

export function useTheme() {
    const themeConfig = zedLight
    const theme = buildTheme(themeConfig)
    const ui = buildUI(theme)

    return {
        config: themeConfig,
        ui,
    }
}
