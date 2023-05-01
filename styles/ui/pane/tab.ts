import { iconButton } from "@components/button"
import { Button } from "@components/button/build"
import { useSurfaceIntensity } from "@components/surface"
import { Theme } from "@theme*"
import { border } from "@theme/border"
import { ContainedIcon, ContainedText, ContainerStyle, Interactive, State, buildIntensitiesForStates, container } from "@theme/container"
import { FlexStyle, flex } from "@theme/element/flex"
import { IconStyle, iconStyle } from "@theme/icon"
import { addToElementIntensities, useElementIntensities } from "@theme/intensity"
import { padding } from "@theme/properties"
import { background } from "@theme/properties/background"
import { TextStyle, useText } from "@theme/text"

interface TabProps {
    theme: Theme
    active?: boolean
    state: State
}

interface Indicators {
    dirty: IconStyle
    conflict: IconStyle
}

interface Tab {
    flex: FlexStyle;
    container: ContainerStyle;
    // Indicates the type of tab, e.g. "Project Search", "Feedback"
    icon: IconStyle;
    // Indicates the status of the tab, e.g. "Dirty", "Conflict"
    indicator: Indicators;
    label: TextStyle;
    // When two tabs of the same name are open, a description appears next to the label
    description: ContainedText;
    close: Button<ContainedIcon>;
}

function tabState({
    theme,
    active = false,
    state,
}: TabProps): Tab {
    const name = active ? "active_tab" : "tab"
    const TAB_HEIGHT = 32

    const intensities =
        active
            ? useSurfaceIntensity(theme, "pane")
            : addToElementIntensities(useSurfaceIntensity(theme, "pane"), 20)

    const resolvedIntensities = useElementIntensities(theme, intensities)

    const interactiveIntensities = buildIntensitiesForStates(theme, name, resolvedIntensities)


    const containerStyle = (state: State): ContainerStyle => {
        return {
            width: "auto",
            height: TAB_HEIGHT,
            background: background(theme, interactiveIntensities[state].bg),
            border: border(theme, interactiveIntensities[state].border),
            padding: padding(0, 12, 0, 8)
        }
    }

    const text = useText(theme, {
        intensity: 70,
    })

    return {
        container: containerStyle(state),
        flex: flex(8, {
            alignItems: "center",
        }),
        icon: iconStyle({
            theme,
            size: 'md',
            intensity: 70
        }),
        indicator: {
            dirty: iconStyle({
                theme,
                size: 'sm',
                color: 'accent'

            }),
            conflict: iconStyle({
                theme,
                size: 'sm',
                color: 'warning'
            })
        },
        label: text,
        description: {
            container: {
                ...container.blank,
            },
            text: useText(theme, {
                intensity: 50,
            }),
        },
        close: iconButton(theme)
    }
}

export function activeTab(theme: Theme): Interactive<Tab> {
    return {
        default: tabState({
            theme,
            active: false,
            state: "default"
        }),
        hovered: tabState({
            theme,
            active: false,
            state: "hovered"
        }),
        pressed: tabState({
            theme,
            active: false,
            state: "pressed"
        })
    }
}

export function inactiveTab(theme: Theme): Interactive<Tab> {
    return {
        default: tabState({
            theme,
            active: false,
            state: "default"
        }),
        hovered: tabState({
            theme,
            active: false,
            state: "hovered"
        }),
        pressed: tabState({
            theme,
            active: false,
            state: "pressed"
        })
    }
}
