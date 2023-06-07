import * as fs from "fs"
import { tmpdir } from "os"
import * as path from "path"
import app from "./styleTree/app"
import { ColorScheme, createColorScheme } from "./themes/common/colorScheme"
import snakeCase from "./utils/snakeCase"
import { themes } from "./themes"

const assetsDirectory = `${__dirname}/../../assets`
const tempDirectory = fs.mkdtempSync(path.join(tmpdir(), "build-themes"))

// Clear existing themes
function clearThemes(themeDirectory: string) {
    if (!fs.existsSync(themeDirectory)) {
        fs.mkdirSync(themeDirectory, { recursive: true })
    } else {
        for (const file of fs.readdirSync(themeDirectory)) {
            if (file.endsWith(".json")) {
                fs.unlinkSync(path.join(themeDirectory, file))
            }
        }
    }
}

function writeThemes(colorSchemes: ColorScheme[], outputDirectory: string) {
    clearThemes(outputDirectory)
    for (let colorScheme of colorSchemes) {
        let styleTree = snakeCase(app(colorScheme))
        let styleTreeJSON = JSON.stringify(styleTree, null, 2)
        let tempPath = path.join(tempDirectory, `${colorScheme.name}.json`)
        let outPath = path.join(outputDirectory, `${colorScheme.name}.json`)
        fs.writeFileSync(tempPath, styleTreeJSON)
        fs.renameSync(tempPath, outPath)
        console.log(`- ${outPath} created`)
    }
}

const colorSchemes: ColorScheme[] = themes.map((theme) => createColorScheme(theme))

// Write new themes to theme directory
writeThemes(colorSchemes, `${assetsDirectory}/themes`)
