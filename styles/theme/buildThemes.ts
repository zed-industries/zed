import fs from "fs"
import * as themeConfigs from "@/themes"
import { buildUI } from "@/ui"
import { buildTheme } from "./buildTheme"

const EXPORT_PATH = "./target"

export function writeThemeToDisk(
    name: string,
    json: string,
    path: string
): void {
    const slug = name.toLowerCase().replace(/ /g, "_")
    path = `${path}/${slug}.json`

    fs.writeFile(path, json, (err) => {
        if (err) {
            console.error(err)
            return
        }
        console.log(`Wrote ${name} to ${path}`)
    })
}

export function buildThemes(): void {
    for (const themeConfig of Object.values(themeConfigs)) {
        const theme = buildTheme(themeConfig)
        const ui = buildUI(theme)
        const json = JSON.stringify(ui)
        writeThemeToDisk(theme.name, json, EXPORT_PATH)
    }
}
