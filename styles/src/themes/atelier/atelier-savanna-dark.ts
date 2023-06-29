import { chroma, ThemeAppearance, ThemeConfig, color_ramp } from "../../common"
import { meta, build_syntax, Variant } from "./common"

const variant: Variant = {
    colors: {
        base00: "#171c19",
        base01: "#232a25",
        base02: "#526057",
        base03: "#5f6d64",
        base04: "#78877d",
        base05: "#87928a",
        base06: "#dfe7e2",
        base07: "#ecf4ee",
        base08: "#b16139",
        base09: "#9f713c",
        base0A: "#a07e3b",
        base0B: "#489963",
        base0C: "#1c9aa0",
        base0D: "#478c90",
        base0E: "#55859b",
        base0F: "#867469",
    },
}

const syntax = build_syntax(variant)

const get_theme = (variant: Variant): ThemeConfig => {
    const { colors } = variant

    return {
        name: `${meta.name} Savanna Dark`,
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
