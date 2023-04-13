import { Border, Theme, useColors } from "@/theme"

type Margin = [number, number, number, number]
type Padding = [number, number, number, number]

interface ContainerStyle {
    background: string
    margin: Margin
    padding: Padding
    border: Border
}

enum IconSize {
    "Small" = 7,
    "Medium" = 11,
    "Large" = 15,
}

interface IconStyle {
    color: string
    size: IconSize
}

interface ButtonWithIconStyle {
    container: ContainerStyle
    icon: IconStyle
}

export function buttonWithIconStyle(theme: Theme): ButtonWithIconStyle {
    const color = useColors(theme)

    return {
        container: {
            background: color.neutral(26),
            margin: [0, 0, 0, 0],
            padding: [4, 4, 4, 4],
            border: {
                width: 0,
                color: color.neutral(32),
                style: "solid",
                inset: false,
            },
        },
        icon: {
            color: color.neutral(100),
            size: IconSize.Medium,
        },
    }
}
