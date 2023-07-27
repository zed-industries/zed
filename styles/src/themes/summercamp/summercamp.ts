import {
    chroma,
    color_ramp,
    ThemeAppearance,
    ThemeLicenseType,
    ThemeConfig,
} from "../../common"

export const theme: ThemeConfig = {
    name: "Summercamp",
    author: "zoefiri",
    appearance: ThemeAppearance.Dark,
    license_type: ThemeLicenseType.MIT,
    license_url: "https://github.com/zoefiri/base16-sc",
    license_file: `${__dirname}/LICENSE`,
    input_color: {
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
        red: color_ramp(chroma("#e35142")),
        orange: color_ramp(chroma("#fba11b")),
        yellow: color_ramp(chroma("#f2ff27")),
        green: color_ramp(chroma("#5ceb5a")),
        cyan: color_ramp(chroma("#5aebbc")),
        blue: color_ramp(chroma("#489bf0")),
        violet: color_ramp(chroma("#FF8080")),
        magenta: color_ramp(chroma("#F69BE7")),
    },
    override: { syntax: {} },
}
