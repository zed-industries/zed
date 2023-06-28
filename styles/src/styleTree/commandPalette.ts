import { ColorScheme } from "../theme/colorScheme"
import { withOpacity } from "../theme/color"
import { text, background } from "./components"
import { toggleable } from "../element"

export default function commandPalette(colorScheme: ColorScheme): any {
    const layer = colorScheme.highest

    const key = toggleable({
        base: {
            text: text(layer, "mono", "variant", "default", { size: "xs" }),
            cornerRadius: 2,
            background: background(layer, "on"),
            padding: {
                top: 1,
                bottom: 1,
                left: 6,
                right: 6,
            },
            margin: {
                top: 1,
                bottom: 1,
                left: 2,
            },
        },
        state: {
            active: {
                text: text(layer, "mono", "on", "default", { size: "xs" }),
                background: withOpacity(background(layer, "on"), 0.2),
            },
        },
    })

    return {
        keystrokeSpacing: 8,
        // TODO: This should be a Toggle<ContainedText> on the rust side so we don't have to do this
        key: {
            inactive: { ...key.inactive },
            active: key.active,
        },
    }
}
