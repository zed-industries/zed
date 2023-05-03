import { buildSurfaces } from "@components/surface"
import { Theme } from "@theme/config"
import { ContainedText, container } from "@theme/container"
import { margin, padding } from "@theme/properties"
import { textStyle } from "@theme/text"

export default function tooltip(theme: Theme) {
    const surface = buildSurfaces(theme)

    const tooltipStyle: ContainedText = {
        container: {
            ...container.blank,
            ...surface.popover,
            padding: padding(8, 4),
            margin: margin(6),
        },
        text: textStyle(theme)
    }

    return {
        // background: background(layer),
        // border: border(layer),
        padding: { top: 4, bottom: 4, left: 8, right: 8 },
        margin: { top: 6, left: 6 },
        shadow: colorScheme.popoverShadow,
        cornerRadius: 6,
        text: textStyle(layer, "sans", { size: "xs" }),
        keystroke: {
            background: background(layer, "on"),
            cornerRadius: 4,
            margin: { left: 6 },
            padding: { left: 4, right: 4 },
            ...textStyle(layer, "mono", "on", { size: "xs", weight: "bold" }),
        },
        maxTextWidth: 200,
    }
}
