import chroma from "chroma-js"
import { Meta, ThemeSyntax } from "../common/colorScheme"
import { colorRamp, createColorScheme } from "../common/ramps"

const name = "Gruvbox"

const color = {
    dark0_hard: "#1d2021",
    dark0: "#282828",
    dark0_soft: "#32302f",
    dark1: "#3c3836",
    dark2: "#504945",
    dark3: "#665c54",
    dark4: "#7c6f64",
    dark4_256: "#7c6f64",

    gray_245: "#928374",
    gray_244: "#928374",

    light0_hard: "#f9f5d7",
    light0: "#fbf1c7",
    light0_soft: "#f2e5bc",
    light1: "#ebdbb2",
    light2: "#d5c4a1",
    light3: "#bdae93",
    light4: "#a89984",
    light4_256: "#a89984",

    bright_red: "#fb4934",
    bright_green: "#b8bb26",
    bright_yellow: "#fabd2f",
    bright_blue: "#83a598",
    bright_purple: "#d3869b",
    bright_aqua: "#8ec07c",
    bright_orange: "#fe8019",

    neutral_red: "#cc241d",
    neutral_green: "#98971a",
    neutral_yellow: "#d79921",
    neutral_blue: "#458588",
    neutral_purple: "#b16286",
    neutral_aqua: "#689d6a",
    neutral_orange: "#d65d0e",

    faded_red: "#9d0006",
    faded_green: "#79740e",
    faded_yellow: "#b57614",
    faded_blue: "#076678",
    faded_purple: "#8f3f71",
    faded_aqua: "#427b58",
    faded_orange: "#af3a03",
}

interface ThemeColors {
    red: string
    green: string
    yellow: string
    blue: string
    purple: string
    aqua: string
    orange: string
    gray: string
}

const darkNeutrals = [
    color.dark1,
    color.dark2,
    color.dark3,
    color.dark4,
    color.light4,
    color.light3,
    color.light2,
    color.light1,
    color.light0,
]

const dark: ThemeColors = {
    red: color.bright_red,
    green: color.bright_green,
    yellow: color.bright_yellow,
    blue: color.bright_blue,
    purple: color.bright_purple,
    aqua: color.bright_aqua,
    orange: color.bright_orange,
    gray: color.light4,
}

const lightNeutrals = [
    color.light1,
    color.light2,
    color.light3,
    color.light4,
    color.dark4,
    color.dark3,
    color.dark2,
    color.dark1,
    color.dark0,
]

const light: ThemeColors = {
    red: color.faded_red,
    green: color.faded_green,
    yellow: color.faded_yellow,
    blue: color.faded_blue,
    purple: color.faded_purple,
    aqua: color.faded_aqua,
    orange: color.faded_orange,
    gray: color.dark4,
}

const darkHardNeutral = [color.dark0_hard, ...darkNeutrals]
const darkNeutral = [color.dark0, ...darkNeutrals]
const darkSoftNeutral = [color.dark0_soft, ...darkNeutrals]

const lightHardNeutral = [color.light0_hard, ...lightNeutrals]
const lightNeutral = [color.light0, ...lightNeutrals]
const lightSoftNeutral = [color.light0_soft, ...lightNeutrals]

interface Variant {
    name: string
    appearance: "light" | "dark"
    colors: ThemeColors
}

const variant: Variant[] = [
    {
        name: "Dark Hard",
        appearance: "dark",
        colors: dark,
    },
    {
        name: "Dark",
        appearance: "dark",
        colors: dark,
    },
    {
        name: "Dark Soft",
        appearance: "dark",
        colors: dark,
    },
    {
        name: "Light Hard",
        appearance: "light",
        colors: light,
    },
    {
        name: "Light",
        appearance: "light",

        colors: light,
    },
    {
        name: "Light Soft",
        appearance: "light",
        colors: light,
    },
]

const buildVariant = (variant: Variant) => {
    const { colors } = variant

    const name = `Gruvbox ${variant.name}`

    const isLight = variant.appearance === "light"

    let neutral: string[] = []

    switch (variant.name) {
        case "Dark Hard": {
            neutral = darkHardNeutral
            break
        }
        case "Dark": {
            neutral = darkNeutral
            break
        }
        case "Dark Soft": {
            neutral = darkSoftNeutral
            break
        }
        case "Light Hard": {
            neutral = lightHardNeutral
            break
        }
        case "Light": {
            neutral = lightNeutral
            break
        }
        case "Light Soft": {
            neutral = lightSoftNeutral
            break
        }
    }

    const ramps = {
        neutral: chroma.scale(isLight ? neutral.reverse() : neutral),
        red: colorRamp(chroma(variant.colors.red)),
        orange: colorRamp(chroma(variant.colors.orange)),
        yellow: colorRamp(chroma(variant.colors.yellow)),
        green: colorRamp(chroma(variant.colors.green)),
        cyan: colorRamp(chroma(variant.colors.aqua)),
        blue: colorRamp(chroma(variant.colors.blue)),
        violet: colorRamp(chroma(variant.colors.purple)),
        magenta: colorRamp(chroma(variant.colors.gray)),
    }

    const syntax: ThemeSyntax = {
        primary: { color: neutral[isLight ? 0 : 8] },
        "text.literal": { color: colors.blue },
        comment: { color: colors.gray },
        punctuation: { color: neutral[isLight ? 1 : 7] },
        "punctuation.bracket": { color: neutral[isLight ? 3 : 5] },
        "punctuation.list_marker": { color: neutral[isLight ? 0 : 8] },
        operator: { color: colors.aqua },
        boolean: { color: colors.purple },
        number: { color: colors.purple },
        string: { color: colors.green },
        "string.special": { color: colors.purple },
        "string.special.symbol": { color: colors.aqua },
        "string.regex": { color: colors.orange },
        type: { color: colors.yellow },
        enum: { color: colors.orange },
        tag: { color: colors.aqua },
        constant: { color: colors.yellow },
        keyword: { color: colors.red },
        function: { color: colors.green },
        "function.builtin": { color: colors.red },
        variable: { color: colors.blue },
        property: { color: neutral[isLight ? 0 : 8] },
        embedded: { color: colors.aqua },
        linkText: { color: colors.aqua },
        linkUri: { color: colors.purple },
        title: { color: colors.green },
    }

    return createColorScheme(name, isLight, ramps, syntax)
}

// Variants
export const darkHard = buildVariant(variant[0])
export const darkDefault = buildVariant(variant[1])
export const darkSoft = buildVariant(variant[2])
export const lightHard = buildVariant(variant[3])
export const lightDefault = buildVariant(variant[4])
export const lightSoft = buildVariant(variant[5])

export const meta: Meta = {
    name,
    license: {
        SPDX: "MIT", // "MIT/X11"
        license_text:
            "Copyright <YEAR> <COPYRIGHT HOLDER>\n\nPermission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files(the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/ or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.\n\nTHE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.",
    },
    author: "morhetz <morhetz@gmail.com>",
    url: "https://github.com/morhetz/gruvbox",
}
