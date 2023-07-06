import { ThemeAppearance, ThemeConfig } from "../../common"
import { ayu, meta, build_theme } from "./common"

const variant = ayu.dark
const { ramps, syntax } = build_theme(variant, false)

export const theme: ThemeConfig = {
    name: `${meta.name} Dark`,
    author: meta.author,
    appearance: ThemeAppearance.Dark,
    license_type: meta.license_type,
    license_url: meta.license_url,
    license_file: `${__dirname}/LICENSE`,
    input_color: ramps,
    override: { syntax },
}
