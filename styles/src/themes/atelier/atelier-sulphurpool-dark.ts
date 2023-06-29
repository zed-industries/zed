import { chroma, ThemeAppearance, ThemeConfig, color_ramp } from "../../common"
import { meta, build_syntax, Variant } from "./common"

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

const syntax = build_syntax(variant)

const get_theme = (variant: Variant): ThemeConfig => {
    const { colors } = variant

    return {
        name: `${meta.name} Sulphurpool Dark`,
        author: meta.author,
        appearance: ThemeAppearance.Dark,
        license_type: meta.license_type,
        license_url: meta.license_url,
        license_file: `${__dirname}/LICENSE`,
        input_color: {
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
            red: color_ramp(chroma(colors.base08)),
            orange: color_ramp(chroma(colors.base09)),
            yellow: color_ramp(chroma(colors.base0A)),
            green: color_ramp(chroma(colors.base0B)),
            cyan: color_ramp(chroma(colors.base0C)),
            blue: color_ramp(chroma(colors.base0D)),
            violet: color_ramp(chroma(colors.base0E)),
            magenta: color_ramp(chroma(colors.base0F)),
        },
        override: { syntax },
    }
}

export const theme = get_theme(variant)
