import {
    ElementIntensities,
    Intensity,
    addToElementIntensities,
    useElementIntensities,
} from "@theme/intensity"
import { ContainerStyle } from "@theme/container"
import { Theme } from "@theme/config"
import { useColors } from "@theme/colors"
import { border } from "@theme/border"
import { buildSurfaceTokens } from "./tokens"

export type SurfaceLevel = 0 | 1

export function surfaceStyle(
    theme: Theme,
    level: SurfaceLevel,
    intensity: ElementIntensities
): Partial<ContainerStyle> {
    const color = useColors(theme)

    const resolvedIntensity = useElementIntensities(theme, intensity)

    // SInce we've already resolved the theme's appearance intensity above we can direclty cast ElementIntensities to <Intensity>
    let surfaceIntensity: ElementIntensities<Intensity>

    if (level === 0) {
        surfaceIntensity = addToElementIntensities(resolvedIntensity, 10)
    } else surfaceIntensity = resolvedIntensity

    return {
        background: color.neutral(surfaceIntensity.bg),
        border: border(theme, intensity),
    }
}

export function buildSurfaceLevels(theme: Theme) {
    const SURFACE_INTENSITY: ElementIntensities = {
        bg: 1,
        border: 12,
        fg: 100,
    }

    const surfaceIntensities = useElementIntensities(theme, SURFACE_INTENSITY)

    const surface = {
        background: surfaceStyle(theme, 0, surfaceIntensities),
        panel: surfaceStyle(theme, 1, surfaceIntensities),
        pane: surfaceStyle(theme, 1, surfaceIntensities),
    }

    return surface
}

export function buildSurfaces(theme: Theme) {
    const surfaces = buildSurfaceLevels(theme)
    buildSurfaceTokens(theme)

    return surfaces
}
