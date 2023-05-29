import chroma from "chroma-js"
import { Meta } from "../common/colorScheme"
import { colorRamp, createColorScheme } from "../common/ramps"
import { metaCommon, name, buildSyntax, Variant } from "../common/atelier-common"

const variant: Variant = {
    meta: {
        name: `${name} Heath Dark`,
        ...metaCommon,
        url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/heath/",
    },
    colors: {
        base00: "#1b181b",
        base01: "#292329",
        base02: "#695d69",
        base03: "#776977",
        base04: "#9e8f9e",
        base05: "#ab9bab",
        base06: "#d8cad8",
        base07: "#f7f3f7",
        base08: "#ca402b",
        base09: "#a65926",
        base0A: "#bb8a35",
        base0B: "#918b3b",
        base0C: "#159393",
        base0D: "#516aec",
        base0E: "#7b59c0",
        base0F: "#cc33cc",
    },
}

const syntax = buildSyntax(variant)

const theme = (variant: Variant) => {
    const { meta, colors } = variant

    return createColorScheme(
        meta.name,
        false,
        {
            neutral: chroma.scale([
                colors.base00,
                colors.base01,
                colors.base02,
                colors.base03,
                colors.base04,
                colors.base05,
                colors.base06,
                colors.base07,
            ]),
            red: colorRamp(chroma(colors.base08)),
            orange: colorRamp(chroma(colors.base09)),
            yellow: colorRamp(chroma(colors.base0A)),
            green: colorRamp(chroma(colors.base0B)),
            cyan: colorRamp(chroma(colors.base0C)),
            blue: colorRamp(chroma(colors.base0D)),
            violet: colorRamp(chroma(colors.base0E)),
            magenta: colorRamp(chroma(colors.base0F)),
        },
        syntax
    )
}

export const dark = theme(variant)

export const meta: Meta = variant.meta
