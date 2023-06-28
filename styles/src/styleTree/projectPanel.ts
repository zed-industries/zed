import { ColorScheme } from "../theme/colorScheme"
import { withOpacity } from "../theme/color"
import { Border, TextProperties, background, border, foreground, text } from "./components"
import { interactive, toggleable } from "../element"
import { InteractiveState } from "../element/interactive"
export default function projectPanel(colorScheme: ColorScheme) {
    const { isLight } = colorScheme

    let layer = colorScheme.middle

    const default_entry = interactive({
        base: {
            ...baseEntry,
            status,
        },
        state: {
            default: {
                background: background(layer),
            },
            hovered: {
                background: background(layer, "variant", "hovered"),
            },
            clicked: {
                background: background(layer, "variant", "pressed"),
            },
        },
    })

    let base_entry = toggleable({
        base: default_entry,
        state: {
            active: interactive({
                base: {
                    ...default_entry,
                },
                state: {
                    default: {
                        background: background(colorScheme.lowest),
                    },
                    hovered: {
                        background: background(colorScheme.lowest, "hovered"),
                    },
                    clicked: {
                        background: background(colorScheme.lowest, "pressed"),
                    },
                },
            }),
        },
    })

    type EntryStateProps = {
        background?: string,
        border: Border,
        text: TextProperties,
        iconColor: string,
    }

    type EntryState = Record<Partial<InteractiveState>, EntryStateProps>

    const entry = (base: object, unselected: EntryState, selected: EntryState) => {
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
            iconColor: foreground(layer, "variant"),
            iconSize: 7,
            iconSpacing: 5,
            text: text(layer, "mono", "variant", { size: "sm" }),
            status: {
                ...git_status
            }
        }

        const unselected_i = interactive({
            base: base_properties,
            state: {
                default: {
                    background: background(layer),
                    ...unselected.default ?? {},
                },
                hovered: {
                    background: background(layer, "variant", "hovered"),
                    ...unselected.hovered ?? {},
                },
                clicked: {
                    background: background(layer, "variant", "pressed"),
                    ...unselected.clicked ?? {},
                },
            },
        })

        const selected_i = interactive({
            base: base,
            state: {
                default: {
                    ...base_entry,
                    ...(selected.default ?? {}),
            },
            hovered: {
                ...base_entry,
                ...selected.hovered ?? {},
            },
            clicked: {
                ...base_entry,
                ...selected.clicked ?? {},
            },
            }
        })

        return toggleable({
            state: {
            inactive: unselected_i,
            active: selected_i,
            }
        })

    }

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
        entry,
        draggedEntry: {
            ...baseEntry,
            text: text(layer, "mono", "on", { size: "sm" }),
            background: withOpacity(background(layer, "on"), 0.9),
            border: border(layer),
            status,
        },
        ignoredEntry: {
            ...entry,
            iconColor: foreground(layer, "disabled"),
            text: text(layer, "mono", "disabled"),
            active: {
                ...entry.active,
                iconColor: foreground(layer, "variant"),
            },
        },
        cutEntry: {
            ...entry,
            text: text(layer, "mono", "disabled"),
            active: {
                ...entry.active,
                default: {
                    ...entry.active.default,
                    background: background(layer, "active"),
                    text: text(layer, "mono", "disabled", { size: "sm" }),
                },
            },
        },
        filenameEditor: {
            background: background(layer, "on"),
            text: text(layer, "mono", "on", { size: "sm" }),
            selection: colorScheme.players[0],
        },
    }
}
