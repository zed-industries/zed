import chroma from "chroma-js"
import { Meta, colorRamp, createColorScheme, ThemeAppearance } from "../common"
import { metaCommon, name, buildSyntax, Variant } from "./common"

const variant: Variant = {
    meta: {
        name: `${name} Seaside Dark`,
        ...metaCommon,
        url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/seaside/",
    },
    colors: {
        base00: "#131513",
        base01: "#242924",
        base02: "#5e6e5e",
        base03: "#687d68",
        base04: "#809980",
        base05: "#8ca68c",
        base06: "#cfe8cf",
        base07: "#f4fbf4",
        base08: "#e6193c",
        base09: "#87711d",
        base0A: "#98981b",
        base0B: "#29a329",
        base0C: "#1999b3",
        base0D: "#3d62f5",
        base0E: "#ad2bee",
        base0F: "#e619c3",
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
