import * as fs from "fs"
import { tmpdir } from "os"
import * as path from "path"
import app from "./style_tree/app"
import { ColorScheme, create_color_scheme } from "./theme/color_scheme"
import { themes } from "./themes"

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

function write_themes(themes: ColorScheme[], output_directory: string) {
    clear_themes(output_directory)
    for (const color_scheme of themes) {
        const style_tree = app(color_scheme)
        const style_tree_json = JSON.stringify(style_tree, null, 2)
        const temp_path = path.join(temp_directory, `${color_scheme.name}.json`)
        const out_path = path.join(
            output_directory,
            `${color_scheme.name}.json`
        )
        fs.writeFileSync(temp_path, style_tree_json)
        fs.renameSync(temp_path, out_path)
        console.log(`- ${out_path} created`)
    }
}

const all_themes: ColorScheme[] = themes.map((theme) =>
    create_color_scheme(theme)
)

write_themes(all_themes, `${assets_directory}/themes`)
