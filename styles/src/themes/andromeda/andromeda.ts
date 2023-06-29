import {
    chroma,
    color_ramp,
    ThemeAppearance,
    ThemeLicenseType,
    ThemeConfig,
} from "../../common"

export const dark: ThemeConfig = {
    name: "Andromeda",
    author: "EliverLara",
    appearance: ThemeAppearance.Dark,
    license_type: ThemeLicenseType.MIT,
    license_url: "https://github.com/EliverLara/Andromeda",
    license_file: `${__dirname}/LICENSE`,
    input_color: {
        neutral: chroma
            .scale([
                "#1E2025",
                "#23262E",
                "#292E38",
                "#2E323C",
                "#ACA8AE",
                "#CBC9CF",
                "#E1DDE4",
                "#F7F7F8",
            ])
            .domain([0, 0.15, 0.25, 0.35, 0.7, 0.8, 0.9, 1]),
        red: color_ramp(chroma("#F92672")),
        orange: color_ramp(chroma("#F39C12")),
        yellow: color_ramp(chroma("#FFE66D")),
        green: color_ramp(chroma("#96E072")),
        cyan: color_ramp(chroma("#00E8C6")),
        blue: color_ramp(chroma("#0CA793")),
        violet: color_ramp(chroma("#8A3FA6")),
        magenta: color_ramp(chroma("#C74DED")),
    },
    override: { syntax: {} },
}
