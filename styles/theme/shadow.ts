import { Surface, useSurfaceIntensity } from "@components/surface"
import { Theme } from "./config"

export interface Shadow {
    blur: number
    color: string
    offset: number[]
}

export function shadow(theme: Theme, surface: Surface): Shadow {
    const DEFAULT_SHADOW_BLUR = 4 as const

    const intensity = useSurfaceIntensity(theme, surface)

    let shadowAlpha: number
    let offsetX: number
    let offsetY: number

    switch (surface) {
        case "popover":
            shadowAlpha = 0.12 as const
            offsetX = 1
            offsetY = 2
            break
        default:
            shadowAlpha = 0.12 as const
            offsetX = 1
            offsetY = 4
            break
    }

    const blur = DEFAULT_SHADOW_BLUR
    const color = theme.color.neutral[intensity.bg * shadowAlpha]
    const offset = [offsetX, offsetY]

    return {
        blur,
        color,
        offset,
    }
}
