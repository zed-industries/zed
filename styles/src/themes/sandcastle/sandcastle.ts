import chroma from "chroma-js"
import { Meta } from "../common/colorScheme"
import { colorRamp, createColorScheme } from "../common/ramps"

const name = "Sandcastle"

const ramps = {
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
}

export const dark = createColorScheme(name, false, ramps)

export const meta: Meta = {
    name,
    author: "gessig",
    license: {
        SPDX: "MIT",
    },
    url: "https://github.com/gessig/base16-sandcastle-scheme",
}
