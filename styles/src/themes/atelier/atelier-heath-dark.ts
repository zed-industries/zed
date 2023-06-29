import { chroma, ThemeAppearance, ThemeConfig, color_ramp } from "../../common"
import { meta, build_syntax, Variant } from "./common"

const variant: Variant = {
    colors: {
        base00: "#1b181b",
        base01: "#292329",
        base02: "#695d69",
        base03: "#776977",
        base04: "#9e8f9e",
        base05: "#ab9bab",
        base06: "#d8cad8",
        base07: "#f7f3f7",
        base08: "#ca402b",
        base09: "#a65926",
        base0A: "#bb8a35",
        base0B: "#918b3b",
        base0C: "#159393",
        base0D: "#516aec",
        base0E: "#7b59c0",
        base0F: "#cc33cc",
    },
}

const syntax = build_syntax(variant)

const get_theme = (variant: Variant): ThemeConfig => {
    const { colors } = variant

    return {
        name: `${meta.name} Heath Dark`,
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
