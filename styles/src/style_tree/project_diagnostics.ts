import { ColorScheme } from "../theme/color_scheme"
import { background, text } from "./components"

export default function project_diagnostics(colorScheme: ColorScheme): any {
    const layer = colorScheme.highest
    return {
        background: background(layer),
        tabIconSpacing: 4,
        tab_icon_width: 13,
        tabSummarySpacing: 10,
        emptyMessage: text(layer, "sans", "variant", { size: "md" }),
    }
}
