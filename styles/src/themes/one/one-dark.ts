import {
    chroma,
    font_weights,
    color_ramp,
    ThemeAppearance,
    ThemeLicenseType,
    ThemeConfig,
} from "../../common"

const color = {
    white: "#ACB2BE",
    grey: "#5D636F",
    red: "#D07277",
    dark_red: "#B1574B",
    orange: "#C0966B",
    yellow: "#DFC184",
    green: "#A1C181",
    teal: "#6FB4C0",
    blue: "#74ADE9",
    purple: "#B478CF",
}

export const theme: ThemeConfig = {
    name: "One Dark",
    author: "simurai",
    appearance: ThemeAppearance.Dark,
    license_type: ThemeLicenseType.MIT,
    license_url:
        "https://github.com/atom/atom/tree/master/packages/one-dark-ui",
    license_file: `${__dirname}/LICENSE`,
    input_color: {
        neutral: chroma
            .scale([
                "#282c34",
                "#353b45",
                "#3e4451",
                "#545862",
                "#565c64",
                "#abb2bf",
                "#b6bdca",
                "#c8ccd4",
            ])
            .domain([0.05, 0.22, 0.25, 0.45, 0.62, 0.8, 0.9, 1]),
        red: color_ramp(chroma(color.red)),
        orange: color_ramp(chroma(color.orange)),
        yellow: color_ramp(chroma(color.yellow)),
        green: color_ramp(chroma(color.green)),
        cyan: color_ramp(chroma(color.teal)),
        blue: color_ramp(chroma(color.blue)),
        violet: color_ramp(chroma(color.purple)),
        magenta: color_ramp(chroma("#be5046")),
    },
    override: {
        syntax: {
            boolean: { color: color.orange },
            comment: { color: color.grey },
            "emphasis.strong": { color: color.orange },
            function: { color: color.blue },
            keyword: { color: color.purple },
            link_text: { color: color.blue, italic: false },
            link_uri: { color: color.teal },
            number: { color: color.orange },
            constant: { color: color.yellow },
            operator: { color: color.teal },
            primary: { color: color.white },
            property: { color: color.red },
            punctuation: { color: color.white },
            "punctuation.list_marker": { color: color.red },
            "punctuation.special": { color: color.dark_red },
            string: { color: color.green },
            title: { color: color.red, weight: font_weights.normal },
            "text.literal": { color: color.green },
            type: { color: color.teal },
            "variable.special": { color: color.orange },
            "method.constructor": { color: color.blue },
        },
    },
}
