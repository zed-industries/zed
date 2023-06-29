import { ColorScheme } from "../theme/color_scheme"
import { background } from "./components"

export default function sharedScreen(theme: ColorScheme) {
    return {
        background: background(theme.highest),
    }
}
