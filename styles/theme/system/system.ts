import chroma from "chroma-js"
import * as colorFamily from "@system/color"

const color = {
    neutral: chroma
        .scale(colorFamily.neutral.scale.values)
        .mode("lch")
        .colors(9),
    red: chroma.scale(colorFamily.red.scale.values).mode("lch").colors(9),
    sunset: chroma.scale(colorFamily.sunset.scale.values).mode("lch").colors(9),
    orange: chroma.scale(colorFamily.orange.scale.values).mode("lch").colors(9),
    amber: chroma.scale(colorFamily.amber.scale.values).mode("lch").colors(9),
    yellow: chroma.scale(colorFamily.yellow.scale.values).mode("lch").colors(9),
    lemon: chroma.scale(colorFamily.lemon.scale.values).mode("lch").colors(9),
    citron: chroma.scale(colorFamily.citron.scale.values).mode("lch").colors(9),
    lime: chroma.scale(colorFamily.lime.scale.values).mode("lch").colors(9),
    green: chroma.scale(colorFamily.green.scale.values).mode("lch").colors(9),
    mint: chroma.scale(colorFamily.mint.scale.values).mode("lch").colors(9),
    cyan: chroma.scale(colorFamily.cyan.scale.values).mode("lch").colors(9),
    sky: chroma.scale(colorFamily.sky.scale.values).mode("lch").colors(9),
    blue: chroma.scale(colorFamily.blue.scale.values).mode("lch").colors(9),
    indigo: chroma.scale(colorFamily.indigo.scale.values).mode("lch").colors(9),
    purple: chroma.scale(colorFamily.purple.scale.values).mode("lch").colors(9),
    pink: chroma.scale(colorFamily.pink.scale.values).mode("lch").colors(9),
    rose: chroma.scale(colorFamily.rose.scale.values).mode("lch").colors(9),
}

export { color }
