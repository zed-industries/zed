import { ColorScheme } from "../theme/colorScheme"
import { background } from "./components"

export default function sharedScreen(colorScheme: ColorScheme) {
    let layer = colorScheme.highest
    return {
        background: background(layer),
    }
}
