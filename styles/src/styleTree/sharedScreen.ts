import { ColorScheme } from "../theme/colorScheme"
import { StyleTree } from "../types"
import { background } from "./components"

export default function sharedScreen(
    colorScheme: ColorScheme
): StyleTree.SharedScreen {
    const layer = colorScheme.highest
    return {
        background: background(layer),
    }
}
