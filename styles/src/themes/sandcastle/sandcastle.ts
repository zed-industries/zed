import {
    chroma,
    colorRamp,
    ThemeAppearance,
    ThemeLicenseType,
    ThemeConfig,
} from "../../common"

export const theme: ThemeConfig = {
    name: "Sandcastle",
    author: "gessig",
    appearance: ThemeAppearance.Dark,
    licenseType: ThemeLicenseType.MIT,
    licenseUrl: "https://github.com/gessig/base16-sandcastle-scheme",
    licenseFile: `${__dirname}/LICENSE`,
    inputColor: {
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
        red: colorRamp(chroma("#B4637A")),
        orange: colorRamp(chroma("#a07e3b")),
        yellow: colorRamp(chroma("#a07e3b")),
        green: colorRamp(chroma("#83a598")),
        cyan: colorRamp(chroma("#83a598")),
        blue: colorRamp(chroma("#528b8b")),
        violet: colorRamp(chroma("#d75f5f")),
        magenta: colorRamp(chroma("#a87322")),
    },
    override: { syntax: {} },
}
