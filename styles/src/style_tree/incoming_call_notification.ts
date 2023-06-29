import { ColorScheme } from "../theme/color_scheme"
import { background, border, text } from "./components"

export default function incoming_call_notification(
    colorScheme: ColorScheme
): unknown {
    const layer = colorScheme.middle
    const avatarSize = 48
    return {
        windowHeight: 74,
        windowWidth: 380,
        background: background(layer),
        callerContainer: {
            padding: 12,
        },
        callerAvatar: {
            height: avatarSize,
            width: avatarSize,
            corner_radius: avatarSize / 2,
        },
        callerMetadata: {
            margin: { left: 10 },
        },
        callerUsername: {
            ...text(layer, "sans", { size: "sm", weight: "bold" }),
            margin: { top: -3 },
        },
        callerMessage: {
            ...text(layer, "sans", "variant", { size: "xs" }),
            margin: { top: -3 },
        },
        worktreeRoots: {
            ...text(layer, "sans", "variant", { size: "xs", weight: "bold" }),
            margin: { top: -3 },
        },
        button_width: 96,
        acceptButton: {
            background: background(layer, "accent"),
            border: border(layer, { left: true, bottom: true }),
            ...text(layer, "sans", "positive", {
                size: "xs",
                weight: "bold",
            }),
        },
        declineButton: {
            border: border(layer, { left: true }),
            ...text(layer, "sans", "negative", {
                size: "xs",
                weight: "bold",
            }),
        },
    }
}
