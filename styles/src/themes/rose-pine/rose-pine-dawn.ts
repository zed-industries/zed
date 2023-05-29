import chroma from "chroma-js"
import { Meta } from "../common/colorScheme"
import { colorRamp, createColorScheme } from "../common/ramps"

const name = "Rosé Pine Dawn"

const ramps = {
    neutral: chroma
        .scale([
            "#575279",
            "#797593",
            "#9893A5",
            "#B5AFB8",
            "#D3CCCC",
            "#F2E9E1",
            "#FFFAF3",
            "#FAF4ED",
        ])
        .domain([0, 0.35, 0.45, 0.65, 0.7, 0.8, 0.9, 1]),
    red: colorRamp(chroma("#B4637A")),
    orange: colorRamp(chroma("#D7827E")),
    yellow: colorRamp(chroma("#EA9D34")),
    green: colorRamp(chroma("#679967")),
    cyan: colorRamp(chroma("#286983")),
    blue: colorRamp(chroma("#56949F")),
    violet: colorRamp(chroma("#907AA9")),
    magenta: colorRamp(chroma("#79549F")),
}

export const light = createColorScheme(name, true, ramps)

export const meta: Meta = {
    name,
    author: "edunfelt",
    license: {
        SPDX: "MIT",
        license_text: {
            https_url:
                "https://raw.githubusercontent.com/edunfelt/base16-rose-pine-scheme/main/LICENSE",
            license_checksum:
                "6ca1b9da8c78c8441c5aa43d024a4e4a7bf59d1ecca1480196e94fda0f91ee4a",
        },
    },
    url: "https://github.com/edunfelt/base16-rose-pine-scheme",
}
