import { chroma, ThemeAppearance, ThemeConfig, color_ramp } from "../../common"
import { meta, build_syntax, Variant } from "./common"

const variant: Variant = {
    colors: {
        base00: "#20201d",
        base01: "#292824",
        base02: "#6e6b5e",
        base03: "#7d7a68",
        base04: "#999580",
        base05: "#a6a28c",
        base06: "#e8e4cf",
        base07: "#fefbec",
        base08: "#d73737",
        base09: "#b65611",
        base0A: "#ae9513",
        base0B: "#60ac39",
        base0C: "#1fad83",
        base0D: "#6684e1",
        base0E: "#b854d4",
        base0F: "#d43552",
    },
}

const syntax = build_syntax(variant)

const get_theme = (variant: Variant): ThemeConfig => {
    const { colors } = variant

    return {
        name: `${meta.name} Dune Dark`,
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
