import { ColorScheme } from "../theme/colorScheme"
import { text, border, background, foreground } from "./components"
import editor from "./editor"

export default function assistant(colorScheme: ColorScheme) {
    const layer = colorScheme.highest
    return {
        container: {
            background: editor(colorScheme).background,
            padding: { left: 12 },
        },
        messageHeader: {
            border: border(layer, "default", { bottom: true, top: true }),
            margin: { bottom: 6, top: 6 },
            background: editor(colorScheme).background,
        },
        hamburgerButton: {
          icon: {
            color: text(layer, "sans", "default", { size: "sm" }).color,
            asset: "icons/hamburger.svg",
            dimensions: {
              width: 15,
              height: 15,
            },
          },
          container: {}
        },
        userSender: {
            ...text(layer, "sans", "default", { size: "sm", weight: "bold" }),
        },
        assistantSender: {
            ...text(layer, "sans", "accent", { size: "sm", weight: "bold" }),
        },
        systemSender: {
            ...text(layer, "sans", "variant", { size: "sm", weight: "bold" }),
        },
        sentAt: {
            margin: { top: 2, left: 8 },
            ...text(layer, "sans", "default", { size: "2xs" }),
        },
        modelInfoContainer: {
            margin: { right: 16, top: 4 },
        },
        model: {
            background: background(layer, "on"),
            border: border(layer, "on", { overlay: true }),
            padding: 4,
            cornerRadius: 4,
            ...text(layer, "sans", "default", { size: "xs" }),
            hover: {
                background: background(layer, "on", "hovered"),
            },
        },
        remainingTokens: {
            background: background(layer, "on"),
            border: border(layer, "on", { overlay: true }),
            padding: 4,
            margin: { left: 4 },
            cornerRadius: 4,
            ...text(layer, "sans", "positive", { size: "xs" }),
        },
        noRemainingTokens: {
            background: background(layer, "on"),
            border: border(layer, "on", { overlay: true }),
            padding: 4,
            margin: { left: 4 },
            cornerRadius: 4,
            ...text(layer, "sans", "negative", { size: "xs" }),
        },
        errorIcon: {
            margin: { left: 8 },
            color: foreground(layer, "negative"),
            width: 12,
        },
        apiKeyEditor: {
            background: background(layer, "on"),
            cornerRadius: 6,
            text: text(layer, "mono", "on"),
            placeholderText: text(layer, "mono", "on", "disabled", {
                size: "xs",
            }),
            selection: colorScheme.players[0],
            border: border(layer, "on"),
            padding: {
                bottom: 4,
                left: 8,
                right: 8,
                top: 4,
            },
        },
        apiKeyPrompt: {
            padding: 10,
            ...text(layer, "sans", "default", { size: "xs" }),
        },
    }
}
