import { useColors } from "./colors"
import { Theme } from "./config"
import { ContainedText, InteractiveContainer, buildStates, container } from "./container"
import { ElementIntensities, Intensity, resolveThemeColorIntensity, useElementIntensities } from "./intensity"

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
}

const DEFAULT_TEXT_OPTIONS: Partial<TextStyle> = {
    family: family.sans,
    size: size.md,
    weight: weight.regular,
    lineHeight: 1,
}

const DEFAULT_TEXT_INTENSITY: Intensity = 100

export function text(
    theme: Theme,
    intensity?: Intensity,
    options?: Partial<TextStyle>,
): TextStyle {

    if (!intensity) {
        intensity = DEFAULT_TEXT_INTENSITY
    }

    const themeColor = useColors(theme)
    const resolvedColorIntensity = resolveThemeColorIntensity(theme, intensity)
    const DEFAULT_COLOR = themeColor.neutral(resolvedColorIntensity)

    const color = options.color ? themeColor[options.color](intensity) : DEFAULT_COLOR

    const mergedOptions = {
        ...DEFAULT_TEXT_OPTIONS,
        ...options,
    }

    const text: TextStyle = {
        family: mergedOptions.family,
        size: mergedOptions.size,
        weight: mergedOptions.weight,
        color,
        lineHeight: mergedOptions.lineHeight,
    }

    return text
}

export function interactiveText(
    theme: Theme,
    options?: Partial<TextStyle>,
): InteractiveContainer<ContainedText> {

    const DEFAULT_INTENSITIES: ElementIntensities = {
        bg: DEFAULT_TEXT_INTENSITY,
        border: DEFAULT_TEXT_INTENSITY,
        fg: DEFAULT_TEXT_INTENSITY,
    }

    const fg = resolveThemeColorIntensity(theme, DEFAULT_INTENSITIES.fg);
    const { color = 'neutral', ...mergedOptions } = options;
    const { neutral, ...themeColor } = useColors(theme);
    const states = buildStates(theme, DEFAULT_INTENSITIES);

    const common = {
        container: container.blank,
    };

    const createText = (fgColor: Intensity): ContainedText => ({
        ...common,
        text: text(theme, fgColor, { ...mergedOptions, color: themeColor[color](fg) }),
    });

    return {
        default: createText(states.default.fg),
        hovered: createText(states.hovered.fg),
        pressed: createText(states.pressed.fg),
    };
}
