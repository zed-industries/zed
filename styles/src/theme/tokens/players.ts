import { SingleColorToken } from "@tokens-studio/types"
import { color_token } from "./token"
import { ColorScheme, Players } from "../color_scheme"

export type PlayerToken = Record<"selection" | "cursor", SingleColorToken>

export type PlayersToken = Record<keyof Players, PlayerToken>

function build_player_token(theme: ColorScheme, index: number): PlayerToken {
    const player_number = index.toString() as keyof Players

    return {
        selection: color_token(
            `player${index}Selection`,
            theme.players[player_number].selection
        ),
        cursor: color_token(
            `player${index}Cursor`,
            theme.players[player_number].cursor
        ),
    }
}

export const players_token = (theme: ColorScheme): PlayersToken => ({
    "0": build_player_token(theme, 0),
    "1": build_player_token(theme, 1),
    "2": build_player_token(theme, 2),
    "3": build_player_token(theme, 3),
    "4": build_player_token(theme, 4),
    "5": build_player_token(theme, 5),
    "6": build_player_token(theme, 6),
    "7": build_player_token(theme, 7),
})
