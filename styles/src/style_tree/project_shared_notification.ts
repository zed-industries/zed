import { ColorScheme } from "../theme/color_scheme"
import { background, border, text } from "./components"

export default function project_shared_notification(
    theme: ColorScheme
): unknown {
    const avatar_size = 48
    return {
        window_height: 74,
        window_width: 380,
        background: background(theme.middle),
        owner_container: {
            padding: 12,
        },
        owner_avatar: {
            height: avatar_size,
            width: avatar_size,
            corner_radius: avatar_size / 2,
        },
        owner_metadata: {
            margin: { left: 10 },
        },
        owner_username: {
            ...text(theme.middle, "sans", { size: "sm", weight: "bold" }),
            margin: { top: -3 },
        },
        message: {
            ...text(theme.middle, "sans", "variant", { size: "xs" }),
            margin: { top: -3 },
        },
        worktree_roots: {
            ...text(theme.middle, "sans", "variant", {
                size: "xs",
                weight: "bold",
            }),
            margin: { top: -3 },
        },
        button_width: 96,
        open_button: {
            background: background(theme.middle, "accent"),
            border: border(theme.middle, { left: true, bottom: true }),
            ...text(theme.middle, "sans", "accent", {
                size: "xs",
                weight: "bold",
            }),
        },
        dismiss_button: {
            border: border(theme.middle, { left: true }),
            ...text(theme.middle, "sans", "variant", {
                size: "xs",
                weight: "bold",
            }),
        },
    }
}
