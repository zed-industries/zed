import { background, border, foreground, text } from "./components"
import { interactive, toggleable } from "../element"
import { useTheme } from "../common"
export default function status_bar(): any {
    const theme = useTheme()

    const layer = theme.lowest

    const status_container = {
        corner_radius: 6,
        padding: { top: 3, bottom: 3, left: 6, right: 6 },
    }

    const diagnostic_status_container = {
        corner_radius: 6,
        padding: { top: 1, bottom: 1, left: 6, right: 6 },
    }

    return {
        height: 30,
        item_spacing: 8,
        padding: {
            top: 1,
            bottom: 1,
            left: 6,
            right: 6,
        },
        border: border(layer, { top: true, overlay: true }),
        cursor_position: text(layer, "sans", "variant"),
        vim_mode: text(layer, "sans", "variant"),
        active_language: interactive({
            base: {
                padding: { left: 6, right: 6 },
                ...text(layer, "sans", "variant"),
            },
            state: {
                hovered: {
                    ...text(layer, "sans", "on"),
                },
            },
        }),
        auto_update_progress_message: text(layer, "sans", "variant"),
        auto_update_done_message: text(layer, "sans", "variant"),
        lsp_status: interactive({
            base: {
                ...diagnostic_status_container,
                icon_spacing: 4,
                icon_width: 14,
                height: 18,
                message: text(layer, "sans"),
                icon_color: foreground(layer),
            },
            state: {
                hovered: {
                    message: text(layer, "sans"),
                    icon_color: foreground(layer),
                    background: background(layer, "hovered"),
                },
            },
        }),
        diagnostic_message: interactive({
            base: {
                ...text(layer, "sans"),
            },
            state: { hovered: text(layer, "sans", "hovered") },
        }),
        diagnostic_summary: interactive({
            base: {
                height: 20,
                icon_width: 16,
                icon_spacing: 2,
                summary_spacing: 6,
                text: text(layer, "sans", { size: "sm" }),
                icon_color_ok: foreground(layer, "variant"),
                icon_color_warning: foreground(layer, "warning"),
                icon_color_error: foreground(layer, "negative"),
                container_ok: {
                    corner_radius: 6,
                    padding: { top: 3, bottom: 3, left: 7, right: 7 },
                },
                container_warning: {
                    ...diagnostic_status_container,
                    background: background(layer, "warning"),
                    border: border(layer, "warning"),
                },
                container_error: {
                    ...diagnostic_status_container,
                    background: background(layer, "negative"),
                    border: border(layer, "negative"),
                },
            },
            state: {
                hovered: {
                    icon_color_ok: foreground(layer, "on"),
                    container_ok: {
                        background: background(layer, "on", "hovered"),
                    },
                    container_warning: {
                        background: background(layer, "warning", "hovered"),
                        border: border(layer, "warning", "hovered"),
                    },
                    container_error: {
                        background: background(layer, "negative", "hovered"),
                        border: border(layer, "negative", "hovered"),
                    },
                },
            },
        }),
        panel_buttons: {
            group_left: {},
            group_bottom: {},
            group_right: {},
            button: toggleable({
                base: interactive({
                    base: {
                        ...status_container,
                        icon_size: 16,
                        icon_color: foreground(layer, "variant"),
                        label: {
                            margin: { left: 6 },
                            ...text(layer, "sans", { size: "sm" }),
                        },
                    },
                    state: {
                        hovered: {
                            icon_color: foreground(layer, "hovered"),
                            background: background(layer, "variant"),
                        },
                    },
                }),
                state: {
                    active: {
                        default: {
                            icon_color: foreground(layer, "active"),
                            background: background(layer, "active"),
                        },
                        hovered: {
                            icon_color: foreground(layer, "hovered"),
                            background: background(layer, "hovered"),
                        },
                        clicked: {
                            icon_color: foreground(layer, "pressed"),
                            background: background(layer, "pressed"),
                        },
                    },
                },
            }),
            badge: {
                corner_radius: 3,
                padding: 2,
                margin: { bottom: -1, right: -1 },
                border: border(layer),
                background: background(layer, "accent"),
            },
        },
    }
}
