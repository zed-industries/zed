import { EXPORT_PATH, writeToDisk } from "@/lib/export"
import { tokens } from "./tokens"
import { slugify } from "@/lib/slugify"

export function writeTokens(themeName: string): void {
    const tokensName = slugify(themeName)
    const path = `${EXPORT_PATH}/tokens`
    const json = JSON.stringify(tokens.values, null, 2)

    writeToDisk(`${tokensName}_tokens`, json, path)
}
