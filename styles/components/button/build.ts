import { Theme, useColors } from "@/theme"
import { border } from "@theme/border"
import {
    BorderRadius,
    ContainedIcon,
    ContainedText,
    ContainerStyle,
    IconSize,
    IconStyle,
    InteractiveContainer,
    StateIntensity,
    buildIntensitiesForStates,
} from "@theme/container"
import { TextStyle } from "@theme/text"
import { ElementIntensities, useElementIntensities } from "@theme/intensity"
import { margin, padding } from "@theme/properties"
import { text as textStyle } from "@theme/text"

type ButtonSizes = "small" | "medium" | "large"
type ButtonSize = (typeof buttonSize)[keyof typeof buttonSize]

const buttonSize: Record<ButtonSizes, number> = {
    small: 15,
    medium: 21,
    large: 25,
}

const DEFAULT_BUTTON_INTENSITIES: ElementIntensities = {
    bg: 5,
    border: 8,
    fg: 100,
}

interface ButtonProps {
    theme: Theme
    /** A unique name for the button
     *
     *  Used for debugging & contrast validation  */
    name: string
    kind: ButtonKind
    intensities?: ElementIntensities
    size?: ButtonSize
}

type Button =
    | InteractiveContainer<ContainedIcon>
    | InteractiveContainer<ContainedText>

type ButtonKind = "icon" | "label"

export function buildButton({
    theme,
    name,
    kind = "label",
    intensities = DEFAULT_BUTTON_INTENSITIES,
    size = buttonSize.medium,
}: ButtonProps): Button {
    const color = useColors(theme)
    const resolvedIntensities = useElementIntensities(theme, intensities)

    let container: ContainerStyle = {
        background: color.neutral(resolvedIntensities.bg),
        margin: margin(0, 0, 0, 0),
        padding: padding(0, 0, 0, 0),
        borderRadius: BorderRadius.Medium,
        border: border(theme, resolvedIntensities.border),
        width: "auto",
        height: size,
    }

    let icon: IconStyle = {
        color: color.neutral(resolvedIntensities.fg),
        size: IconSize.Medium,
    }

    let text: TextStyle = textStyle(theme, resolvedIntensities.fg)

    const states = buildIntensitiesForStates(theme, name, resolvedIntensities)

    const buildStates = (intensities: StateIntensity) => {
        container.background = color.neutral(intensities.bg)
        container.border = border(theme, intensities.border)
        icon.color = color.neutral(intensities.fg)
        text.color = color.neutral(intensities.fg)

        let stateStyle

        switch (kind) {
            case "icon":
                stateStyle = {
                    container,
                    icon,
                }

                return stateStyle as ContainedIcon
            case "label":
                stateStyle = {
                    container,
                    text,
                }
                return stateStyle as ContainedText
            default:
                throw new Error("Unhandled button kind")
        }
    }

    let button = {
        default: buildStates(states.default),
        hovered: buildStates(states.hovered),
        pressed: buildStates(states.pressed)
    }

    switch (kind) {
        case "icon":
            return button as InteractiveContainer<ContainedIcon>
        case "label":
            return button as InteractiveContainer<ContainedText>
        default:
            throw new Error("Unhandled button kind")
    }
}
