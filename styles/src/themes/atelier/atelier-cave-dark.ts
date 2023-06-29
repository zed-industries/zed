import { chroma, ThemeAppearance, ThemeConfig, color_ramp } from "../../common"
import { meta, build_syntax, Variant } from "./common"

const variant: Variant = {
    colors: {
        base00: "#19171c",
        base01: "#26232a",
        base02: "#585260",
        base03: "#655f6d",
        base04: "#7e7887",
        base05: "#8b8792",
        base06: "#e2dfe7",
        base07: "#efecf4",
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

const syntax = build_syntax(variant)

const get_theme = (variant: Variant): ThemeConfig => {
    const { colors } = variant

    return {
        name: `${meta.name} Cave Dark`,
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
