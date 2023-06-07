import { chroma, ThemeAppearance, ThemeConfig, colorRamp } from "../../common"
import { meta, buildSyntax, Variant } from "./common"

const variant: Variant = {
    colors: {
        base00: "#1b1818",
        base01: "#292424",
        base02: "#585050",
        base03: "#655d5d",
        base04: "#7e7777",
        base05: "#8a8585",
        base06: "#e7dfdf",
        base07: "#f4ecec",
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

const getTheme = (variant: Variant): ThemeConfig => {
    const { colors } = variant

    return {
        name: `${meta.name} Plateau Dark`,
        author: meta.author,
        appearance: ThemeAppearance.Dark,
        licenseType: meta.licenseType,
        licenseUrl: meta.licenseUrl,
        licenseFile: `${__dirname}/LICENSE`,
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
    }
}

export const theme = getTheme(variant)
