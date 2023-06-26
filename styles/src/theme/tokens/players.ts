import { SingleColorToken } from "@tokens-studio/types"
import { ColorScheme, Players } from "../../common"
import { colorToken } from "./token"

export type PlayerToken = Record<"selection" | "cursor", SingleColorToken>

export type PlayersToken = Record<keyof Players, PlayerToken>

function buildPlayerToken(
    colorScheme: ColorScheme,
    index: number
): PlayerToken {
    const playerNumber = index.toString() as keyof Players

    return {
        selection: colorToken(
            `player${index}Selection`,
            colorScheme.players[playerNumber].selection
        ),
        cursor: colorToken(
            `player${index}Cursor`,
            colorScheme.players[playerNumber].cursor
        ),
    }
}

export const playersToken = (colorScheme: ColorScheme): PlayersToken => ({
    "0": buildPlayerToken(colorScheme, 0),
    "1": buildPlayerToken(colorScheme, 1),
    "2": buildPlayerToken(colorScheme, 2),
    "3": buildPlayerToken(colorScheme, 3),
    "4": buildPlayerToken(colorScheme, 4),
    "5": buildPlayerToken(colorScheme, 5),
    "6": buildPlayerToken(colorScheme, 6),
    "7": buildPlayerToken(colorScheme, 7),
})
