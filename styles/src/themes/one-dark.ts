import chroma from "chroma-js"
import { Meta, ThemeSyntax } from "./common/colorScheme"
import { colorRamp, createColorScheme } from "./common/ramps"

const name = "One Dark"

const color = {
    white: "#ACB2BE",
    grey: "#5D636F",
    red: "#D07277",
    orange: "#C0966B",
    yellow: "#DFC184",
    green: "#A1C181",
    teal: "#6FB4C0",
    blue: "#74ADE9",
    purple: "#B478CF",
}

const ramps = {
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
    red: colorRamp(chroma(color.red)),
    orange: colorRamp(chroma(color.orange)),
    yellow: colorRamp(chroma(color.yellow)),
    green: colorRamp(chroma(color.green)),
    cyan: colorRamp(chroma(color.teal)),
    blue: colorRamp(chroma(color.blue)),
    violet: colorRamp(chroma(color.purple)),
    magenta: colorRamp(chroma("#be5046")),
}

const syntax: ThemeSyntax = {
    primary: { color: color.white },
    comment: { color: color.grey },
    function: { color: color.blue },
    type: { color: color.teal },
    property: { color: color.red },
    number: { color: color.orange },
    string: { color: color.green },
    keyword: { color: color.purple },
    boolean: { color: color.orange },
    punctuation: { color: color.white },
    operator: { color: color.teal },
}

export const dark = createColorScheme(name, false, ramps, syntax)

export const meta: Meta = {
    name,
    author: "simurai",
    license: {
        SPDX: "MIT",
        https_url:
            "https://raw.githubusercontent.com/atom/atom/master/packages/one-light-ui/LICENSE.md",
        license_checksum:
            "d5af8fc171f6f600c0ab4e7597dca398dda80dbe6821ce01cef78e859e7a00f8",
    },
    url: "https://github.com/atom/atom/tree/master/packages/one-dark-ui",
}
