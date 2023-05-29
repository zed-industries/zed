import { createColorScheme } from "../common/ramps"
import { ayu, meta as themeMeta, buildTheme } from "../common/ayu-common"

export const meta = {
    ...themeMeta,
    name: `${themeMeta.name} Dark`
}

const variant = ayu.dark
const theme = buildTheme(variant, false)

export const dark = createColorScheme(
    meta.name,
    false,
    theme.ramps,
    theme.syntax
)
