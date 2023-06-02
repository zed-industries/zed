import chroma from "chroma-js"
import { Meta, colorRamp, createColorScheme, ThemeAppearance } from "../common"
import { metaCommon, name, buildSyntax, Variant } from "./common"

const variant: Variant = {
    meta: {
        name: `${name} Estuary Light`,
        ...metaCommon,
        url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/estuary/",
    },
    colors: {
        base00: "#f4f3ec",
        base01: "#e7e6df",
        base02: "#929181",
        base03: "#878573",
        base04: "#6c6b5a",
        base05: "#5f5e4e",
        base06: "#302f27",
        base07: "#22221b",
        base08: "#ba6236",
        base09: "#ae7313",
        base0A: "#a5980d",
        base0B: "#7d9726",
        base0C: "#5b9d48",
        base0D: "#36a166",
        base0E: "#5f9182",
        base0F: "#9d6c7c",
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
