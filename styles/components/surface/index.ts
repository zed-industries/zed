import {
    ElementIntensities,
    Intensity,
    addToElementIntensities,
    addToIntensity,
    useElementIntensities,
} from "@theme/intensity"
import { ContainerStyle } from "@theme/container"
import { Theme } from "@theme/config"
import { useColors } from "@theme/colors"
import { border } from "@theme/border"
import { buildSurfaceTokens } from "./tokens"

type SurfaceLevel = 0 | 1 | 2
type SurfaceName = "background" | "panel" | "pane" | "popover" | "palette" | "tooltip"

type SurfaceLevels = Record<SurfaceName, SurfaceLevel>

type Surface = keyof SurfaceLevels

const surfaceLevel: SurfaceLevels = {
    "background": 0,
    "panel": 1,
    "pane": 1,
    "popover": 2,
    "palette": 2,
    "tooltip": 2,
}

type SurfaceStyle = Partial<ContainerStyle>

function useSurfaceIntensity(theme: Theme, surface: Surface): ElementIntensities<Intensity> {
    const level = surfaceLevel[surface]

    const BASE_SURFACE_INTENSITIES: ElementIntensities<Intensity> = {
        bg: 1,
        border: 12,
        fg: 100,
    }

    const intensity = useElementIntensities(theme, BASE_SURFACE_INTENSITIES)

    switch (level) {
        case 1:
            return addToElementIntensities(intensity, 10)
        case 2:
            return addToElementIntensities(intensity, 20)
        default:
            return intensity
    }
}

function buildSurfaceStyle(theme: Theme, surface: Surface): SurfaceStyle {
    const color = useColors(theme)
    const intensity = useSurfaceIntensity(theme, surface)

    const borderIntensity = intensity.border as Intensity

    return {
        background: color.neutral(intensity.bg),
        border: border(theme, borderIntensity),
    }
}

function buildSurfaceLevels(theme: Theme) {
    const surface = {
        background: buildSurfaceStyle(theme, "background"),
        panel: buildSurfaceStyle(theme, "panel"),
        pane: buildSurfaceStyle(theme, "pane"),
        popover: buildSurfaceStyle(theme, "popover"),
    }

    return surface
}

const useSurfaceStyle = buildSurfaceStyle

const surface = (theme: Theme) => {
    buildSurfaceTokens(theme)

    return {
        level: surfaceLevel,
        style: buildSurfaceLevels(theme)
    }
}

// Placeholder for defining element background intensity relative to surface logic
// TODO: You should be able to specific adding or subtracting intensity
function relativeIntensityToSurface(surfaceIntensity: Intensity, intensityChange: Intensity): Intensity {
    // adjust background color based on the relative difference between surface intensity and intensityChange
    const newIntensity: Intensity = addToIntensity(surfaceIntensity, intensityChange)

    return newIntensity
}

export {
    Surface,
    SurfaceStyle,
    useSurfaceIntensity,
    useSurfaceStyle,
    relativeIntensityToSurface,
    surface,
}
