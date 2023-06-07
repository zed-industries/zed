import { chroma, ThemeAppearance, ThemeConfig, colorRamp } from "../../common"
import { meta, buildSyntax, Variant } from "./common"

const variant: Variant = {
    colors: {
        base00: "#161b1d",
        base01: "#1f292e",
        base02: "#516d7b",
        base03: "#5a7b8c",
        base04: "#7195a8",
        base05: "#7ea2b4",
        base06: "#c1e4f6",
        base07: "#ebf8ff",
        base08: "#d22d72",
        base09: "#935c25",
        base0A: "#8a8a0f",
        base0B: "#568c3b",
        base0C: "#2d8f6f",
        base0D: "#257fad",
        base0E: "#6b6bb8",
        base0F: "#b72dd2",
    },
}

const syntax = buildSyntax(variant)

const getTheme = (variant: Variant): ThemeConfig => {
    const { colors } = variant

    return {
        name: `${meta.name} Lakeside Dark`,
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
