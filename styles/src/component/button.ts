import { font_sizes, useTheme } from "../common"
import { Layer, Theme } from "../theme"
import { TextStyle, background } from "../style_tree/components"

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace Button {
    export type Options = {
        layer: Layer
        background: keyof Theme["lowest"]
        color: keyof Theme["lowest"]
        variant: Button.Variant
        size: Button.Size
        shape: Button.Shape
        margin: {
            top?: number
            bottom?: number
            left?: number
            right?: number
        }
        states: {
            enabled?: boolean
            hovered?: boolean
            pressed?: boolean
            focused?: boolean
            disabled?: boolean
        }
    }

    export type ToggleableOptions = Options & {
        active_background: keyof Theme["lowest"]
        active_color: keyof Theme["lowest"]
    }

    /** Padding added to each side of a Shape.Rectangle button */
    export const RECTANGLE_PADDING = 2
    export const FONT_SIZE = font_sizes.sm
    export const ICON_SIZE = 14
    export const CORNER_RADIUS = 6

    export const variant = {
        Default: "filled",
        Outline: "outline",
        Ghost: "ghost",
    } as const

    export type Variant = (typeof variant)[keyof typeof variant]

    export const shape = {
        Rectangle: "rectangle",
        Square: "square",
    } as const

    export type Shape = (typeof shape)[keyof typeof shape]

    export const size = {
        Small: "sm",
        Medium: "md",
    } as const

    export type Size = (typeof size)[keyof typeof size]

    export type BaseStyle = {
        corder_radius: number
        background: string | null
        padding: {
            top: number
            bottom: number
            left: number
            right: number
        }
        margin: Button.Options["margin"]
        button_height: number
    }

    export type LabelButtonStyle = BaseStyle & TextStyle
    // export type IconButtonStyle = ButtonStyle

    export const button_base = (
        options: Partial<Button.Options> = {
            variant: Button.variant.Default,
            shape: Button.shape.Rectangle,
            states: {
                hovered: true,
                pressed: true,
            },
        }
    ): BaseStyle => {
        const theme = useTheme()

        const layer = options.layer ?? theme.middle
        const color = options.color ?? "base"
        const background_color =
            options.variant === Button.variant.Ghost
                ? null
                : background(layer, options.background ?? color)

        const m = {
            top: options.margin?.top ?? 0,
            bottom: options.margin?.bottom ?? 0,
            left: options.margin?.left ?? 0,
            right: options.margin?.right ?? 0,
        }
        const size = options.size || Button.size.Medium
        const padding = 2

        const base: BaseStyle = {
            background: background_color,
            corder_radius: Button.CORNER_RADIUS,
            padding: {
                top: padding,
                bottom: padding,
                left:
                    options.shape === Button.shape.Rectangle
                        ? padding + Button.RECTANGLE_PADDING
                        : padding,
                right:
                    options.shape === Button.shape.Rectangle
                        ? padding + Button.RECTANGLE_PADDING
                        : padding,
            },
            margin: m,
            button_height: 16,
        }

        return base
    }
}
