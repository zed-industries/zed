import * as fs from "fs"
import * as path from "path"
import { ColorScheme, createColorScheme } from "./common"
import { themes } from "./themes"
import { slugify } from "./utils/slugify"
import { colorSchemeTokens } from "./theme/tokens/colorScheme"

const TOKENS_DIRECTORY = path.join(__dirname, "..", "target", "tokens")

function clearTokens(tokensDirectory: string) {
    if (!fs.existsSync(tokensDirectory)) {
        fs.mkdirSync(tokensDirectory, { recursive: true })
    } else {
        for (const file of fs.readdirSync(tokensDirectory)) {
            if (file.endsWith(".json")) {
                fs.unlinkSync(path.join(tokensDirectory, file))
            }
        }
    }
}

function writeTokens(colorSchemes: ColorScheme[], tokensDirectory: string) {
    clearTokens(tokensDirectory)

    for (const colorScheme of colorSchemes) {
        const fileName = slugify(colorScheme.name)
        const tokens = colorSchemeTokens(colorScheme)
        const tokensJSON = JSON.stringify(tokens, null, 2)
        const outPath = path.join(tokensDirectory, `${fileName}.json`)
        fs.writeFileSync(outPath, tokensJSON)
        console.log(`- ${outPath} created`)
    }
}

const colorSchemes: ColorScheme[] = themes.map((theme) =>
    createColorScheme(theme)
)

writeTokens(colorSchemes, TOKENS_DIRECTORY)
