import { Interactive, interactive, toggleable, Toggleable } from "../element"
import { TextStyle, background, text } from "../style_tree/components"
import { useTheme } from "../theme"
import { Button } from "./button"

type LabelButtonStyle = {
    corder_radius: number
    background: string | null
    padding: {
        top: number
        bottom: number
        left: number
        right: number
    },
    margin: Button.Options['margin']
    button_height: number
} & TextStyle

/** Styles an Interactive&lt;ContainedText> */
export function label_button_style(
    options: Partial<Button.Options> = {
        variant: Button.variant.Default,
        shape: Button.shape.Rectangle,
        states: {
            hovered: true,
            pressed: true
        }
    }
): Interactive<LabelButtonStyle> {
    const theme = useTheme()

    const base = Button.button_base(options)
    const layer = options.layer ?? theme.middle
    const color = options.color ?? "base"

    const default_state = {
        ...base,
        ...text(layer ?? theme.lowest, "sans", color),
        font_size: Button.FONT_SIZE,
    }

    return interactive({
        base: default_state,
        state: {
            hovered: {
                background: background(layer, options.background ?? color, "hovered")
            },
            clicked: {
                background: background(layer, options.background ?? color, "pressed")
            }
        }
    })
}

/** Styles an Toggleable&lt;Interactive&lt;ContainedText>> */
export function toggle_label_button_style(
    options: Partial<Button.ToggleableOptions> = {
        variant: Button.variant.Default,
        shape: Button.shape.Rectangle,
        states: {
            hovered: true,
            pressed: true
        }
    }
): Toggleable<Interactive<LabelButtonStyle>> {
    const activeOptions = {
        ...options,
        color: options.active_color || options.color,
        background: options.active_background || options.background
    }

    return toggleable({
        state: {
            inactive: label_button_style(options),
            active: label_button_style(activeOptions),
        },
    })
}
