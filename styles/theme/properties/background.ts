import { Intensity, Theme, useColors } from "@theme"
import { ThemeColor } from "@theme/config"
import { resolveThemeColorIntensity } from "@theme/intensity"

/** Get a background color from the theme.
*
* Takes an intensity, and optionally a color
*
* If no color is specified, neutral is used
*/
export function background(
    theme: Theme,
    intensity: Intensity,
    color?: ThemeColor
): string {
    const themeColor = useColors(theme)

    const resolvedColorIntensity = resolveThemeColorIntensity(theme, intensity)

    if (!color) {
        return themeColor.neutral(resolvedColorIntensity)
    }

    return themeColor[color](resolvedColorIntensity)
}
