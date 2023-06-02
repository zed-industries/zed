import { createColorScheme, ThemeAppearance } from "../common"
import { ayu, meta as themeMeta, buildTheme } from "./common"

export const meta = {
    ...themeMeta,
    name: `${themeMeta.name} Light`,
}

const variant = ayu.light
const theme = buildTheme(variant, true)

export const light = createColorScheme({
    name: meta.name,
    author: meta.author,
    appearance: ThemeAppearance.Light,
    inputColor: theme.ramps,
    override: { syntax: theme.syntax },
})
