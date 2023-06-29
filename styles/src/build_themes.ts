import * as fs from "fs"
import { tmpdir } from "os"
import * as path from "path"
import app from "./style_tree/app"
import { ColorScheme, createColorScheme } from "./theme/color_scheme"
import snakeCase from "./utils/snake_case"
import { themes } from "./themes"

const assets_directory = `${__dirname}/../../assets`
const temp_directory = fs.mkdtempSync(path.join(tmpdir(), "build-themes"))

// Clear existing themes
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

function write_themes(color_schemes: ColorScheme[], output_directory: string) {
    clear_themes(output_directory)
    for (const color_scheme of color_schemes) {
        const style_tree = snakeCase(app(color_scheme))
        const style_tree_json = JSON.stringify(style_tree, null, 2)
        const temp_path = path.join(temp_directory, `${color_scheme.name}.json`)
        const out_path = path.join(output_directory, `${color_scheme.name}.json`)
        fs.writeFileSync(temp_path, style_tree_json)
        fs.renameSync(temp_path, out_path)
        console.log(`- ${out_path} created`)
    }
}

const color_schemes: ColorScheme[] = themes.map((theme) =>
    createColorScheme(theme)
)

// Write new themes to theme directory
write_themes(color_schemes, `${assets_directory}/themes`)
