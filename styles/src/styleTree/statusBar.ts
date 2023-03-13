import { ColorScheme } from "../themes/common/colorScheme"
import { background, border, foreground, text } from "./components"

export default function statusBar(colorScheme: ColorScheme) {
    let layer = colorScheme.lowest

    const statusContainer = {
        cornerRadius: 6,
        padding: { top: 3, bottom: 3, left: 6, right: 6 },
    }

    const diagnosticStatusContainer = {
        cornerRadius: 6,
        padding: { top: 1, bottom: 1, left: 6, right: 6 },
    }

    return {
        height: 30,
        itemSpacing: 8,
        padding: {
            top: 1,
            bottom: 1,
            left: 6,
            right: 6,
        },
        border: border(layer, { top: true, overlay: true }),
        cursorPosition: text(layer, "sans", "variant"),
        activeLanguage: {
            padding: { left: 6, right: 6 },
            ...text(layer, "sans", "variant"),
            hover: {
                ...text(layer, "sans", "on"),
            },
        },
        autoUpdateProgressMessage: text(layer, "sans", "variant"),
        autoUpdateDoneMessage: text(layer, "sans", "variant"),
        lspStatus: {
            ...diagnosticStatusContainer,
            iconSpacing: 4,
            iconWidth: 14,
            height: 18,
            message: text(layer, "sans"),
            iconColor: foreground(layer),
            hover: {
                message: text(layer, "sans"),
                iconColor: foreground(layer),
                background: background(layer),
            },
        },
        diagnosticMessage: {
            ...text(layer, "sans"),
            hover: text(layer, "sans", "hovered"),
        },
        diagnosticSummary: {
            height: 20,
            iconWidth: 16,
            iconSpacing: 2,
            summarySpacing: 6,
            text: text(layer, "sans", { size: "sm" }),
            iconColorOk: foreground(layer, "variant"),
            iconColorWarning: foreground(layer, "warning"),
            iconColorError: foreground(layer, "negative"),
            containerOk: {
                cornerRadius: 6,
                padding: { top: 3, bottom: 3, left: 7, right: 7 },
            },
            containerWarning: {
                ...diagnosticStatusContainer,
                background: background(layer, "warning"),
                border: border(layer, "warning"),
            },
            containerError: {
                ...diagnosticStatusContainer,
                background: background(layer, "negative"),
                border: border(layer, "negative"),
            },
            hover: {
                iconColorOk: foreground(layer, "on"),
                containerOk: {
                    cornerRadius: 6,
                    padding: { top: 3, bottom: 3, left: 7, right: 7 },
                    background: background(layer, "on", "hovered"),
                },
                containerWarning: {
                    ...diagnosticStatusContainer,
                    background: background(layer, "warning", "hovered"),
                    border: border(layer, "warning", "hovered"),
                },
                containerError: {
                    ...diagnosticStatusContainer,
                    background: background(layer, "negative", "hovered"),
                    border: border(layer, "negative", "hovered"),
                },
            },
        },
        sidebarButtons: {
            groupLeft: {},
            groupRight: {},
            item: {
                ...statusContainer,
                iconSize: 16,
                iconColor: foreground(layer, "variant"),
                hover: {
                    iconColor: foreground(layer, "hovered"),
                    background: background(layer, "variant"),
                },
                active: {
                    iconColor: foreground(layer, "active"),
                    background: background(layer, "active"),
                },
            },
            badge: {
                cornerRadius: 3,
                padding: 2,
                margin: { bottom: -1, right: -1 },
                border: border(layer),
                background: background(layer, "accent"),
            },
        },
    }
}
