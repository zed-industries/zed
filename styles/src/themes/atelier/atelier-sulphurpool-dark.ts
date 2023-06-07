import { chroma, ThemeAppearance, ThemeConfig, colorRamp } from "../../common"
import { meta, buildSyntax, Variant } from "./common"

const variant: Variant = {
    colors: {
        base00: "#202746",
        base01: "#293256",
        base02: "#5e6687",
        base03: "#6b7394",
        base04: "#898ea4",
        base05: "#979db4",
        base06: "#dfe2f1",
        base07: "#f5f7ff",
        base08: "#c94922",
        base09: "#c76b29",
        base0A: "#c08b30",
        base0B: "#ac9739",
        base0C: "#22a2c9",
        base0D: "#3d8fd1",
        base0E: "#6679cc",
        base0F: "#9c637a",
    },
}

const syntax = buildSyntax(variant)

const getTheme = (variant: Variant): ThemeConfig => {
    const { colors } = variant

    return {
        name: `${meta.name} Sulphurpool Dark`,
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
