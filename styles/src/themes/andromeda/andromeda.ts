import {
    chroma,
    colorRamp,
    ThemeAppearance,
    ThemeLicenseType,
    ThemeConfig,
} from "../../common"

export const dark: ThemeConfig = {
    name: "Andromeda",
    author: "EliverLara",
    appearance: ThemeAppearance.Dark,
    licenseType: ThemeLicenseType.MIT,
    licenseUrl: "https://github.com/EliverLara/Andromeda",
    licenseFile: `${__dirname}/LICENSE`,
    inputColor: {
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
        red: colorRamp(chroma("#F92672")),
        orange: colorRamp(chroma("#F39C12")),
        yellow: colorRamp(chroma("#FFE66D")),
        green: colorRamp(chroma("#96E072")),
        cyan: colorRamp(chroma("#00E8C6")),
        blue: colorRamp(chroma("#0CA793")),
        violet: colorRamp(chroma("#8A3FA6")),
        magenta: colorRamp(chroma("#C74DED")),
    },
    override: { syntax: {} },
}
