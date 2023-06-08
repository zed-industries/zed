import { ColorScheme } from "../colorScheme"
import { LayerToken, layerToken } from "./layer"
import { PlayerTokens, players } from "./players"

interface ColorSchemeTokens {
    lowest: LayerToken
    middle: LayerToken
    highest: LayerToken
    players: PlayerTokens
}

export function colorSchemeTokens(colorScheme: ColorScheme): ColorSchemeTokens {
    return {
        lowest: layerToken(colorScheme.lowest, "lowest"),
        middle: layerToken(colorScheme.middle, "middle"),
        highest: layerToken(colorScheme.highest, "highest"),
        players: players(colorScheme),
    }
}
