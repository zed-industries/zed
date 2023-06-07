import {
    chroma,
    colorRamp,
    ThemeAppearance,
    ThemeLicenseType,
    ThemeConfig,
} from "../../common"

export const theme: ThemeConfig = {
    name: "Summercamp",
    author: "zoefiri",
    appearance: ThemeAppearance.Dark,
    licenseType: ThemeLicenseType.MIT,
    licenseUrl: "https://github.com/zoefiri/base16-sc",
    licenseFile: `${__dirname}/LICENSE`,
    inputColor: {
        neutral: chroma
            .scale([
                "#1c1810",
                "#2a261c",
                "#3a3527",
                "#3a3527",
                "#5f5b45",
                "#736e55",
                "#bab696",
                "#f8f5de",
            ])
            .domain([0, 0.2, 0.38, 0.4, 0.65, 0.7, 0.85, 1]),
        red: colorRamp(chroma("#e35142")),
        orange: colorRamp(chroma("#fba11b")),
        yellow: colorRamp(chroma("#f2ff27")),
        green: colorRamp(chroma("#5ceb5a")),
        cyan: colorRamp(chroma("#5aebbc")),
        blue: colorRamp(chroma("#489bf0")),
        violet: colorRamp(chroma("#FF8080")),
        magenta: colorRamp(chroma("#F69BE7")),
    },
    override: { syntax: {} },
}
