import chroma from "chroma-js"
import { Meta } from "../common/colorScheme"
import { colorRamp, createColorScheme } from "../common/ramps"
import { metaCommon, name, buildSyntax, Variant } from "../common/atelier-common"

const variant: Variant = {
    meta: {
        name: `${name} Savanna Light`,
        ...metaCommon,
        url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/savanna/",
    },
    colors: {
        base00: "#ecf4ee",
        base01: "#dfe7e2",
        base02: "#87928a",
        base03: "#78877d",
        base04: "#5f6d64",
        base05: "#526057",
        base06: "#232a25",
        base07: "#171c19",
        base08: "#b16139",
        base09: "#9f713c",
        base0A: "#a07e3b",
        base0B: "#489963",
        base0C: "#1c9aa0",
        base0D: "#478c90",
        base0E: "#55859b",
        base0F: "#867469",
    },
}

const syntax = buildSyntax(variant)

const theme = (variant: Variant) => {
    const { meta, colors } = variant

    return createColorScheme(
        meta.name,
        true,
        {
            neutral: chroma.scale(
                [
                    colors.base00,
                    colors.base01,
                    colors.base02,
                    colors.base03,
                    colors.base04,
                    colors.base05,
                    colors.base06,
                    colors.base07,
                ].reverse()
            ),
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
