import chroma from "chroma-js"
import { Meta as Metadata } from "./common/colorScheme"
import { colorRamp, createColorScheme } from "./common/ramps"

const name = "Solarized"

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

export const dark = createColorScheme(`${name} Dark`, false, ramps)
export const light = createColorScheme(`${name} Light`, true, ramps)

export const meta: Metadata = {
    name,
    author: "Ethan Schoonover",
    license: {
        SPDX: "MIT",
        license_text: {
            https_url:
                "https://raw.githubusercontent.com/altercation/solarized/master/LICENSE",
        },
    },
    url: "https://github.com/altercation/solarized",
}
