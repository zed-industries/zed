import { ColorScheme } from "../theme/colorScheme"
import { background, border, foreground, text } from "./components"
import { interactive } from "../element"
import { toggleable } from "./toggle"
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
        activeLanguage: interactive({
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
        autoUpdateProgressMessage: text(layer, "sans", "variant"),
        autoUpdateDoneMessage: text(layer, "sans", "variant"),
        lspStatus: interactive({
            base: {
                ...diagnosticStatusContainer,
                iconSpacing: 4,
                iconWidth: 14,
                height: 18,
                message: text(layer, "sans"),
                iconColor: foreground(layer),
            },
            state: {
                hovered: {
                    message: text(layer, "sans"),
                    iconColor: foreground(layer),
                    background: background(layer, "hovered"),
                },
            },
        }),
        diagnosticMessage: interactive({
            base: {
                ...text(layer, "sans"),
            },
            state: { hovered: text(layer, "sans", "hovered") },
        }),
        diagnosticSummary: interactive({
            base: {
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
            },
            state: {
                hovered: {
                    iconColorOk: foreground(layer, "on"),
                    containerOk: {
                        background: background(layer, "on", "hovered"),
                    },
                    containerWarning: {
                        background: background(layer, "warning", "hovered"),
                        border: border(layer, "warning", "hovered"),
                    },
                    containerError: {
                        background: background(layer, "negative", "hovered"),
                        border: border(layer, "negative", "hovered"),
                    },
                },
            },
        }),
        panelButtons: {
            groupLeft: {},
            groupBottom: {},
            groupRight: {},
            button: toggleable(
                interactive({
                    base: {
                        ...statusContainer,
                        iconSize: 16,
                        iconColor: foreground(layer, "variant"),
                        label: {
                            margin: { left: 6 },
                            ...text(layer, "sans", { size: "sm" }),
                        },
                    },
                    state: {
                        hovered: {
                            iconColor: foreground(layer, "hovered"),
                            background: background(layer, "variant"),
                        },
                    },
                }),
                {
                    default: {
                        iconColor: foreground(layer, "active"),
                        background: background(layer, "active"),
                    },
                }
            ),
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
