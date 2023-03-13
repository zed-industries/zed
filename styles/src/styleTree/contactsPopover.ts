import { ColorScheme } from "../themes/common/colorScheme"
import { background, border, text } from "./components"

export default function contactsPopover(colorScheme: ColorScheme) {
    let layer = colorScheme.middle
    const sidePadding = 12
    return {
        background: background(layer),
        cornerRadius: 6,
        padding: { top: 6, bottom: 6 },
        margin: { top: -6 },
        shadow: colorScheme.popoverShadow,
        border: border(layer),
        width: 300,
        height: 400,
    }
}
