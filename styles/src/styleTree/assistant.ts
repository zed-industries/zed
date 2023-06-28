import { ColorScheme } from "../theme/colorScheme"
import { text, border, background, foreground } from "./components"
import editor from "./editor"
import { interactive } from "../element"

export default function assistant(colorScheme: ColorScheme) {
    const layer = colorScheme.highest
    return {
        container: {
            background: editor(colorScheme).background,
            padding: { left: 12 },
        },
        messageHeader: {
            margin: { bottom: 6, top: 6 },
            background: editor(colorScheme).background,
        },
        hamburgerButton: interactive({
            base: {
                icon: {
                    color: foreground(layer, "variant"),
                    asset: "icons/hamburger_15.svg",
                    dimensions: {
                        width: 15,
                        height: 15,
                    },
                },
                container: {
                    padding: { left: 12, right: 8.5 },
                },
            },
            state: {
                hovered: {
                    icon: {
                        color: foreground(layer, "hovered"),
                    },
                },
            },
        }),
        splitButton: interactive({
            base: {
                icon: {
                    color: foreground(layer, "variant"),
                    asset: "icons/split_message_15.svg",
                    dimensions: {
                        width: 15,
                        height: 15,
                    },
                },
                container: {
                    padding: { left: 8.5, right: 8.5 },
                },
            },
            state: {
                hovered: {
                    icon: {
                        color: foreground(layer, "hovered"),
                    },
                },
            },
        }),
        quoteButton: interactive({
            base: {
                icon: {
                    color: foreground(layer, "variant"),
                    asset: "icons/quote_15.svg",
                    dimensions: {
                        width: 15,
                        height: 15,
                    },
                },
                container: {
                    padding: { left: 8.5, right: 8.5 },
                },
            },
            state: {
                hovered: {
                    icon: {
                        color: foreground(layer, "hovered"),
                    },
                },
            },
        }),
        assistButton: interactive({
            base: {
                icon: {
                    color: foreground(layer, "variant"),
                    asset: "icons/assist_15.svg",
                    dimensions: {
                        width: 15,
                        height: 15,
                    },
                },
                container: {
                    padding: { left: 8.5, right: 8.5 },
                },
            },
            state: {
                hovered: {
                    icon: {
                        color: foreground(layer, "hovered"),
                    },
                },
            },
        }),
        zoomInButton: interactive({
            base: {
                icon: {
                    color: foreground(layer, "variant"),
                    asset: "icons/maximize_8.svg",
                    dimensions: {
                        width: 12,
                        height: 12,
                    },
                },
                container: {
                    padding: { left: 10, right: 10 },
                },
            },
            state: {
                hovered: {
                    icon: {
                        color: foreground(layer, "hovered"),
                    },
                },
            },
        }),
        zoomOutButton: interactive({
            base: {
                icon: {
                    color: foreground(layer, "variant"),
                    asset: "icons/minimize_8.svg",
                    dimensions: {
                        width: 12,
                        height: 12,
                    },
                },
                container: {
                    padding: { left: 10, right: 10 },
                },
            },
            state: {
                hovered: {
                    icon: {
                        color: foreground(layer, "hovered"),
                    },
                },
            },
        }),
        plusButton: interactive({
            base: {
                icon: {
                    color: foreground(layer, "variant"),
                    asset: "icons/plus_12.svg",
                    dimensions: {
                        width: 12,
                        height: 12,
                    },
                },
                container: {
                    padding: { left: 10, right: 10 },
                },
            },
            state: {
                hovered: {
                    icon: {
                        color: foreground(layer, "hovered"),
                    },
                },
            },
        }),
        title: {
            ...text(layer, "sans", "default", { size: "sm" }),
        },
        savedConversation: {
            container: interactive({
                base: {
                    background: background(layer, "on"),
                    padding: { top: 4, bottom: 4 },
                },
                state: {
                    hovered: {
                        background: background(layer, "on", "hovered"),
                    },
                },
            }),
            savedAt: {
                margin: { left: 8 },
                ...text(layer, "sans", "default", { size: "xs" }),
            },
            title: {
                margin: { left: 16 },
                ...text(layer, "sans", "default", {
                    size: "sm",
                    weight: "bold",
                }),
            },
        },
        userSender: {
            default: {
                ...text(layer, "sans", "default", {
                    size: "sm",
                    weight: "bold",
                }),
            },
        },
        assistantSender: {
            default: {
                ...text(layer, "sans", "accent", {
                    size: "sm",
                    weight: "bold",
                }),
            },
        },
        systemSender: {
            default: {
                ...text(layer, "sans", "variant", {
                    size: "sm",
                    weight: "bold",
                }),
            },
        },
        sentAt: {
            margin: { top: 2, left: 8 },
            ...text(layer, "sans", "default", { size: "2xs" }),
        },
        model: interactive({
            base: {
                background: background(layer, "on"),
                margin: { left: 12, right: 12, top: 12 },
                padding: 4,
                cornerRadius: 4,
                ...text(layer, "sans", "default", { size: "xs" }),
            },
            state: {
                hovered: {
                    background: background(layer, "on", "hovered"),
                    border: border(layer, "on", { overlay: true }),
                },
            },
        }),
        remainingTokens: {
            background: background(layer, "on"),
            margin: { top: 12, right: 24 },
            padding: 4,
            cornerRadius: 4,
            ...text(layer, "sans", "positive", { size: "xs" }),
        },
        noRemainingTokens: {
            background: background(layer, "on"),
            margin: { top: 12, right: 24 },
            padding: 4,
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
