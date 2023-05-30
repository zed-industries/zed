import { createColorScheme } from "../common/ramps"
import { ayu, meta as themeMeta, buildTheme } from "./common"

export const meta = {
    ...themeMeta,
    name: `${themeMeta.name} Mirage`
}

const variant = ayu.mirage
const theme = buildTheme(variant, false)

export const dark = createColorScheme(
    meta.name,
    false,
    theme.ramps,
    theme.syntax
)
