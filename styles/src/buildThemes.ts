import * as fs from "fs"
import { tmpdir } from "os"
import * as path from "path"
import { colorSchemes, staffColorSchemes } from "./colorSchemes"
import app from "./styleTree/app"
import { ColorScheme } from "./themes/common/colorScheme"
import snakeCase from "./utils/snakeCase"

const assetsDirectory = `${__dirname}/../../assets`
const themeDirectory = `${assetsDirectory}/themes`
const staffDirectory = `${themeDirectory}/staff`

const tempDirectory = fs.mkdtempSync(path.join(tmpdir(), "build-themes"))

// Clear existing themes
function clearThemes(themeDirectory: string) {
    if (!fs.existsSync(themeDirectory)) {
        fs.mkdirSync(themeDirectory, { recursive: true })
    } else {
        for (const file of fs.readdirSync(themeDirectory)) {
            if (file.endsWith(".json")) {
                const name = file.replace(/\.json$/, "")
                if (
                    !colorSchemes.find(
                        (colorScheme) => colorScheme.name === name
                    )
                ) {
                    fs.unlinkSync(path.join(themeDirectory, file))
                }
            }
        }
    }
}

clearThemes(themeDirectory)
clearThemes(staffDirectory)

function writeThemes(colorSchemes: ColorScheme[], outputDirectory: string) {
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

// Write new themes to theme directory
writeThemes(colorSchemes, themeDirectory)
writeThemes(staffColorSchemes, staffDirectory)
