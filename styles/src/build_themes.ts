import * as fs from "fs"
import { tmpdir } from "os"
import * as path from "path"
import app from "./style_tree/app"
import { Theme, create_theme } from "./theme/create_theme"
import { themes } from "./themes"
import { useThemeStore } from "./theme"

const assets_directory = `${__dirname}/../../assets`
const temp_directory = fs.mkdtempSync(path.join(tmpdir(), "build-themes"))

function clear_themes(theme_directory: string) {
    if (!fs.existsSync(theme_directory)) {
        fs.mkdirSync(theme_directory, { recursive: true })
    } else {
        for (const file of fs.readdirSync(theme_directory)) {
            if (file.endsWith(".json")) {
                fs.unlinkSync(path.join(theme_directory, file))
            }
        }
    }
}

const all_themes: Theme[] = themes.map((theme) => create_theme(theme))

function write_themes(themes: Theme[], output_directory: string) {
    clear_themes(output_directory)
    for (const theme of themes) {
        const { setTheme } = useThemeStore.getState()
        setTheme(theme)

        const style_tree = app()
        const style_tree_json = JSON.stringify(style_tree, null, 2)
        const temp_path = path.join(temp_directory, `${theme.name}.json`)
        const out_path = path.join(output_directory, `${theme.name}.json`)
        fs.writeFileSync(temp_path, style_tree_json)
        fs.renameSync(temp_path, out_path)
        console.log(`- ${out_path} created`)
    }
}

write_themes(all_themes, `${assets_directory}/themes`)
