import { Theme } from "@/theme"
import {
    ContainedIcon,
    InteractiveContainer,
    InteractiveToggleableContainer,
} from "@/theme/container"
import { buttonWithIconStyle, iconButton } from "@components/button"

// Use ContainedIcon just as an example
export function toggleButton(
    theme: Theme
): InteractiveToggleableContainer<InteractiveContainer<ContainedIcon>> {
    const inactive = iconButton(theme)
    const active = buttonWithIconStyle({
        theme,
        inputIntensity: {
            bg: 32,
            border: [48, 36],
            fg: 100,
        },
    })

    return {
        inactive,
        active,
    }
}
