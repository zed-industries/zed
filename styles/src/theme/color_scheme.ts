import { Scale, Color } from "chroma-js"
import { Syntax, ThemeSyntax, SyntaxHighlightStyle } from "./syntax"
export { Syntax, ThemeSyntax, SyntaxHighlightStyle }
import {
    ThemeConfig,
    ThemeAppearance,
    ThemeConfigInputColors,
} from "./theme_config"
import { get_ramps } from "./ramps"

export interface ColorScheme {
    name: string
    is_light: boolean

    lowest: Layer
    middle: Layer
    highest: Layer

    ramps: RampSet

    popover_shadow: Shadow
    modal_shadow: Shadow

    players: Players
    syntax?: Partial<ThemeSyntax>
}

export interface Meta {
    name: string
    author: string
    url: string
    license: License
}

export interface License {
    SPDX: SPDXExpression
}

// License name -> License text
export interface Licenses {
    [key: string]: string
}

// FIXME: Add support for the SPDX expression syntax
export type SPDXExpression = "MIT"

export interface Player {
    cursor: string
    selection: string
}

export interface Players {
    "0": Player
    "1": Player
    "2": Player
    "3": Player
    "4": Player
    "5": Player
    "6": Player
    "7": Player
}

export interface Shadow {
    blur: number
    color: string
    offset: number[]
}

export type StyleSets = keyof Layer
export interface Layer {
    base: StyleSet
    variant: StyleSet
    on: StyleSet
    accent: StyleSet
    positive: StyleSet
    warning: StyleSet
    negative: StyleSet
}

export interface RampSet {
    neutral: Scale
    red: Scale
    orange: Scale
    yellow: Scale
    green: Scale
    cyan: Scale
    blue: Scale
    violet: Scale
    magenta: Scale
}

export type Styles = keyof StyleSet
export interface StyleSet {
    default: Style
    active: Style
    disabled: Style
    hovered: Style
    pressed: Style
    inverted: Style
}

export interface Style {
    background: string
    border: string
    foreground: string
}

export function create_color_scheme(theme: ThemeConfig): ColorScheme {
    const {
        name,
        appearance,
        input_color,
        override: { syntax },
    } = theme

    const is_light = appearance === ThemeAppearance.Light
    const color_ramps: ThemeConfigInputColors = input_color

    // Chromajs scales from 0 to 1 flipped if is_light is true
    const ramps = get_ramps(is_light, color_ramps)
    const lowest = lowest_layer(ramps)
    const middle = middle_layer(ramps)
    const highest = highest_layer(ramps)

    const popover_shadow = {
        blur: 4,
        color: ramps
            .neutral(is_light ? 7 : 0)
            .darken()
            .alpha(0.2)
            .hex(), // TODO used blend previously. Replace with something else
        offset: [1, 2],
    }

    const modal_shadow = {
        blur: 16,
        color: ramps
            .neutral(is_light ? 7 : 0)
            .darken()
            .alpha(0.2)
            .hex(), // TODO used blend previously. Replace with something else
        offset: [0, 2],
    }

    const players = {
        "0": player(ramps.blue),
        "1": player(ramps.green),
        "2": player(ramps.magenta),
        "3": player(ramps.orange),
        "4": player(ramps.violet),
        "5": player(ramps.cyan),
        "6": player(ramps.red),
        "7": player(ramps.yellow),
    }

    return {
        name,
        is_light,

        ramps,

        lowest,
        middle,
        highest,

        popover_shadow,
        modal_shadow,

        players,
        syntax,
    }
}

function player(ramp: Scale): Player {
    return {
        selection: ramp(0.5).alpha(0.24).hex(),
        cursor: ramp(0.5).hex(),
    }
}

function lowest_layer(ramps: RampSet): Layer {
    return {
        base: build_style_set(ramps.neutral, 0.2, 1),
        variant: build_style_set(ramps.neutral, 0.2, 0.7),
        on: build_style_set(ramps.neutral, 0.1, 1),
        accent: build_style_set(ramps.blue, 0.1, 0.5),
        positive: build_style_set(ramps.green, 0.1, 0.5),
        warning: build_style_set(ramps.yellow, 0.1, 0.5),
        negative: build_style_set(ramps.red, 0.1, 0.5),
    }
}

function middle_layer(ramps: RampSet): Layer {
    return {
        base: build_style_set(ramps.neutral, 0.1, 1),
        variant: build_style_set(ramps.neutral, 0.1, 0.7),
        on: build_style_set(ramps.neutral, 0, 1),
        accent: build_style_set(ramps.blue, 0.1, 0.5),
        positive: build_style_set(ramps.green, 0.1, 0.5),
        warning: build_style_set(ramps.yellow, 0.1, 0.5),
        negative: build_style_set(ramps.red, 0.1, 0.5),
    }
}

function highest_layer(ramps: RampSet): Layer {
    return {
        base: build_style_set(ramps.neutral, 0, 1),
        variant: build_style_set(ramps.neutral, 0, 0.7),
        on: build_style_set(ramps.neutral, 0.1, 1),
        accent: build_style_set(ramps.blue, 0.1, 0.5),
        positive: build_style_set(ramps.green, 0.1, 0.5),
        warning: build_style_set(ramps.yellow, 0.1, 0.5),
        negative: build_style_set(ramps.red, 0.1, 0.5),
    }
}

function build_style_set(
    ramp: Scale,
    background_base: number,
    foreground_base: number,
    step = 0.08
): StyleSet {
    const style_definitions = build_style_definition(
        background_base,
        foreground_base,
        step
    )

    function color_string(index_or_color: number | Color): string {
        if (typeof index_or_color === "number") {
            return ramp(index_or_color).hex()
        } else {
            return index_or_color.hex()
        }
    }

    function build_style(style: Styles): Style {
        return {
            background: color_string(style_definitions.background[style]),
            border: color_string(style_definitions.border[style]),
            foreground: color_string(style_definitions.foreground[style]),
        }
    }

    return {
        default: build_style("default"),
        hovered: build_style("hovered"),
        pressed: build_style("pressed"),
        active: build_style("active"),
        disabled: build_style("disabled"),
        inverted: build_style("inverted"),
    }
}

function build_style_definition(bg_base: number, fg_base: number, step = 0.08) {
    return {
        background: {
            default: bg_base,
            hovered: bg_base + step,
            pressed: bg_base + step * 1.5,
            active: bg_base + step * 2.2,
            disabled: bg_base,
            inverted: fg_base + step * 6,
        },
        border: {
            default: bg_base + step * 1,
            hovered: bg_base + step,
            pressed: bg_base + step,
            active: bg_base + step * 3,
            disabled: bg_base + step * 0.5,
            inverted: bg_base - step * 3,
        },
        foreground: {
            default: fg_base,
            hovered: fg_base,
            pressed: fg_base,
            active: fg_base + step * 6,
            disabled: bg_base + step * 4,
            inverted: bg_base + step * 2,
        },
    }
}
