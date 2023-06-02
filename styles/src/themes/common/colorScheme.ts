import { Scale, Color } from "chroma-js"
import { Syntax, ThemeSyntax, SyntaxHighlightStyle } from "./syntax"
export { Syntax, ThemeSyntax, SyntaxHighlightStyle }
import {
    ThemeConfig,
    ThemeAppearance,
    ThemeConfigInputColors,
} from "../../themeConfig"
import { getRamps } from "./ramps"

export interface ColorScheme {
    name: string
    isLight: boolean

    lowest: Layer
    middle: Layer
    highest: Layer

    ramps: RampSet

    popoverShadow: Shadow
    modalShadow: Shadow

    players: Players
    syntax?: Partial<ThemeSyntax>
}

export interface MetaAndLicense {
    meta: Meta
    licenseFile: string
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

export function createColorScheme(theme: ThemeConfig): ColorScheme {
    const {
        name,
        appearance,
        inputColor,
        override: { syntax },
    } = theme

    const isLight = appearance === ThemeAppearance.Light
    const colorRamps: ThemeConfigInputColors = inputColor

    // Chromajs scales from 0 to 1 flipped if isLight is true
    const ramps = getRamps(isLight, colorRamps)
    const lowest = lowestLayer(ramps)
    const middle = middleLayer(ramps)
    const highest = highestLayer(ramps)

    const popoverShadow = {
        blur: 4,
        color: ramps
            .neutral(isLight ? 7 : 0)
            .darken()
            .alpha(0.2)
            .hex(), // TODO used blend previously. Replace with something else
        offset: [1, 2],
    }

    const modalShadow = {
        blur: 16,
        color: ramps
            .neutral(isLight ? 7 : 0)
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
        isLight,

        ramps,

        lowest,
        middle,
        highest,

        popoverShadow,
        modalShadow,

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

function lowestLayer(ramps: RampSet): Layer {
    return {
        base: buildStyleSet(ramps.neutral, 0.2, 1),
        variant: buildStyleSet(ramps.neutral, 0.2, 0.7),
        on: buildStyleSet(ramps.neutral, 0.1, 1),
        accent: buildStyleSet(ramps.blue, 0.1, 0.5),
        positive: buildStyleSet(ramps.green, 0.1, 0.5),
        warning: buildStyleSet(ramps.yellow, 0.1, 0.5),
        negative: buildStyleSet(ramps.red, 0.1, 0.5),
    }
}

function middleLayer(ramps: RampSet): Layer {
    return {
        base: buildStyleSet(ramps.neutral, 0.1, 1),
        variant: buildStyleSet(ramps.neutral, 0.1, 0.7),
        on: buildStyleSet(ramps.neutral, 0, 1),
        accent: buildStyleSet(ramps.blue, 0.1, 0.5),
        positive: buildStyleSet(ramps.green, 0.1, 0.5),
        warning: buildStyleSet(ramps.yellow, 0.1, 0.5),
        negative: buildStyleSet(ramps.red, 0.1, 0.5),
    }
}

function highestLayer(ramps: RampSet): Layer {
    return {
        base: buildStyleSet(ramps.neutral, 0, 1),
        variant: buildStyleSet(ramps.neutral, 0, 0.7),
        on: buildStyleSet(ramps.neutral, 0.1, 1),
        accent: buildStyleSet(ramps.blue, 0.1, 0.5),
        positive: buildStyleSet(ramps.green, 0.1, 0.5),
        warning: buildStyleSet(ramps.yellow, 0.1, 0.5),
        negative: buildStyleSet(ramps.red, 0.1, 0.5),
    }
}

function buildStyleSet(
    ramp: Scale,
    backgroundBase: number,
    foregroundBase: number,
    step: number = 0.08
): StyleSet {
    let styleDefinitions = buildStyleDefinition(
        backgroundBase,
        foregroundBase,
        step
    )

    function colorString(indexOrColor: number | Color): string {
        if (typeof indexOrColor === "number") {
            return ramp(indexOrColor).hex()
        } else {
            return indexOrColor.hex()
        }
    }

    function buildStyle(style: Styles): Style {
        return {
            background: colorString(styleDefinitions.background[style]),
            border: colorString(styleDefinitions.border[style]),
            foreground: colorString(styleDefinitions.foreground[style]),
        }
    }

    return {
        default: buildStyle("default"),
        hovered: buildStyle("hovered"),
        pressed: buildStyle("pressed"),
        active: buildStyle("active"),
        disabled: buildStyle("disabled"),
        inverted: buildStyle("inverted"),
    }
}

function buildStyleDefinition(
    bgBase: number,
    fgBase: number,
    step: number = 0.08
) {
    return {
        background: {
            default: bgBase,
            hovered: bgBase + step,
            pressed: bgBase + step * 1.5,
            active: bgBase + step * 2.2,
            disabled: bgBase,
            inverted: fgBase + step * 6,
        },
        border: {
            default: bgBase + step * 1,
            hovered: bgBase + step,
            pressed: bgBase + step,
            active: bgBase + step * 3,
            disabled: bgBase + step * 0.5,
            inverted: bgBase - step * 3,
        },
        foreground: {
            default: fgBase,
            hovered: fgBase,
            pressed: fgBase,
            active: fgBase + step * 6,
            disabled: bgBase + step * 4,
            inverted: bgBase + step * 2,
        },
    }
}
