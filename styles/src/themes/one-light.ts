import chroma from "chroma-js"
import { fontWeights } from "../common";
import { Meta, ThemeSyntax } from "./common/colorScheme"
import { colorRamp, createColorScheme } from "./common/ramps"

const name = "One Light"

const color = {
    black: "#383A41",
    grey: "#A2A3A7",
    red: "#D36050",
    orange: "#AD6F26",
    yellow: "#DFC184",
    green: "#659F58",
    teal: "#3982B7",
    blue: "#5B79E3",
    purple: "#A449AB",
    magenta: "#994EA6"
};

const ramps = {
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
        .domain([0.05, 0.22, 0.25, 0.45, 0.62, 0.8, 0.9, 1])
    ,
    red: colorRamp(chroma(color.red)),
    orange: colorRamp(chroma(color.orange)),
    yellow: colorRamp(chroma(color.yellow)),
    green: colorRamp(chroma(color.green)),
    cyan: colorRamp(chroma(color.teal)),
    blue: colorRamp(chroma(color.blue)),
    violet: colorRamp(chroma(color.purple)),
    magenta: colorRamp(chroma(color.magenta)),
};

const syntax: ThemeSyntax = {
    primary: { color: color.black },
    "variable.special": { color: color.orange },
    comment: { color: color.grey },
    punctuation: { color: color.black },
    keyword: { color: color.purple },
    function: { color: color.blue },
    type: { color: color.teal },
    variant: { color: color.blue },
    property: { color: color.red },
    enum: { color: color.red },
    operator: { color: color.teal },
    string: { color: color.green },
    number: { color: color.orange },
    boolean: { color: color.orange },
    title: { color: color.red, weight: fontWeights.normal },
    "emphasis.strong": {
        color: color.orange,
    },
    linkText: { color: color.blue },
    linkUri: { color: color.teal },
}

export const light = createColorScheme(name, true, ramps, syntax)

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
    url: "https://github.com/atom/atom/tree/master/packages/one-light-ui",
}
