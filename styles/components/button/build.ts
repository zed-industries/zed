import { Theme, useColors } from "@/theme"
import { border } from "@theme/border"
import {
    BorderRadius,
    ContainedIcon,
    ContainedText,
    ContainerStyle,
    Interactive,
    StateIntensity,
    buildIntensitiesForStates,
} from "@theme/container"
import { TextStyle } from "@theme/text"
import { ElementIntensities, useElementIntensities } from "@theme/intensity"
import { margin, padding } from "@theme/properties"
import { textStyle } from "@theme/text"
import { iconStyle } from "@theme/icon"

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

export type Button<T = ContainedIcon | ContainedText> = Interactive<T>

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
        padding: padding(6, 4),
        borderRadius: BorderRadius.Medium,
        border: border(theme, resolvedIntensities.border),
        width: "auto",
        height: size,
    }

    const icon = iconStyle({
        theme,
        intensity: resolvedIntensities.fg,
        size: 'md'
    })

    let text: TextStyle = textStyle(theme,
        { intensity: resolvedIntensities.fg })

    const states = buildIntensitiesForStates(theme, name, resolvedIntensities)

    const buildStates = (intensities: StateIntensity) => {
        let updatedContainer = {
            ...container,
            background: color.neutral(intensities.bg),
            border: border(theme, intensities.border),
        };

        let updatedIcon = {
            ...icon,
            color: color.neutral(intensities.fg),
        };

        let updatedText = {
            ...text,
            color: color.neutral(intensities.fg),
        };

        let stateStyle;

        switch (kind) {
            case "icon":
                stateStyle = {
                    container: updatedContainer,
                    icon: updatedIcon,
                };

                return stateStyle as ContainedIcon;
            case "label":
                stateStyle = {
                    container: updatedContainer,
                    text: updatedText,
                };
                return stateStyle as ContainedText;
            default:
                throw new Error("Unhandled button kind");
        }
    };

    let button = {
        default: buildStates(states.default),
        hovered: buildStates(states.hovered),
        pressed: buildStates(states.pressed)
    }

    switch (kind) {
        case "icon":
            return button as Button<ContainedIcon>
        case "label":
            return button as Button<ContainedText>
        default:
            throw new Error("Unhandled button kind")
    }
}
