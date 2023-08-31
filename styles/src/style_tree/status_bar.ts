import { background, border, foreground, text } from "./components"
import { interactive, toggleable } from "../element"
import { useTheme } from "../common"
import { text_button } from "../component/text_button"

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
        cursor_position: text(layer, "sans", "base", { size: "xs" }),
        vim_mode_indicator: {
            margin: { left: 6 },
            ...text(layer, "mono", "base", { size: "xs" }),
        },
        active_language: text_button({
            color: "base",
        }),
        auto_update_progress_message: text(layer, "sans", "base", {
            size: "xs",
        }),
        auto_update_done_message: text(layer, "sans", "base", { size: "xs" }),
        lsp_status: interactive({
            base: {
                ...diagnostic_status_container,
                icon_spacing: 4,
                icon_width: 14,
                height: 18,
                message: text(layer, "sans", { size: "xs" }),
                icon_color: foreground(layer),
            },
            state: {
                hovered: {
                    message: text(layer, "sans", { size: "xs" }),
                    icon_color: foreground(layer),
                    background: background(layer, "hovered"),
                },
            },
        }),
        diagnostic_message: interactive({
            base: {
                ...text(layer, "sans", { size: "xs" }),
            },
            state: { hovered: text(layer, "sans", "hovered", { size: "xs" }) },
        }),
        diagnostic_summary: interactive({
            base: {
                height: 20,
                icon_width: 14,
                icon_spacing: 2,
                summary_spacing: 6,
                text: text(layer, "sans", { size: "sm" }),
                icon_color_ok: foreground(layer, "base"),
                icon_color_warning: foreground(layer, "warning"),
                icon_color_error: foreground(layer, "negative"),
                container_ok: {
                    corner_radius: 6,
                    padding: { top: 2, bottom: 2, left: 6, right: 6 },
                },
                container_warning: diagnostic_status_container,
                container_error: diagnostic_status_container
            },
            state: {
                hovered: {
                    icon_color_ok: foreground(layer, "on"),
                    container_ok: {
                        background: background(layer, "hovered")
                    },
                    container_warning: {
                        background: background(layer, "hovered")
                    },
                    container_error: {
                        background: background(layer, "hovered")
                    },
                },
                clicked: {
                    icon_color_ok: foreground(layer, "on"),
                    container_ok: {
                        background: background(layer, "pressed")
                    },
                    container_warning: {
                        background: background(layer, "pressed")
                    },
                    container_error: {
                        background: background(layer, "pressed")
                    }
                }
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
                        icon_size: 14,
                        icon_color: foreground(layer, "base"),
                        background: background(layer, "default"),
                        label: {
                            margin: { left: 6 },
                            ...text(layer, "sans", { size: "xs" }),
                        },
                    },
                    state: {
                        hovered: {
                            background: background(layer, "hovered"),
                        },
                        clicked: {
                            background: background(layer, "pressed"),
                        },
                    },
                }),
                state: {
                    active: {
                        default: {
                            icon_color: foreground(layer, "accent", "default"),
                            background: background(layer, "default"),
                        },
                        hovered: {
                            icon_color: foreground(layer, "accent", "hovered"),
                            background: background(layer, "hovered"),
                        },
                        clicked: {
                            icon_color: foreground(layer, "accent", "pressed"),
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
