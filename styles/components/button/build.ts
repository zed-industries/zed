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
    buildStates,
    checkContrast,
} from "@theme/container"
import { TextStyle, useText } from "@theme/font"
import { ElementIntensities, useElementIntensities } from "@theme/intensity"

type ButtonSizes = "small" | "medium" | "large"
type ButtonSize = (typeof buttonSize)[keyof typeof buttonSize]

const buttonSize: Record<ButtonSizes, number> = {
    small: 15,
    medium: 21,
    large: 25,
}

const DEFAULT_BUTTON_INTENSITIES: ElementIntensities = {
    bg: 1,
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
    const intensity = useElementIntensities(theme, intensities)

    const states = buildStates(theme, intensity)

    let container: ContainerStyle = {
        background: color.neutral(states.default.bg),
        margin: [0, 0, 0, 0],
        padding: [4, 4, 4, 4],
        borderRadius: BorderRadius.Medium,
        border: border(theme, states.default.border),
        width: "auto",
        height: size,
    }

    let icon: IconStyle = {
        color: color.neutral(states.default.fg),
        size: IconSize.Medium,
    }

    let text: TextStyle = useText(color.neutral(states.default.fg))

    const buildStateStyle = (state: StateIntensity) => {
        container.background = color.neutral(state.bg)
        container.border = border(theme, state.border)
        icon.color = color.neutral(state.fg)
        text.color = color.neutral(state.fg)

        checkContrast(`${name}:${state}`, state.bg, state.fg)

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
        default: buildStateStyle(states.default),
        hovered: buildStateStyle(states.hovered),
        pressed: buildStateStyle(states.pressed)
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
