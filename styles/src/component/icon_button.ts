import { interactive, toggleable } from "../element"
import { background, foreground } from "../style_tree/components"
import { useTheme, Theme } from "../theme"
import { ButtonVariant, Variant } from "./button"

export type Margin = {
    top: number
    bottom: number
    left: number
    right: number
}

interface IconButtonOptions {
    layer?:
    | Theme["lowest"]
    | Theme["middle"]
    | Theme["highest"]
    color?: keyof Theme["lowest"]
    margin?: Partial<Margin>
    variant?: Variant
}

type ToggleableIconButtonOptions = IconButtonOptions & {
    active_color?: keyof Theme["lowest"]
}

export function icon_button({ color, margin, layer, variant = ButtonVariant.Default }: IconButtonOptions) {
    const theme = useTheme()

    if (!color) color = "base"

    const background_color = variant === ButtonVariant.Ghost ? null : background(layer ?? theme.lowest, color)

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
                background: background_color,
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
    theme: Theme,
    { color, active_color, margin, variant }: ToggleableIconButtonOptions
) {
    if (!color) color = "base"

    return toggleable({
        state: {
            inactive: icon_button({ color, margin, variant }),
            active: icon_button({
                color: active_color ? active_color : color,
                margin,
                layer: theme.middle,
            }),
        },
    })
}
