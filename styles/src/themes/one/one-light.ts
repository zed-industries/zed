import {
    chroma,
    fontWeights,
    colorRamp,
    ThemeAppearance,
    ThemeLicenseType,
    ThemeConfig,
} from "../../common"

const color = {
    black: "#383A41",
    grey: "#A2A3A7",
    red: "#D36050",
    darkRed: "#B92C46",
    orange: "#AD6F26",
    yellow: "#DFC184",
    green: "#659F58",
    teal: "#3982B7",
    blue: "#5B79E3",
    purple: "#A449AB",
    magenta: "#994EA6",
}

export const theme: ThemeConfig = {
    name: "One Light",
    author: "simurai",
    appearance: ThemeAppearance.Light,
    licenseType: ThemeLicenseType.MIT,
    licenseUrl:
        "https://github.com/atom/atom/tree/master/packages/one-light-ui",
    licenseFile: `${__dirname}/LICENSE`,
    inputColor: {
        neutral: chroma
            .scale([
                "#383A41",
                "#535456",
                "#696c77",
                "#9D9D9F",
                "#A9A9A9",
                "#DBDBDC",
                "#EAEAEB",
                "#FAFAFA",
            ])
            .domain([0.05, 0.22, 0.25, 0.45, 0.62, 0.8, 0.9, 1]),
        red: colorRamp(chroma(color.red)),
        orange: colorRamp(chroma(color.orange)),
        yellow: colorRamp(chroma(color.yellow)),
        green: colorRamp(chroma(color.green)),
        cyan: colorRamp(chroma(color.teal)),
        blue: colorRamp(chroma(color.blue)),
        violet: colorRamp(chroma(color.purple)),
        magenta: colorRamp(chroma(color.magenta)),
    },
    override: {
        syntax: {
            boolean: { color: color.orange },
            comment: { color: color.grey },
            enum: { color: color.red },
            "emphasis.strong": { color: color.orange },
            function: { color: color.blue },
            keyword: { color: color.purple },
            linkText: { color: color.blue },
            linkUri: { color: color.teal },
            number: { color: color.orange },
            operator: { color: color.teal },
            primary: { color: color.black },
            property: { color: color.red },
            punctuation: { color: color.black },
            "punctuation.list_marker": { color: color.red },
            "punctuation.special": { color: color.darkRed },
            string: { color: color.green },
            title: { color: color.red, weight: fontWeights.normal },
            "text.literal": { color: color.green },
            type: { color: color.teal },
            "variable.special": { color: color.orange },
            variant: { color: color.blue },
        },
    },
}
