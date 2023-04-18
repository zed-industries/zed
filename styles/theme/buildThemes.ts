import * as themeConfigs from "@/themes"
import { buildUI } from "@/ui"
import { buildTheme } from "./buildTheme"
import { EXPORT_PATH, writeToDisk } from "@/lib/export"
import { writeTokens } from "./tokens"
import { buildComponents } from "@components/buildComponents"

export function buildThemes(): void {
    for (const themeConfig of Object.values(themeConfigs)) {
        // ThemeConfig => Theme
        const theme = buildTheme(themeConfig)

        // Build the common components used acrossed UI elements
        buildComponents(theme)

        const ui = buildUI(theme)

        // Write outputs
        writeTokens(theme.name)

        const json = JSON.stringify(ui)
        writeToDisk(theme.name, json, EXPORT_PATH)
    }
}
