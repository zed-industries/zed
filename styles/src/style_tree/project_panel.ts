import { with_opacity } from "../theme/color"
import {
    Border,
    TextStyle,
    background,
    border,
    foreground,
    text,
} from "./components"
import { interactive, toggleable } from "../element"
import merge from "ts-deepmerge"
import { useTheme } from "../theme"
export default function project_panel(): any {
    const theme = useTheme()

    const { is_light } = theme

    type EntryStateProps = {
        background?: string
        border?: Border
        text?: TextStyle
        icon_color?: string
    }

    type EntryState = {
        default: EntryStateProps
        hovered?: EntryStateProps
        clicked?: EntryStateProps
    }

    const entry = (unselected?: EntryState, selected?: EntryState) => {
        const git_status = {
            git: {
                modified: is_light
                    ? theme.ramps.yellow(0.6).hex()
                    : theme.ramps.yellow(0.5).hex(),
                inserted: is_light
                    ? theme.ramps.green(0.45).hex()
                    : theme.ramps.green(0.5).hex(),
                conflict: is_light
                    ? theme.ramps.red(0.6).hex()
                    : theme.ramps.red(0.5).hex(),
            },
        }

        const base_properties = {
            height: 22,
            background: background(theme.middle),
            chevron_color: foreground(theme.middle, "variant"),
            icon_color: foreground(theme.middle, "active"),
            chevron_size: 7,
            icon_size: 14,
            icon_spacing: 5,
            text: text(theme.middle, "sans", "variant", { size: "sm" }),
            status: {
                ...git_status,
            },
        }

        const selected_style: EntryState | undefined = selected
            ? selected
            : unselected

        const unselected_default_style = merge(
            base_properties,
            unselected?.default ?? {},
            {}
        )
        const unselected_hovered_style = merge(
            base_properties,
            { background: background(theme.middle, "hovered") },
            unselected?.hovered ?? {}
        )
        const unselected_clicked_style = merge(
            base_properties,
            { background: background(theme.middle, "pressed") },
            unselected?.clicked ?? {}
        )
        const selected_default_style = merge(
            base_properties,
            {
                background: background(theme.lowest),
                text: text(theme.lowest, "sans", { size: "sm" }),
            },
            selected_style?.default ?? {}
        )
        const selected_hovered_style = merge(
            base_properties,
            {
                background: background(theme.lowest, "hovered"),
                text: text(theme.lowest, "sans", { size: "sm" }),
            },
            selected_style?.hovered ?? {}
        )
        const selected_clicked_style = merge(
            base_properties,
            {
                background: background(theme.lowest, "pressed"),
                text: text(theme.lowest, "sans", { size: "sm" }),
            },
            selected_style?.clicked ?? {}
        )

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
            },
        })
    }

    const default_entry = entry()

    return {
        open_project_button: interactive({
            base: {
                background: background(theme.middle),
                border: border(theme.middle, "active"),
                corner_radius: 4,
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
                ...text(theme.middle, "sans", "default", { size: "sm" }),
            },
            state: {
                hovered: {
                    ...text(theme.middle, "sans", "default", { size: "sm" }),
                    background: background(theme.middle, "hovered"),
                    border: border(theme.middle, "active"),
                },
                clicked: {
                    ...text(theme.middle, "sans", "default", { size: "sm" }),
                    background: background(theme.middle, "pressed"),
                    border: border(theme.middle, "active"),
                },
            },
        }),
        background: background(theme.middle),
        padding: { left: 6, right: 6, top: 0, bottom: 6 },
        indent_width: 12,
        entry: default_entry,
        dragged_entry: {
            ...default_entry.inactive.default,
            text: text(theme.middle, "sans", "on", { size: "sm" }),
            background: with_opacity(background(theme.middle, "on"), 0.9),
            border: border(theme.middle),
        },
        ignored_entry: entry(
            {
                default: {
                    text: text(theme.middle, "sans", "disabled"),
                },
            },
            {
                default: {
                    icon_color: foreground(theme.middle, "variant"),
                },
            }
        ),
        cut_entry: entry(
            {
                default: {
                    text: text(theme.middle, "sans", "disabled"),
                },
            },
            {
                default: {
                    background: background(theme.middle, "active"),
                    text: text(theme.middle, "sans", "disabled", {
                        size: "sm",
                    }),
                },
            }
        ),
        filename_editor: {
            background: background(theme.middle, "on"),
            text: text(theme.middle, "sans", "on", { size: "sm" }),
            selection: theme.players[0],
        },
    }
}
