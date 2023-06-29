import { ColorScheme } from "../theme/color_scheme"
import { background } from "./components"

export default function sharedScreen(colorScheme: ColorScheme) {
    const layer = colorScheme.highest
    return {
        background: background(layer),
    }
}
