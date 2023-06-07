import { ThemeAppearance, ThemeConfig } from "../../common"
import { ayu, meta, buildTheme } from "./common"

const variant = ayu.mirage
const { ramps, syntax } = buildTheme(variant, false)

export const theme: ThemeConfig = {
    name: `${meta.name} Mirage`,
    author: meta.author,
    appearance: ThemeAppearance.Dark,
    licenseType: meta.licenseType,
    licenseUrl: meta.licenseUrl,
    licenseFile: `${__dirname}/LICENSE`,
    inputColor: ramps,
    override: { syntax },
}
