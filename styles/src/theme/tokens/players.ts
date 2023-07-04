import { SingleColorToken } from "@tokens-studio/types"
import { color_token } from "./token"
import { Players } from "../color_scheme"
import { useTheme } from "@/src/common"

export type PlayerToken = Record<"selection" | "cursor", SingleColorToken>

export type PlayersToken = Record<keyof Players, PlayerToken>

function build_player_token(index: number): PlayerToken {
    const theme = useTheme()
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

export const players_token = (): PlayersToken => {
    return {
        "0": build_player_token(0),
        "1": build_player_token(1),
        "2": build_player_token(2),
        "3": build_player_token(3),
        "4": build_player_token(4),
        "5": build_player_token(5),
        "6": build_player_token(6),
        "7": build_player_token(7),
    }
}
