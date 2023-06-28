import { ColorScheme } from "../common"
import { interactive, toggleable } from "../element"
import { background, foreground } from "../styleTree/components"

export type Margin = {
    top: number
    bottom: number
    left: number
    right: number
}

interface IconButtonOptions {
    layer?:
        | ColorScheme["lowest"]
        | ColorScheme["middle"]
        | ColorScheme["highest"]
    color?: keyof ColorScheme["lowest"]
    margin?: Partial<Margin>
}

type ToggleableIconButtonOptions = IconButtonOptions & {
    active_color?: keyof ColorScheme["lowest"]
}

export function icon_button(
    theme: ColorScheme,
    { color, margin, layer }: IconButtonOptions
) {
    if (!color) color = "base"

    const m = {
        top: margin?.top ?? 0,
        bottom: margin?.bottom ?? 0,
        left: margin?.left ?? 0,
        right: margin?.right ?? 0,
    }

    return interactive({
        base: {
            corner_radius: 6,
            padding: {
                top: 2,
                bottom: 2,
                left: 4,
                right: 4,
            },
            margin: m,
            icon_width: 14,
            icon_height: 14,
            button_width: 20,
            button_height: 16,
        },
        state: {
            default: {
                background: background(layer ?? theme.lowest, color),
                color: foreground(layer ?? theme.lowest, color),
            },
            hovered: {
                background: background(layer ?? theme.lowest, color, "hovered"),
                color: foreground(layer ?? theme.lowest, color, "hovered"),
            },
            clicked: {
                background: background(layer ?? theme.lowest, color, "pressed"),
                color: foreground(layer ?? theme.lowest, color, "pressed"),
            },
        },
    })
}

export function toggleable_icon_button(
    theme: ColorScheme,
    { color, active_color, margin }: ToggleableIconButtonOptions
) {
    if (!color) color = "base"

    return toggleable({
        state: {
            inactive: icon_button(theme, { color, margin }),
            active: icon_button(theme, {
                color: active_color ? active_color : color,
                margin,
                layer: theme.middle,
            }),
        },
    })
}
