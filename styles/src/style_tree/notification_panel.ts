import { background, border, text } from "./components"
import { icon_button } from "../component/icon_button"
import { useTheme, with_opacity } from "../theme"
import { text_button } from "../component"

export default function (): any {
    const theme = useTheme()
    const layer = theme.middle

    const notification_text = {
        padding: { top: 4, bottom: 4 },
        ...text(layer, "sans", "base"),
    }

    const notification_read_text_color = with_opacity(
        theme.middle.base.default.foreground,
        0.6
    )

    return {
        background: background(layer),
        avatar: {
            icon_width: 24,
            icon_height: 24,
            corner_radius: 12,
            outer_width: 24,
            outer_corner_radius: 24,
        },
        title: {
            ...text(layer, "sans", "default"),
            padding: { left: 8, right: 8 },
            border: border(layer, { bottom: true }),
        },
        title_height: 32,
        title_icon: {
            asset: "icons/feedback.svg",
            color: text(theme.lowest, "sans", "default").color,
            dimensions: {
                width: 16,
                height: 16,
            },
        },
        read_text: {
            ...notification_text,
            color: notification_read_text_color,
        },
        meta_text: {
            padding: { top: 4, bottom: 4, right: 4 },
            ...text(layer, "sans", "base"),
            color: with_opacity(
                theme.middle.base.default.foreground,
                0.6)
        },
        unread_text: notification_text,
        button: text_button({
            variant: "ghost",
        }),
        timestamp: text(layer, "sans", "base", "disabled"),
        avatar_container: {
            padding: {
                right: 8,
                left: 2,
                top: 4,
                bottom: 2,
            },
        },
        list: {
            padding: {
                left: 8,
                right: 8,
            },
        },
        icon_button: icon_button({
            variant: "ghost",
            color: "variant",
            size: "sm",
        }),
        sign_in_prompt: {
            default: text(layer, "sans", "base"),
        },
    }
}
