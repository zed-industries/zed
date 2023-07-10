import { useTheme } from "../theme"
import { background, border } from "./components"

export default function contacts_popover(): any {
    const theme = useTheme()

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
