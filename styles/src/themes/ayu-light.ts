import { createColorScheme } from "./common/ramps"
import { ayu, buildTheme } from "./common/ayu-common"

const name = "Ayu"
const author = "Konstantin Pschera <me@kons.ch>"
const url = "https://github.com/ayu-theme/ayu-colors"
const license = {
    type: "MIT",
    url: "https://github.com/ayu-theme/ayu-colors/blob/master/license",
}

const variant = ayu.light
const theme = buildTheme(variant, true)

export const light = createColorScheme(
    `${name} Light`,
    true,
    theme.ramps,
    theme.syntax
)
