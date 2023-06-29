import { ColorScheme } from "../theme/color_scheme"
import { background, border } from "./components"

export default function contacts_popover(theme: ColorScheme): any {
    return {
        background: background(theme.middle),
        corner_radius: 6,
        padding: { top: 6, bottom: 6 },
        shadow: theme.popover_shadow,
        border: border(theme.middle),
        width: 300,
        height: 400,
    }
}
