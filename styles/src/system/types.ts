import { Curve } from "./ref/curves"

export interface ColorAccessibilityValue {
    value: number
    aaPass: boolean
    aaaPass: boolean
}

/**
 * Calculates the color contrast between a specified color and its corresponding background and foreground colors.
 *
 * @note This implementation is currently basic – Currently we only calculate contrasts against black and white, in the future will allow for dynamic color contrast calculation based on the colors present in a given palette.
 * @note The goal is to align with WCAG3 accessibility standards as they become stabilized. See the [WCAG 3 Introduction](https://www.w3.org/WAI/standards-guidelines/wcag/wcag3-intro/) for more information.
 */
export interface ColorAccessibility {
    black: ColorAccessibilityValue
    white: ColorAccessibilityValue
}

export type Color = {
    step: number
    contrast: ColorAccessibility
    hex: string
    lch: number[]
    rgba: number[]
    isLight: boolean
}

export interface ColorScale {
    colors: Color[]
    // An array of hex values for each color in the scale
    values: string[]
}

export type ColorFamily = {
    name: string
    scale: ColorScale
    invertedScale: ColorScale
}

export interface ColorFamilyHue {
    start: number
    end: number
    curve: Curve
}

export interface ColorFamilySaturation {
    start: number
    end: number
    curve: Curve
}

export interface ColorFamilyLightness {
    start: number
    end: number
    curve: Curve
}

export interface ColorFamilyConfig {
    name: string
    color: {
        hue: ColorFamilyHue
        saturation: ColorFamilySaturation
        lightness: ColorFamilyLightness
    }
}
