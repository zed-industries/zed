import chroma from "chroma-js"
import { Meta, colorRamp, createColorScheme, ThemeAppearance } from "../common"
import { metaCommon, name, buildSyntax, Variant } from "./common"

const variant: Variant = {
    meta: {
        name: `${name} Dune Dark`,
        ...metaCommon,
        url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/dune/",
    },
    colors: {
        base00: "#20201d",
        base01: "#292824",
        base02: "#6e6b5e",
        base03: "#7d7a68",
        base04: "#999580",
        base05: "#a6a28c",
        base06: "#e8e4cf",
        base07: "#fefbec",
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

    return createColorScheme({
        name: meta.name,
        author: meta.author,
        appearance: ThemeAppearance.Dark,
        inputColor: {
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
        override: { syntax },
    })
}

export const dark = theme(variant)

export const meta: Meta = variant.meta
