import chroma from "chroma-js"
import { Meta } from "../common/colorScheme"
import { colorRamp, createColorScheme } from "../common/ramps"
import { metaCommon, name, buildSyntax, Variant } from "./common"

const variant: Variant = {
    meta: {
        name: `${name} Estuary Dark`,
        ...metaCommon,
        url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/estuary/",
    },
    colors: {
        base00: "#22221b",
        base01: "#302f27",
        base02: "#5f5e4e",
        base03: "#6c6b5a",
        base04: "#878573",
        base05: "#929181",
        base06: "#e7e6df",
        base07: "#f4f3ec",
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
