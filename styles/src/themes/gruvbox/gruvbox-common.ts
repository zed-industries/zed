import {
    chroma,
    color_ramp,
    ThemeAppearance,
    ThemeLicenseType,
    ThemeConfig,
    ThemeFamilyMeta,
    ThemeConfigInputSyntax,
} from "../../common"

const meta: ThemeFamilyMeta = {
    name: "Gruvbox",
    author: "morhetz <morhetz@gmail.com>",
    license_type: ThemeLicenseType.MIT,
    license_url: "https://github.com/morhetz/gruvbox",
}

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

const dark_neutrals = [
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

const light_neutrals = [
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

const dark_hard_neutral = [color.dark0_hard, ...dark_neutrals]
const dark_neutral = [color.dark0, ...dark_neutrals]
const dark_soft_neutral = [color.dark0_soft, ...dark_neutrals]

const light_hard_neutral = [color.light0_hard, ...light_neutrals]
const light_neutral = [color.light0, ...light_neutrals]
const light_soft_neutral = [color.light0_soft, ...light_neutrals]

const build_variant = (variant: Variant): ThemeConfig => {
    const { colors } = variant

    const name = `Gruvbox ${variant.name}`

    const is_light = variant.appearance === "light"

    let neutral: string[] = []

    switch (variant.name) {
        case "Dark Hard":
            neutral = dark_hard_neutral
            break

        case "Dark":
            neutral = dark_neutral
            break

        case "Dark Soft":
            neutral = dark_soft_neutral
            break

        case "Light Hard":
            neutral = light_hard_neutral
            break

        case "Light":
            neutral = light_neutral
            break

        case "Light Soft":
            neutral = light_soft_neutral
            break
    }

    const ramps = {
        neutral: chroma.scale(is_light ? neutral.reverse() : neutral),
        red: color_ramp(chroma(variant.colors.red)),
        orange: color_ramp(chroma(variant.colors.orange)),
        yellow: color_ramp(chroma(variant.colors.yellow)),
        green: color_ramp(chroma(variant.colors.green)),
        cyan: color_ramp(chroma(variant.colors.aqua)),
        blue: color_ramp(chroma(variant.colors.blue)),
        violet: color_ramp(chroma(variant.colors.purple)),
        magenta: color_ramp(chroma(variant.colors.gray)),
    }

    const syntax: ThemeConfigInputSyntax = {
        primary: { color: neutral[is_light ? 0 : 8] },
        "text.literal": { color: colors.blue },
        comment: { color: colors.gray },
        punctuation: { color: neutral[is_light ? 1 : 7] },
        "punctuation.bracket": { color: neutral[is_light ? 3 : 5] },
        "punctuation.list_marker": { color: neutral[is_light ? 0 : 8] },
        operator: { color: colors.aqua },
        boolean: { color: colors.purple },
        number: { color: colors.purple },
        string: { color: colors.green },
        "string.special": { color: colors.purple },
        "string.special.symbol": { color: colors.aqua },
        "string.regex": { color: colors.orange },
        type: { color: colors.yellow },
        // enum: { color: colors.orange },
        tag: { color: colors.aqua },
        constant: { color: colors.yellow },
        keyword: { color: colors.red },
        function: { color: colors.green },
        "function.builtin": { color: colors.red },
        variable: { color: colors.blue },
        property: { color: neutral[is_light ? 0 : 8] },
        embedded: { color: colors.aqua },
        link_text: { color: colors.aqua },
        link_uri: { color: colors.purple },
        title: { color: colors.green },
    }

    return {
        name,
        author: meta.author,
        appearance: variant.appearance as ThemeAppearance,
        license_type: meta.license_type,
        license_url: meta.license_url,
        license_file: `${__dirname}/LICENSE`,
        input_color: ramps,
        override: { syntax },
    }
}

// Variants
export const dark_hard = build_variant(variant[0])
export const dark_default = build_variant(variant[1])
export const dark_soft = build_variant(variant[2])
export const light_hard = build_variant(variant[3])
export const light_default = build_variant(variant[4])
export const light_soft = build_variant(variant[5])
