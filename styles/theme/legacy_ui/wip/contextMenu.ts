import { border } from "@theme/border"
import { Theme } from "@theme/config"
import { weight } from "@theme/text"

export default function contextMenu(theme: Theme) {
    const container = {}

    const legacy_properties = {
        keystrokeMargin: 30,
    }

    // container()
    // flex() // container with flex
    // interactiveFlex() // interactive<container> with flex
    // shadow() or add shdow to container

    const containerStyle = container(theme,
        {
            background: background(theme, "popover"),
            borderRadius: 10,
            padding: 4,
            border: border(theme, "popover"),
            shadow: shadow(theme),
        })

    const item = interactiveFlex(theme, {})

    const keystroke = label(theme, {
        weight: weight.bold,
    })

    return {
        ...legacy_properties,
        ...containerStyle,
        item: {
            iconSpacing: 8,
            iconWidth: 14,
            padding: { left: 6, right: 6, top: 2, bottom: 2 },
            cornerRadius: 6,
            label: text(layer, "sans", { size: "sm" }),
            keystroke: {
                ...text(layer, "sans", "variant", {
                    size: "sm",
                    weight: "bold",
                }),
                padding: { left: 3, right: 3 },
            },
            hover: {
                background: background(layer, "hovered"),
                label: text(layer, "sans", "hovered", { size: "sm" }),
                keystroke: {
                    ...text(layer, "sans", "hovered", {
                        size: "sm",
                        weight: "bold",
                    }),
                    padding: { left: 3, right: 3 },
                },
            },
            active: {
                background: background(layer, "active"),
            },
            activeHover: {
                background: background(layer, "active"),
            },
        },
        separator: {
            background: borderColor(layer),
            margin: { top: 2, bottom: 2 },
        },
    }
}
