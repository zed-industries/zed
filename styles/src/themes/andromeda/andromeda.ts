import chroma from "chroma-js"
import { Meta } from "../common/colorScheme"
import { colorRamp, createColorScheme } from "../common/ramps"

const name = "Andromeda"

const ramps = {
    neutral: chroma
        .scale([
            "#1E2025",
            "#23262E",
            "#292E38",
            "#2E323C",
            "#ACA8AE",
            "#CBC9CF",
            "#E1DDE4",
            "#F7F7F8",
        ])
        .domain([0, 0.15, 0.25, 0.35, 0.7, 0.8, 0.9, 1]),
    red: colorRamp(chroma("#F92672")),
    orange: colorRamp(chroma("#F39C12")),
    yellow: colorRamp(chroma("#FFE66D")),
    green: colorRamp(chroma("#96E072")),
    cyan: colorRamp(chroma("#00E8C6")),
    blue: colorRamp(chroma("#0CA793")),
    violet: colorRamp(chroma("#8A3FA6")),
    magenta: colorRamp(chroma("#C74DED")),
}

export const dark = createColorScheme(name, false, ramps)

export const meta: Meta = {
    name,
    author: "EliverLara",
    license: {
        SPDX: "MIT",
        license_text: {
            https_url:
                "https://raw.githubusercontent.com/EliverLara/Andromeda/master/LICENSE.md",
            license_checksum:
                "2f7886f1a05cefc2c26f5e49de1a39fa4466413c1ccb06fc80960e73f5ed4b89",
        },
    },
    url: "https://github.com/EliverLara/Andromeda",
}
