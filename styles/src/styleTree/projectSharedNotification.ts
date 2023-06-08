import { ColorScheme } from "../theme/colorScheme"
import { background, border, text } from "./components"

export default function projectSharedNotification(
    colorScheme: ColorScheme
): Object {
    let layer = colorScheme.middle

    const avatarSize = 48
    return {
        windowHeight: 74,
        windowWidth: 380,
        background: background(layer),
        ownerContainer: {
            padding: 12,
        },
        ownerAvatar: {
            height: avatarSize,
            width: avatarSize,
            cornerRadius: avatarSize / 2,
        },
        ownerMetadata: {
            margin: { left: 10 },
        },
        ownerUsername: {
            ...text(layer, "sans", { size: "sm", weight: "bold" }),
            margin: { top: -3 },
        },
        message: {
            ...text(layer, "sans", "variant", { size: "xs" }),
            margin: { top: -3 },
        },
        worktreeRoots: {
            ...text(layer, "sans", "variant", { size: "xs", weight: "bold" }),
            margin: { top: -3 },
        },
        buttonWidth: 96,
        openButton: {
            background: background(layer, "accent"),
            border: border(layer, { left: true, bottom: true }),
            ...text(layer, "sans", "accent", {
                size: "xs",
                weight: "extra_bold",
            }),
        },
        dismissButton: {
            border: border(layer, { left: true }),
            ...text(layer, "sans", "variant", {
                size: "xs",
                weight: "extra_bold",
            }),
        },
    }
}
