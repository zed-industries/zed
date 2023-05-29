import chroma from "chroma-js"
import { Meta } from "../common/colorScheme"
import { colorRamp, createColorScheme } from "../common/ramps"

const name = "Rosé Pine Moon"

const ramps = {
    neutral: chroma
        .scale([
            "#232136",
            "#2A273F",
            "#393552",
            "#3E3A53",
            "#56526C",
            "#6E6A86",
            "#908CAA",
            "#E0DEF4",
        ])
        .domain([0, 0.3, 0.55, 1]),
    red: colorRamp(chroma("#EB6F92")),
    orange: colorRamp(chroma("#EBBCBA")),
    yellow: colorRamp(chroma("#F6C177")),
    green: colorRamp(chroma("#8DBD8D")),
    cyan: colorRamp(chroma("#409BBE")),
    blue: colorRamp(chroma("#9CCFD8")),
    violet: colorRamp(chroma("#C4A7E7")),
    magenta: colorRamp(chroma("#AB6FE9")),
}

export const dark = createColorScheme(name, false, ramps)

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
