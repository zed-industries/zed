import { useSurfaceStyle } from "@components/surface"
import { Theme } from "@theme*"

export default function sharedScreen(theme: Theme) {
    // TODO: This should be "editor"
    // Update when an approach for editor surface is available
    const surfaceStyle = useSurfaceStyle(theme, "pane")
    return {
        background: surfaceStyle.background,
    }
}
