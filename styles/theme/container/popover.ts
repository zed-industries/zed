import { useSurfaceIntensity, useSurfaceStyle } from "@components/surface"
import { Theme, useColors } from "@theme*"
import { ContainerOptions, ContainerStyle, containerStyle } from "."
import { shadow } from "@theme/shadow"
import { ThemeColor } from "@theme/config"
import { useElementIntensities } from "@theme/intensity"

interface PopoverContainerProps {
    theme: Theme
    color?: ThemeColor
    options?: ContainerOptions
}

export function popoverContainerStyle({
    theme,
    color,
    options,
}: PopoverContainerProps): ContainerStyle {
    const themeColor = useColors(theme)
    const surfaceStyle = useSurfaceStyle(theme, "popover")
    const surfaceIntensity = useSurfaceIntensity(theme, "popover")
    const resolvedIntensities = useElementIntensities(theme, surfaceIntensity)

    let background
    let border

    if (color) {
        background = themeColor[color](resolvedIntensities.bg)
        border = {
            ...surfaceStyle.border,
            color: themeColor[color](resolvedIntensities.border),
        }
    } else {
        background = surfaceStyle.background
        border = surfaceStyle.border
    }

    const container: ContainerStyle = containerStyle({
        ...options,
        background,
        border,
        shadow: shadow(theme, "popover"),
    })

    return container
}
