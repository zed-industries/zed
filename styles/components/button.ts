import { Intensity, Theme, useColors } from "@/theme"
import {
    BorderRadius,
    ContainedIcon,
    IconSize,
    InteractiveContainer,
    StateIntensities,
    buildStateIntensities,
    checkContrast,
} from "@/theme/container"
import { TokenFamily, tokens } from "@/theme/tokens"

/**
 * Single intensity = same for light and dark
 *
 * Array = [dark intensity, light intensity]
 */
type ElementIntensity = Intensity | [Intensity, Intensity]

interface ElementIntensities<T = ElementIntensity> {
    bg: T
    border: T
    fg: T
}

interface BuildButtonProperties {
    theme: Theme
    inputIntensity: ElementIntensities
}

/** Resolves ElementIntensity down to a single Intensity based on the theme's appearance
 *
 * If two intensities are provided, the first is used for dark appearance and the second for light appearance
 *
 * If one intensity is provided, it is used for both dark and light appearance
 */
function useElementIntensities(
    theme: Theme,
    intensity: ElementIntensities<ElementIntensity>
): ElementIntensities<Intensity> {
    if (Array.isArray(intensity)) {
        return {
            bg: theme.appearance === "light" ? intensity[1] : intensity[0],
            border: theme.appearance === "light" ? intensity[1] : intensity[0],
            fg: theme.appearance === "light" ? intensity[1] : intensity[0],
        }
    } else {
        return {
            bg: intensity.bg as Intensity,
            border: intensity.border as Intensity,
            fg: intensity.fg as Intensity,
        }
    }
}

function buttonWithIconStyle({
    theme,
    inputIntensity,
}: BuildButtonProperties): InteractiveContainer<ContainedIcon> {
    const color = useColors(theme)
    const intensity = useElementIntensities(theme, inputIntensity)

    const bgIntensities = buildStateIntensities(
        theme,
        intensity.bg,
        theme.intensity.scaleFactor
    )
    const borderIntensities = buildStateIntensities(
        theme,
        theme.appearance === "light" ? 36 : 24,
        theme.intensity.scaleFactor
    )
    const fgIntensities = buildStateIntensities(
        theme,
        intensity.fg,
        theme.intensity.scaleFactor
    )

    checkContrast(
        "buttonWithIconStyle",
        bgIntensities.default,
        fgIntensities.default
    )

    const button = (state: keyof StateIntensities): ContainedIcon => {
        // Create tokens for design system
        // TODO: This should become a generic function for adding elements to the tokens
        const buttonTokens: TokenFamily = {
            [state]: {
                background: tokens.colorToken(
                    color.neutral(bgIntensities[state])
                ),
                border: tokens.colorToken(
                    color.neutral(borderIntensities[state])
                ),
                foreground: tokens.colorToken(
                    color.neutral(fgIntensities[state])
                ),
            },
        }

        // Push tokens into the global token object
        tokens.addToToken("button", {
            ...buttonTokens,
        })

        return {
            container: {
                background: color.neutral(bgIntensities[state]),
                margin: [0, 0, 0, 0],
                padding: [4, 4, 4, 4],
                borderRadius: BorderRadius.Medium,
                border: {
                    width: 1,
                    color: color.neutral(borderIntensities[state]),
                    style: "solid",
                    inset: false,
                },
                width: 15,
                height: 15,
            },
            icon: {
                color: color.neutral(fgIntensities[state]),
                size: IconSize.Medium,
            },
        }
    }

    // TODO: There should be a function for creating a disabled state
    return {
        default: button("default"),
        hovered: button("hovered"),
        pressed: button("pressed"),
    }
}

export function iconButton(theme: Theme): InteractiveContainer<ContainedIcon> {
    return buttonWithIconStyle({
        theme: theme,
        inputIntensity: {
            bg: 12,
            border: [36, 24],
            fg: 100,
        },
    })
}
