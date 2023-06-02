import chroma from "chroma-js"
import { Meta, colorRamp, createColorScheme, ThemeAppearance } from "../common"
import { metaCommon, name, buildSyntax, Variant } from "./common"

const variant: Variant = {
    meta: {
        name: `${name} Cave Dark`,
        ...metaCommon,
        url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/cave/",
    },
    colors: {
        base00: "#19171c",
        base01: "#26232a",
        base02: "#585260",
        base03: "#655f6d",
        base04: "#7e7887",
        base05: "#8b8792",
        base06: "#e2dfe7",
        base07: "#efecf4",
        base08: "#be4678",
        base09: "#aa573c",
        base0A: "#a06e3b",
        base0B: "#2a9292",
        base0C: "#398bc6",
        base0D: "#576ddb",
        base0E: "#955ae7",
        base0F: "#bf40bf",
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
