import { ColorScheme } from "../theme/color_scheme"
import { background, border } from "./components"

export default function contacts_popover(colorScheme: ColorScheme): any {
    const layer = colorScheme.middle
    return {
        background: background(layer),
        cornerRadius: 6,
        padding: { top: 6, bottom: 6 },
        shadow: colorScheme.popoverShadow,
        border: border(layer),
        width: 300,
        height: 400,
    }
}
