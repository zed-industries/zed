import { createColorScheme } from "../common/ramps"
import { ayu, meta as themeMeta, buildTheme } from "./common"

export const meta = {
    ...themeMeta,
    name: `${themeMeta.name} Light`
}

const variant = ayu.light
const theme = buildTheme(variant, true)

export const light = createColorScheme(
    meta.name,
    true,
    theme.ramps,
    theme.syntax
)
