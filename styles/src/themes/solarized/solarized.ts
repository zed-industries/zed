import {
    chroma,
    color_ramp,
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
    red: color_ramp(chroma("#dc322f")),
    orange: color_ramp(chroma("#cb4b16")),
    yellow: color_ramp(chroma("#b58900")),
    green: color_ramp(chroma("#859900")),
    cyan: color_ramp(chroma("#2aa198")),
    blue: color_ramp(chroma("#268bd2")),
    violet: color_ramp(chroma("#6c71c4")),
    magenta: color_ramp(chroma("#d33682")),
}

export const dark: ThemeConfig = {
    name: "Solarized Dark",
    author: "Ethan Schoonover",
    appearance: ThemeAppearance.Dark,
    license_type: ThemeLicenseType.MIT,
    license_url: "https://github.com/altercation/solarized",
    license_file: `${__dirname}/LICENSE`,
    input_color: ramps,
    override: { syntax: {} },
}

export const light: ThemeConfig = {
    name: "Solarized Light",
    author: "Ethan Schoonover",
    appearance: ThemeAppearance.Light,
    license_type: ThemeLicenseType.MIT,
    license_url: "https://github.com/altercation/solarized",
    license_file: `${__dirname}/LICENSE`,
    input_color: ramps,
    override: { syntax: {} },
}
