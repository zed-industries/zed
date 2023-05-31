import chroma from "chroma-js"
import { Meta } from "../common/colorScheme"
import { colorRamp, createColorScheme } from "../common/ramps"
import { metaCommon, name, buildSyntax, Variant } from "./common"

const variant: Variant = {
    meta: {
        name: `${name} Dune Light`,
        ...metaCommon,
        url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/dune/",
    },
    colors: {
        base00: "#fefbec",
        base01: "#e8e4cf",
        base02: "#a6a28c",
        base03: "#999580",
        base04: "#7d7a68",
        base05: "#6e6b5e",
        base06: "#292824",
        base07: "#20201d",
        base08: "#d73737",
        base09: "#b65611",
        base0A: "#ae9513",
        base0B: "#60ac39",
        base0C: "#1fad83",
        base0D: "#6684e1",
        base0E: "#b854d4",
        base0F: "#d43552",
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
