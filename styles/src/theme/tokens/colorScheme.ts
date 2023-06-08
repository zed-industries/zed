import { ColorScheme } from "../colorScheme"
import { PlayerTokens, players } from "./players"

interface ColorSchemeTokens {
    players: PlayerTokens
}

export function colorSchemeTokens(colorScheme: ColorScheme): ColorSchemeTokens {
    return {
        players: players(colorScheme),
    }
}
