import { chroma, ThemeAppearance, ThemeConfig, color_ramp } from "../../common"
import { meta, build_syntax, Variant } from "./common"

const variant: Variant = {
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

const syntax = build_syntax(variant)

const get_theme = (variant: Variant): ThemeConfig => {
    const { colors } = variant

    return {
        name: `${meta.name} Plateau Light`,
        author: meta.author,
        appearance: ThemeAppearance.Light,
        license_type: meta.license_type,
        license_url: meta.license_url,
        license_file: `${__dirname}/LICENSE`,
        input_color: {
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
