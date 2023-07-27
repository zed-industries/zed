import {
    chroma,
    color_ramp,
    ThemeAppearance,
    ThemeLicenseType,
    ThemeConfig,
} from "../../common"

export const theme: ThemeConfig = {
    name: "Sandcastle",
    author: "gessig",
    appearance: ThemeAppearance.Dark,
    license_type: ThemeLicenseType.MIT,
    license_url: "https://github.com/gessig/base16-sandcastle-scheme",
    license_file: `${__dirname}/LICENSE`,
    input_color: {
        neutral: chroma.scale([
            "#282c34",
            "#2c323b",
            "#3e4451",
            "#665c54",
            "#928374",
            "#a89984",
            "#d5c4a1",
            "#fdf4c1",
        ]),
        red: color_ramp(chroma("#B4637A")),
        orange: color_ramp(chroma("#a07e3b")),
        yellow: color_ramp(chroma("#a07e3b")),
        green: color_ramp(chroma("#83a598")),
        cyan: color_ramp(chroma("#83a598")),
        blue: color_ramp(chroma("#528b8b")),
        violet: color_ramp(chroma("#d75f5f")),
        magenta: color_ramp(chroma("#a87322")),
    },
    override: { syntax: {} },
}
