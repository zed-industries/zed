import * as themeConfigs from "@/themes"
import { buildUI } from "@/ui"
import { buildTheme } from "./buildTheme"
import { EXPORT_PATH, writeToDisk } from "@/lib/export"
import { buildTokens } from "./tokens"

export function buildThemes(): void {
    for (const themeConfig of Object.values(themeConfigs)) {
        const theme = buildTheme(themeConfig)
        const ui = buildUI(theme)
        const json = JSON.stringify(ui)
        buildTokens(theme.name)
        writeToDisk(theme.name, json, EXPORT_PATH)
    }
}
