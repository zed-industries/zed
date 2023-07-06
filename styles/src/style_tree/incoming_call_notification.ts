import { useTheme } from "../theme"
import { background, border, text } from "./components"

export default function incoming_call_notification(): unknown {
    const theme = useTheme()

    const avatar_size = 48
    return {
        window_height: 74,
        window_width: 380,
        background: background(theme.middle),
        caller_container: {
            padding: 12,
        },
        caller_avatar: {
            height: avatar_size,
            width: avatar_size,
            corner_radius: avatar_size / 2,
        },
        caller_metadata: {
            margin: { left: 10 },
        },
        caller_username: {
            ...text(theme.middle, "sans", { size: "sm", weight: "bold" }),
            margin: { top: -3 },
        },
        caller_message: {
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
        accept_button: {
            background: background(theme.middle, "accent"),
            border: border(theme.middle, { left: true, bottom: true }),
            ...text(theme.middle, "sans", "positive", {
                size: "xs",
                weight: "bold",
            }),
        },
        decline_button: {
            border: border(theme.middle, { left: true }),
            ...text(theme.middle, "sans", "negative", {
                size: "xs",
                weight: "bold",
            }),
        },
    }
}
