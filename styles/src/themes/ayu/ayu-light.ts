import { ThemeAppearance, ThemeConfig } from "../../common"
import { ayu, meta, build_theme } from "./common"

const variant = ayu.light
const { ramps, syntax } = build_theme(variant, true)

export const theme: ThemeConfig = {
    name: `${meta.name} Light`,
    author: meta.author,
    appearance: ThemeAppearance.Light,
    license_type: meta.license_type,
    license_url: meta.license_url,
    license_file: `${__dirname}/LICENSE`,
    input_color: ramps,
    override: { syntax },
}
