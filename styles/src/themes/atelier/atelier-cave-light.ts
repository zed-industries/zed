import chroma from "chroma-js"
import { Meta, colorRamp, createColorScheme, ThemeAppearance } from "../common"
import { metaCommon, name, buildSyntax, Variant } from "./common"

const variant: Variant = {
    meta: {
        name: `${name} Cave Light`,
        ...metaCommon,
        url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/cave/",
    },
    colors: {
        base00: "#efecf4",
        base01: "#e2dfe7",
        base02: "#8b8792",
        base03: "#7e7887",
        base04: "#655f6d",
        base05: "#585260",
        base06: "#26232a",
        base07: "#19171c",
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
        appearance: ThemeAppearance.Light,
        inputColor: {
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
        override: { syntax },
    })
}

export const dark = theme(variant)

export const meta: Meta = variant.meta
