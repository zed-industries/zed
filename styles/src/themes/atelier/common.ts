import {
    ThemeLicenseType,
    ThemeFamilyMeta,
    ThemeConfigInputSyntax,
} from "../../common"

export interface Variant {
    colors: {
        base00: string
        base01: string
        base02: string
        base03: string
        base04: string
        base05: string
        base06: string
        base07: string
        base08: string
        base09: string
        base0A: string
        base0B: string
        base0C: string
        base0D: string
        base0E: string
        base0F: string
    }
}

export const meta: ThemeFamilyMeta = {
    name: "Atelier",
    author: "Bram de Haan (http://atelierbramdehaan.nl)",
    license_type: ThemeLicenseType.MIT,
    license_url:
        "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/cave/",
}

export const build_syntax = (variant: Variant): ThemeConfigInputSyntax => {
    const { colors } = variant
    return {
        primary: { color: colors.base06 },
        comment: { color: colors.base03 },
        "punctuation.delimiter": { color: colors.base05 },
        "punctuation.bracket": { color: colors.base05 },
        "punctuation.special": { color: colors.base0F },
        "string.special.symbol": { color: colors.base0B },
        operator: { color: colors.base05 },
        function: { color: colors.base0D },
        "function.method": { color: colors.base0D },
        "function.special.definition": { color: colors.base0A },
        string: { color: colors.base0B },
        "string.special": { color: colors.base0F },
        "string.regex": { color: colors.base0C },
        type: { color: colors.base0A },
        number: { color: colors.base09 },
        property: { color: colors.base08 },
        variable: { color: colors.base06 },
        "variable.special": { color: colors.base0E },
        keyword: { color: colors.base0E },
    }
}
