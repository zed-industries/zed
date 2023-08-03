import { with_opacity } from "../theme/color"
import {
    background,
    border,
    border_color,
    foreground,
    svg,
    text,
} from "./components"
import statusBar from "./status_bar"
import tabBar from "./tab_bar"
import { interactive } from "../element"
import { titlebar } from "./titlebar"
import { useTheme } from "../theme"

export default function workspace(): any {
    const theme = useTheme()

    const { is_light } = theme

    return {
        background: background(theme.lowest),
        blank_pane: {
            logo_container: {
                width: 256,
                height: 256,
            },
            logo: svg(
                with_opacity("#000000", theme.is_light ? 0.6 : 0.8),
                "icons/logo_96.svg",
                256,
                256
            ),

            logo_shadow: svg(
                with_opacity(
                    theme.is_light
                        ? "#FFFFFF"
                        : theme.lowest.base.default.background,
                    theme.is_light ? 1 : 0.6
                ),
                "icons/logo_96.svg",
                256,
                256
            ),
            keyboard_hints: {
                margin: {
                    top: 96,
                },
                corner_radius: 4,
            },
            keyboard_hint: interactive({
                base: {
                    ...text(theme.lowest, "sans", "variant", { size: "sm" }),
                    padding: {
                        top: 3,
                        left: 8,
                        right: 8,
                        bottom: 3,
                    },
                    corner_radius: 8,
                },
                state: {
                    hovered: {
                        ...text(theme.lowest, "sans", "active", { size: "sm" }),
                    },
                },
            }),

            keyboard_hint_width: 320,
        },
        joining_project_avatar: {
            corner_radius: 40,
            width: 80,
        },
        joining_project_message: {
            padding: 12,
            ...text(theme.lowest, "sans", { size: "lg" }),
        },
        external_location_message: {
            background: background(theme.middle, "accent"),
            border: border(theme.middle, "accent"),
            corner_radius: 6,
            padding: 12,
            margin: { bottom: 8, right: 8 },
            ...text(theme.middle, "sans", "accent", { size: "xs" }),
        },
        leader_border_opacity: 0.7,
        leader_border_width: 2.0,
        tab_bar: tabBar(),
        modal: {
            margin: {
                bottom: 52,
                top: 52,
            },
            cursor: "Arrow",
        },
        zoomed_background: {
            cursor: "Arrow",
            background: is_light
                ? with_opacity(background(theme.lowest), 0.8)
                : with_opacity(background(theme.highest), 0.6),
        },
        zoomed_pane_foreground: {
            margin: 16,
            shadow: theme.modal_shadow,
            border: border(theme.lowest, { overlay: true }),
        },
        zoomed_panel_foreground: {
            margin: 16,
            border: border(theme.lowest, { overlay: true }),
        },
        dock: {
            left: {
                border: border(theme.lowest, { right: true }),
            },
            bottom: {
                border: border(theme.lowest, { top: true }),
            },
            right: {
                border: border(theme.lowest, { left: true }),
            },
        },
        pane_divider: {
            color: border_color(theme.lowest),
            width: 1,
        },
        status_bar: statusBar(),
        titlebar: titlebar(),
        toolbar: {
            height: 34,
            background: background(theme.highest),
            border: border(theme.highest, { bottom: true }),
            item_spacing: 8,
            padding: { left: 8, right: 8, top: 4, bottom: 4 },
        },
        breadcrumb_height: 24,
        breadcrumbs: interactive({
            base: {
                ...text(theme.highest, "sans", "variant"),
                corner_radius: 6,
                padding: {
                    left: 6,
                    right: 6,
                },
            },
            state: {
                hovered: {
                    color: foreground(theme.highest, "on", "hovered"),
                    background: background(theme.highest, "on", "hovered"),
                },
            },
        }),
        disconnected_overlay: {
            ...text(theme.lowest, "sans"),
            background: with_opacity(background(theme.lowest), 0.8),
        },
        notification: {
            margin: { top: 10 },
            background: background(theme.middle),
            corner_radius: 6,
            padding: 12,
            border: border(theme.middle),
            shadow: theme.popover_shadow,
        },
        notifications: {
            width: 400,
            margin: { right: 10, bottom: 10 },
        },
        drop_target_overlay_color: with_opacity(
            foreground(theme.lowest, "variant"),
            0.5
        ),
    }
}
