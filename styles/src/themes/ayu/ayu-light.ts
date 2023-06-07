import { ThemeAppearance, ThemeConfig } from "../../common"
import { ayu, meta, buildTheme } from "./common"

const variant = ayu.light
const { ramps, syntax } = buildTheme(variant, true)

export const theme: ThemeConfig = {
    name: `${meta.name} Light`,
    author: meta.author,
    appearance: ThemeAppearance.Light,
    licenseType: meta.licenseType,
    licenseUrl: meta.licenseUrl,
    licenseFile: `${__dirname}/LICENSE`,
    inputColor: ramps,
    override: { syntax },
}
