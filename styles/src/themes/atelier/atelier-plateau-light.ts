import chroma from "chroma-js"
import { Meta, colorRamp, createColorScheme, ThemeAppearance } from "../common"
import { metaCommon, name, buildSyntax, Variant } from "./common"

const variant: Variant = {
    meta: {
        name: `${name} Plateau Light`,
        ...metaCommon,
        url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/plateau/",
    },
    colors: {
        base00: "#f4ecec",
        base01: "#e7dfdf",
        base02: "#8a8585",
        base03: "#7e7777",
        base04: "#655d5d",
        base05: "#585050",
        base06: "#292424",
        base07: "#1b1818",
        base08: "#ca4949",
        base09: "#b45a3c",
        base0A: "#a06e3b",
        base0B: "#4b8b8b",
        base0C: "#5485b6",
        base0D: "#7272ca",
        base0E: "#8464c4",
        base0F: "#bd5187",
    },
}

const syntax = buildSyntax(variant)

const theme = (variant: Variant) => {
    const { meta, colors } = variant

    return createColorScheme({
        name: meta.name,
        author: meta.author,
        appearance: ThemeAppearance.Light,
        inputColor:{
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
        override: { syntax },}
    )
}

export const dark = theme(variant)

export const meta: Meta = variant.meta
