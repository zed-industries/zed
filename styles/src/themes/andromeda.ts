import chroma from "chroma-js"
import { Meta, ThemeSyntax } from "./common/colorScheme"
import { colorRamp, createColorScheme } from "./common/ramps"

const name = "Andromeda"

const color = {
    text: "#D5CED9",
    gray: "#5f6167",
    cyan: "#00e8c6",
    orange: "#f39c12",
    yellow: "#FFE66D",
    pink: "#ff00aa",
    hotPink: "#f92672",
    purple: "#c74ded",
    blue: "#7cb7ff",
    red: "#ee5d43",
    green: "#96E072",
}

const ramps = {
    neutral: chroma
        .scale([
            "#24262D",
            "#292E38",
            "#2E323C",
            "#ACA8AE",
            "#CBC9CF",
            "#E1DDE4",
            "#F7F7F8",
        ])
        .domain([0, 0.15, 0.25, 0.35, 0.7, 0.8, 0.9, 1]),
    red: colorRamp(chroma(color.red)),
    orange: colorRamp(chroma(color.orange)),
    yellow: colorRamp(chroma(color.yellow)),
    green: colorRamp(chroma(color.green)),
    cyan: colorRamp(chroma(color.cyan)),
    blue: colorRamp(chroma(color.blue)),
    violet: colorRamp(chroma(color.purple)),
    magenta: colorRamp(chroma(color.hotPink)),
}

const syntax: ThemeSyntax = {
    variable: { color: color.cyan },
    "variable.special": { color: color.cyan },
    "punctuation.special": { color: color.red },
    attribute: { color: color.text },
    boolean: { color: color.red },
    comment: { color: color.gray },
    function: { color: color.yellow },
    keyword: { color: color.purple },
    number: { color: color.orange },
    operator: { color: color.red },
    primary: { color: color.text },
    property: { color: color.text },
    string: { color: color.green },
    type: { color: color.yellow },
    title: { color: color.hotPink },
    linkText: { color: color.red },
    linkUri: { color: color.purple },
    "text.literal": { color: color.green },
    "punctuation.list_marker": { color: color.yellow },
}

export const dark = createColorScheme(`${name}`, false, ramps, syntax)

export const meta: Meta = {
    name,
    author: "EliverLara",
    license: {
        SPDX: "MIT",
        https_url:
            "https://raw.githubusercontent.com/EliverLara/Andromeda/master/LICENSE.md",
        license_checksum:
            "2f7886f1a05cefc2c26f5e49de1a39fa4466413c1ccb06fc80960e73f5ed4b89",
    },
    url: "https://github.com/EliverLara/Andromeda",
}
