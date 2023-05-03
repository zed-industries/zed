import chroma from "chroma-js"
import { useColors } from "./colors"
import { Theme, ThemeColor } from "./config"
import {
    ContainedText,
    Interactive,
    buildIntensitiesForStates,
    container,
} from "./container"
import {
    ElementIntensities,
    Intensity,
    resolveThemeColorIntensity,
} from "./intensity"

type Font = "Zed Mono" | "Zed Sans"

export interface Families {
    mono: Font
    sans: Font
    ui: Font
    terminal: Font
}

export const family: Families = {
    mono: "Zed Mono",
    sans: "Zed Sans",
    ui: "Zed Sans",
    terminal: "Zed Mono",
}

export interface Sizes {
    xs: number
    sm: number
    md: number
    lg: number
    xl: number
}

export const size: Sizes = {
    xs: 0.75,
    sm: 0.875,
    md: 1,
    lg: 1.125,
    xl: 1.25,
}

export type Weight = 400 | 700

export interface Weights {
    regular: Weight
    bold: Weight
}

export const weight: Weights = {
    regular: 400,
    bold: 700,
}

export interface Features {
    /** Contextual Alternates: Applies a second substitution feature based on a match of a character pattern within a context of surrounding patterns */
    calt?: boolean
    /** Case-Sensitive Forms: Shifts various punctuation marks up to a position that works better with all-capital sequences */
    case?: boolean
    /** Capital Spacing: Adjusts inter-glyph spacing for all-capital text */
    cpsp?: boolean
    /** Fractions: Replaces figures separated by a slash with diagonal fractions */
    frac?: boolean
    /** Standard Ligatures: Replaces a sequence of glyphs with a single glyph which is preferred for typographic purposes */
    liga?: boolean
    /** Oldstyle Figures: Changes selected figures from the default or lining style to oldstyle form. */
    onum?: boolean
    /** Ordinals: Replaces default alphabetic glyphs with the corresponding ordinal forms for use after figures */
    ordn?: boolean
    /** Proportional Figures: Replaces figure glyphs set on uniform (tabular) widths with corresponding glyphs set on proportional widths */
    pnum?: boolean
    /** Subscript: Replaces default glyphs with subscript glyphs */
    subs?: boolean
    /** Superscript: Replaces default glyphs with superscript glyphs */
    sups?: boolean
    /** Swash: Replaces default glyphs with swash glyphs for stylistic purposes */
    swsh?: boolean
    /** Titling: Replaces default glyphs with titling glyphs for use in large-size settings */
    titl?: boolean
    /** Tabular Figures: Replaces figure glyphs set on proportional widths with corresponding glyphs set on uniform (tabular) widths */
    tnum?: boolean
    /** Slashed Zero: Replaces default zero with a slashed zero for better distinction between "0" and "O" */
    zero?: boolean
    /** Stylistic sets 01 - 20 */
    ss01?: boolean
    ss02?: boolean
    ss03?: boolean
    ss04?: boolean
    ss05?: boolean
    ss06?: boolean
    ss07?: boolean
    ss08?: boolean
    ss09?: boolean
    ss10?: boolean
    ss11?: boolean
    ss12?: boolean
    ss13?: boolean
    ss14?: boolean
    ss15?: boolean
    ss16?: boolean
    ss17?: boolean
    ss18?: boolean
    ss19?: boolean
    ss20?: boolean
}

export interface TextStyle {
    family: Font
    size: number
    weight: Weight
    color: string
    lineHeight: number
    underline?: boolean
}

/** Text options. Will be merged with DEFAULT_TEXT_OPTIONS */
interface BuildTextOptions extends Partial<Omit<TextStyle, "color">> {
    // The number relative font sizes are multiplied by to get the actual font size
    baseSize: number
    intensity: Intensity
    /** A color family from the theme */
    color: ThemeColor
}

const DEFAULT_BASE_TEXT_SIZE = 13 as const

const DEFAULT_TEXT_OPTIONS: BuildTextOptions = {
    family: family.sans,
    baseSize: DEFAULT_BASE_TEXT_SIZE,
    size: size.md,
    weight: weight.regular,
    color: "neutral",
    intensity: 100,
    lineHeight: 1,
}

function buildText(
    theme: Theme,
    options?: Partial<BuildTextOptions>
): TextStyle {
    const themeColor = useColors(theme)
    const defaultOptions = DEFAULT_TEXT_OPTIONS

    const mergedOptions = {
        ...defaultOptions,
        ...options,
    }

    const {
        family,
        weight,
        baseSize,
        lineHeight,
        color: colorScale,
        intensity,
    } = mergedOptions

    const resolvedIntensity = resolveThemeColorIntensity(theme, intensity)
    const color = themeColor[colorScale](resolvedIntensity)

    // Calculate the final font size
    const size = mergedOptions.size * baseSize

    // Ensture the color is valid
    chroma.valid(color)

    const text: TextStyle = {
        family,
        weight,
        size,
        lineHeight,
        color,
    }

    return text
}

export function textStyle(
    theme: Theme,
    options?: Partial<BuildTextOptions>
): TextStyle {
    return buildText(theme, options)
}

export function useInteractiveText(
    theme: Theme,
    options?: Partial<BuildTextOptions>
): Interactive<ContainedText> {
    const DEFAULT_INTENSITIES: ElementIntensities = {
        bg: 1,
        border: 15,
        fg: 100,
    } as const

    const states = buildIntensitiesForStates(
        theme,
        "interactiveText",
        DEFAULT_INTENSITIES
    )

    const text = {
        default: buildText(theme, {
            ...options,
            intensity: states.default.fg,
        }),
        hovered: buildText(theme, {
            ...options,
            intensity: states.hovered.fg,
        }),
        pressed: buildText(theme, {
            ...options,
            intensity: states.pressed.fg,
        }),
    }

    const buildContainedText = (text: TextStyle) => {
        return {
            container: container.blank,
            text,
        }
    }

    return {
        default: buildContainedText(text.default),
        hovered: buildContainedText(text.hovered),
        pressed: buildContainedText(text.pressed),
    }
}
