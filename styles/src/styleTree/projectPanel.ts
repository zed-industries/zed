import { ColorScheme } from "../theme/colorScheme"
import { withOpacity } from "../theme/color"
import { Border, TextStyle, background, border, foreground, text } from "./components"
import { interactive, toggleable } from "../element"
import merge from "ts-deepmerge"
export default function projectPanel(colorScheme: ColorScheme) {
    const { isLight } = colorScheme

    let layer = colorScheme.middle

    type EntryStateProps = {
        background?: string,
        border?: Border,
        text?: TextStyle,
        iconColor?: string,
    }

    type EntryState = {
        default: EntryStateProps,
        hovered?: EntryStateProps,
        clicked?: EntryStateProps,
    }

    const entry = (unselected?: EntryState, selected?: EntryState) => {

        const git_status = {
            git: {
                modified: isLight
                    ? colorScheme.ramps.yellow(0.6).hex()
                    : colorScheme.ramps.yellow(0.5).hex(),
                inserted: isLight
                    ? colorScheme.ramps.green(0.45).hex()
                    : colorScheme.ramps.green(0.5).hex(),
                conflict: isLight
                    ? colorScheme.ramps.red(0.6).hex()
                    : colorScheme.ramps.red(0.5).hex(),
            },
        }

        const base_properties = {
            height: 22,
            background: background(layer),
            iconColor: foreground(layer, "variant"),
            iconSize: 7,
            iconSpacing: 5,
            text: text(layer, "mono", "variant", { size: "sm" }),
            status: {
                ...git_status
            }
        }

        const selectedStyle: EntryState | undefined = selected ? selected : unselected

        const unselected_default_style = merge(base_properties, unselected?.default ?? {}, {})
        const unselected_hovered_style = merge(base_properties, unselected?.hovered ?? {}, { background: background(layer, "variant", "hovered") })
        const unselected_clicked_style = merge(base_properties, unselected?.clicked ?? {}, { background: background(layer, "variant", "pressed") })
        const selected_default_style = merge(base_properties, selectedStyle?.default ?? {}, { background: background(layer) })
        const selected_hovered_style = merge(base_properties, selectedStyle?.hovered ?? {}, { background: background(layer, "variant", "hovered") })
        const selected_clicked_style = merge(base_properties, selectedStyle?.clicked ?? {}, { background: background(layer, "variant", "pressed") })

        return toggleable({
            state: {
                inactive: interactive({
                    state: {
                        default: unselected_default_style,
                        hovered: unselected_hovered_style,
                        clicked: unselected_clicked_style,
                    },
                }),
                active: interactive({
                    state: {
                        default: selected_default_style,
                        hovered: selected_hovered_style,
                        clicked: selected_clicked_style,
                    },
                }),
            }
        })

    }

    const defaultEntry = entry()

    return {
        openProjectButton: interactive({
            base: {
                background: background(layer),
                border: border(layer, "active"),
                cornerRadius: 4,
                margin: {
                    top: 16,
                    left: 16,
                    right: 16,
                },
                padding: {
                    top: 3,
                    bottom: 3,
                    left: 7,
                    right: 7,
                },
                ...text(layer, "sans", "default", { size: "sm" }),
            },
            state: {
                hovered: {
                    ...text(layer, "sans", "default", { size: "sm" }),
                    background: background(layer, "hovered"),
                    border: border(layer, "active"),
                },
                clicked: {
                    ...text(layer, "sans", "default", { size: "sm" }),
                    background: background(layer, "pressed"),
                    border: border(layer, "active"),
                },
            },
        }),
        background: background(layer),
        padding: { left: 6, right: 6, top: 0, bottom: 6 },
        indentWidth: 12,
        defaultEntry,
        draggedEntry: entry({
            default: {
                text: text(layer, "mono", "on", { size: "sm" }),
                background: withOpacity(background(layer, "on"), 0.9),
                border: border(layer),
            }
        }),
        ignoredEntry: entry({
            default: {
                text: text(layer, "mono", "disabled"),
            },
        }, {
            default: {
                iconColor: foreground(layer, "variant"),
            }
        }),
        cutEntry: entry({
            default: {
                text: text(layer, "mono", "disabled"),
            },
        }, {
            default: {
                background: background(layer, "active"),
                text: text(layer, "mono", "disabled", { size: "sm" }),
            }
        }),
        filenameEditor: {
            background: background(layer, "on"),
            text: text(layer, "mono", "on", { size: "sm" }),
            selection: colorScheme.players[0],
        },
    }
}
