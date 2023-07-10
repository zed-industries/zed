import { Theme, StyleSets } from "../common"
import { interactive } from "../element"
import { InteractiveState } from "../element/interactive"
import { background, foreground } from "../style_tree/components"

interface TabBarButtonOptions {
    icon: string
    color?: StyleSets
}

type TabBarButtonProps = TabBarButtonOptions & {
    state?: Partial<Record<InteractiveState, Partial<TabBarButtonOptions>>>
}

export function tab_bar_button(theme: Theme, { icon, color = "base" }: TabBarButtonProps) {
    const button_spacing = 8

    return (
        interactive({
            base: {
                icon: {
                    color: foreground(theme.middle, color),
                    asset: icon,
                    dimensions: {
                        width: 15,
                        height: 15,
                    },
                },
                container: {
                    corner_radius: 4,
                    padding: {
                        top: 4, bottom: 4, left: 4, right: 4
                    },
                    margin: {
                        left: button_spacing / 2,
                        right: button_spacing / 2,
                    },
                },
            },
            state: {
                hovered: {
                    container: {
                        background: background(theme.middle, color, "hovered"),

                    }
                },
                clicked: {
                    container: {
                        background: background(theme.middle, color, "pressed"),
                    }
                },
            },
        })
    )
}
