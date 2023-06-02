import { createColorScheme, ThemeAppearance } from "../common"
import { ayu, meta as themeMeta, buildTheme } from "./common"

export const meta = {
    ...themeMeta,
    name: `${themeMeta.name} Dark`,
}

const variant = ayu.dark
const theme = buildTheme(variant, false)

export const dark = createColorScheme({
    name: meta.name,
    author: meta.author,
    appearance: ThemeAppearance.Dark,
    inputColor: theme.ramps,
    override: { syntax: theme.syntax },
})
