import {
    chroma,
    font_weights,
    color_ramp,
    ThemeAppearance,
    ThemeLicenseType,
    ThemeConfig,
} from "../../common"

const color = {
    black: "#383A41",
    grey: "#A2A3A7",
    red: "#D36050",
    dark_red: "#B92C46",
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
    license_type: ThemeLicenseType.MIT,
    license_url:
        "https://github.com/atom/atom/tree/master/packages/one-light-ui",
    license_file: `${__dirname}/LICENSE`,
    input_color: {
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
        red: color_ramp(chroma(color.red)),
        orange: color_ramp(chroma(color.orange)),
        yellow: color_ramp(chroma(color.yellow)),
        green: color_ramp(chroma(color.green)),
        cyan: color_ramp(chroma(color.teal)),
        blue: color_ramp(chroma(color.blue)),
        violet: color_ramp(chroma(color.purple)),
        magenta: color_ramp(chroma(color.magenta)),
    },
    override: {
        syntax: {
            boolean: { color: color.orange },
            comment: { color: color.grey },
            "emphasis.strong": { color: color.orange },
            function: { color: color.blue },
            keyword: { color: color.purple },
            link_text: { color: color.blue },
            link_uri: { color: color.teal },
            number: { color: color.orange },
            operator: { color: color.teal },
            primary: { color: color.black },
            property: { color: color.red },
            punctuation: { color: color.black },
            "punctuation.list_marker": { color: color.red },
            "punctuation.special": { color: color.dark_red },
            string: { color: color.green },
            title: { color: color.red, weight: font_weights.normal },
            "text.literal": { color: color.green },
            type: { color: color.teal },
            "variable.special": { color: color.orange },
        },
    },
}
