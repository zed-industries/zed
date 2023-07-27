import * as fs from "fs"
import * as path from "path"
import { Theme, create_theme, useThemeStore } from "./common"
import { themes } from "./themes"
import { slugify } from "./utils/slugify"
import { theme_tokens } from "./theme/tokens/theme"

const TOKENS_DIRECTORY = path.join(__dirname, "..", "target", "tokens")
const TOKENS_FILE = path.join(TOKENS_DIRECTORY, "$themes.json")
const METADATA_FILE = path.join(TOKENS_DIRECTORY, "$metadata.json")

function clear_tokens(tokens_directory: string) {
    if (!fs.existsSync(tokens_directory)) {
        fs.mkdirSync(tokens_directory, { recursive: true })
    } else {
        for (const file of fs.readdirSync(tokens_directory)) {
            if (file.endsWith(".json")) {
                fs.unlinkSync(path.join(tokens_directory, file))
            }
        }
    }
}

type TokenSet = {
    id: string
    name: string
    selected_token_sets: { [key: string]: "enabled" }
}

function build_token_set_order(theme: Theme[]): {
    token_set_order: string[]
} {
    const token_set_order: string[] = theme.map((scheme) =>
        scheme.name.toLowerCase().replace(/\s+/g, "_")
    )
    return { token_set_order }
}

function build_themes_index(theme: Theme[]): TokenSet[] {
    const themes_index: TokenSet[] = theme.map((scheme, index) => {
        const id = `${scheme.is_light ? "light" : "dark"}_${scheme.name
            .toLowerCase()
            .replace(/\s+/g, "_")}_${index}`
        const selected_token_sets: { [key: string]: "enabled" } = {}
        const token_set = scheme.name.toLowerCase().replace(/\s+/g, "_")
        selected_token_sets[token_set] = "enabled"

        return {
            id,
            name: `${scheme.name} - ${scheme.is_light ? "Light" : "Dark"}`,
            selected_token_sets,
        }
    })

    return themes_index
}

function write_tokens(themes: Theme[], tokens_directory: string) {
    clear_tokens(tokens_directory)

    for (const theme of themes) {
        const { setTheme } = useThemeStore.getState()
        setTheme(theme)

        const file_name = slugify(theme.name) + ".json"
        const tokens = theme_tokens()
        const tokens_json = JSON.stringify(tokens, null, 2)
        const out_path = path.join(tokens_directory, file_name)
        fs.writeFileSync(out_path, tokens_json, { mode: 0o644 })
        console.log(`- ${out_path} created`)
    }

    const theme_index_data = build_themes_index(themes)

    const themes_json = JSON.stringify(theme_index_data, null, 2)
    fs.writeFileSync(TOKENS_FILE, themes_json, { mode: 0o644 })
    console.log(`- ${TOKENS_FILE} created`)

    const token_set_order_data = build_token_set_order(themes)

    const metadata_json = JSON.stringify(token_set_order_data, null, 2)
    fs.writeFileSync(METADATA_FILE, metadata_json, { mode: 0o644 })
    console.log(`- ${METADATA_FILE} created`)
}

const all_themes: Theme[] = themes.map((theme) =>
    create_theme(theme)
)

write_tokens(all_themes, TOKENS_DIRECTORY)
