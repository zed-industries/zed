import { useTheme } from "../theme"
import { background, text } from "./components"

export default function project_diagnostics(): any {
    const theme = useTheme()

    return {
        background: background(theme.highest),
        tab_icon_spacing: 4,
        tab_icon_width: 13,
        tab_summary_spacing: 10,
        empty_message: text(theme.highest, "sans", "variant", { size: "md" }),
    }
}
