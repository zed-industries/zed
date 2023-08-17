import { Layer } from "../common"
import { interactive, toggleable } from "../element"
import { Border, text } from "../style_tree/components"

type TabProps = {
    layer: Layer
}

export const tab = ({ layer }: TabProps) => {
    const active_color = text(layer, "sans", "base").color
    const inactive_border: Border = {
        color: '#FFFFFF00',
        width: 1,
        bottom: true,
        left: false,
        right: false,
        top: false,
    }
    const active_border: Border = {
        ...inactive_border,
        color: active_color,
    }

    const base = {
        ...text(layer, "sans", "variant"),
        padding: {
            top: 8,
            left: 8,
            right: 8,
            bottom: 6
        },
        border: inactive_border,
    }

    const i = interactive({
        state: {
            default: {
                ...base
            },
            hovered: {
                ...base,
                ...text(layer, "sans", "base", "hovered")
            },
            clicked: {
                ...base,
                ...text(layer, "sans", "base", "pressed")
            },
        }
    })

    return toggleable({
        base: i,
        state: {
            active: {
                default: {
                    ...i,
                    ...text(layer, "sans", "base"),
                    border: active_border,
                },
                hovered: {
                    ...i,
                    ...text(layer, "sans", "base", "hovered"),
                    border: active_border
                },
                clicked: {
                    ...i,
                    ...text(layer, "sans", "base", "pressed"),
                    border: active_border
                },
            }
        }
    })
}
