import {
    chroma,
    colorRamp,
    ThemeAppearance,
    ThemeLicenseType,
    ThemeConfig,
} from "../../common"

const ramps = {
    neutral: chroma
        .scale([
            "#002b36",
            "#073642",
            "#586e75",
            "#657b83",
            "#839496",
            "#93a1a1",
            "#eee8d5",
            "#fdf6e3",
        ])
        .domain([0, 0.2, 0.38, 0.45, 0.65, 0.7, 0.85, 1]),
    red: colorRamp(chroma("#dc322f")),
    orange: colorRamp(chroma("#cb4b16")),
    yellow: colorRamp(chroma("#b58900")),
    green: colorRamp(chroma("#859900")),
    cyan: colorRamp(chroma("#2aa198")),
    blue: colorRamp(chroma("#268bd2")),
    violet: colorRamp(chroma("#6c71c4")),
    magenta: colorRamp(chroma("#d33682")),
}

export const dark: ThemeConfig = {
    name: "Solarized Dark",
    author: "Ethan Schoonover",
    appearance: ThemeAppearance.Dark,
    licenseType: ThemeLicenseType.MIT,
    licenseUrl: "https://github.com/altercation/solarized",
    licenseFile: `${__dirname}/LICENSE`,
    inputColor: ramps,
    override: { syntax: {} },
}

export const light: ThemeConfig = {
    name: "Solarized Light",
    author: "Ethan Schoonover",
    appearance: ThemeAppearance.Light,
    licenseType: ThemeLicenseType.MIT,
    licenseUrl: "https://github.com/altercation/solarized",
    licenseFile: `${__dirname}/LICENSE`,
    inputColor: ramps,
    override: { syntax: {} },
}
