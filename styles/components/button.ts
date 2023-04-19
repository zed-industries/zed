import { Theme, useColors } from "@/theme"
import {
    BorderRadius,
    ContainedIcon,
    IconSize,
    InteractiveContainer,
    StateIntensities,
    buildStateIntensities,
    checkContrast,
} from "@theme/container"
import { TokenFamily, tokens } from "@theme/tokens"
import { ElementIntensities, useElementIntensities } from "@theme/intensity"

interface BuildButtonProperties {
    theme: Theme
    inputIntensity: ElementIntensities
}

export function buttonWithIconStyle({
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
        intensity.border,
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
                    color.neutral(bgIntensities[state]),
                    `button.${state}.background: ${bgIntensities[state]} Intensity`
                ),
                border: tokens.colorToken(
                    color.neutral(borderIntensities[state]),
                    `button.${state}.border: ${borderIntensities[state]} Intensity`
                ),
                foreground: tokens.colorToken(
                    color.neutral(fgIntensities[state]),
                    `button.${state}.foreground: ${fgIntensities[state]} Intensity`
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
            bg: 1,
            border: 8,
            fg: 100,
        },
    })
}
