import { Theme } from "@theme/config"
import { TokenFamily, tokens } from "@theme/tokens"
import { buildSurfaceLevels } from "."

export function buildSurfaceTokens(theme: Theme): void {
    const surface = buildSurfaceLevels(theme)

    const surfaceTokens: TokenFamily = {
        background: {
            bg: tokens.colorToken(surface.background.background),
            border: tokens.colorToken(surface.background.border.color),
        },
        pane: {
            bg: tokens.colorToken(surface.pane.background),
            border: tokens.colorToken(surface.pane.border.color),
        },
        panel: {
            bg: tokens.colorToken(surface.panel.background),
            border: tokens.colorToken(surface.panel.border.color),
        },
    }

    // Push tokens into the global token object
    tokens.addToToken("surface", surfaceTokens)
}
