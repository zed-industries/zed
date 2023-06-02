import chroma from "chroma-js"
import { Meta, colorRamp, createColorScheme, ThemeAppearance } from "../common"

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

export const meta: Meta = {
    name,
    author: "Ethan Schoonover",
    license: {
        SPDX: "MIT",
    },
    url: "https://github.com/altercation/solarized",
}

export const dark = createColorScheme({
    name: `${name} Dark`,
    author: meta.author,
    appearance: ThemeAppearance.Dark,
    inputColor: ramps,
    override: { syntax: {} },
})

export const light = createColorScheme({
    name: `${name} Light`,
    author: meta.author,
    appearance: ThemeAppearance.Light,
    inputColor: ramps,
    override: { syntax: {} },
})
