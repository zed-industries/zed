import * as fs from "fs"
import * as path from "path"
import { ColorScheme, createColorScheme } from "./common"
import { themes } from "./themes"
import { slugify } from "./utils/slugify"
import { colorSchemeTokens } from "./theme/tokens/colorScheme"

const TOKENS_DIRECTORY = path.join(__dirname, "..", "target", "tokens")
const TOKENS_FILE = path.join(TOKENS_DIRECTORY, "$themes.json")
const METADATA_FILE = path.join(TOKENS_DIRECTORY, "$metadata.json")

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

type TokenSet = {
    id: string
    name: string
    selectedTokenSets: { [key: string]: "enabled" }
}

function buildTokenSetOrder(colorSchemes: ColorScheme[]): {
    tokenSetOrder: string[]
} {
    const tokenSetOrder: string[] = colorSchemes.map((scheme) =>
        scheme.name.toLowerCase().replace(/\s+/g, "_")
    )
    return { tokenSetOrder }
}

function buildThemesIndex(colorSchemes: ColorScheme[]): TokenSet[] {
    const themesIndex: TokenSet[] = colorSchemes.map((scheme, index) => {
        const id = `${scheme.isLight ? "light" : "dark"}_${scheme.name
            .toLowerCase()
            .replace(/\s+/g, "_")}_${index}`
        const selectedTokenSets: { [key: string]: "enabled" } = {}
        const tokenSet = scheme.name.toLowerCase().replace(/\s+/g, "_")
        selectedTokenSets[tokenSet] = "enabled"

        return {
            id,
            name: `${scheme.name} - ${scheme.isLight ? "Light" : "Dark"}`,
            selectedTokenSets,
        }
    })

    return themesIndex
}

function writeTokens(colorSchemes: ColorScheme[], tokensDirectory: string) {
    clearTokens(tokensDirectory)

    for (const colorScheme of colorSchemes) {
        const fileName = slugify(colorScheme.name) + ".json"
        const tokens = colorSchemeTokens(colorScheme)
        const tokensJSON = JSON.stringify(tokens, null, 2)
        const outPath = path.join(tokensDirectory, fileName)
        fs.writeFileSync(outPath, tokensJSON, { mode: 0o644 })
        console.log(`- ${outPath} created`)
    }

    const themeIndexData = buildThemesIndex(colorSchemes)

    const themesJSON = JSON.stringify(themeIndexData, null, 2)
    fs.writeFileSync(TOKENS_FILE, themesJSON, { mode: 0o644 })
    console.log(`- ${TOKENS_FILE} created`)

    const tokenSetOrderData = buildTokenSetOrder(colorSchemes)

    const metadataJSON = JSON.stringify(tokenSetOrderData, null, 2)
    fs.writeFileSync(METADATA_FILE, metadataJSON, { mode: 0o644 })
    console.log(`- ${METADATA_FILE} created`)
}

const colorSchemes: ColorScheme[] = themes.map((theme) =>
    createColorScheme(theme)
)

writeTokens(colorSchemes, TOKENS_DIRECTORY)
