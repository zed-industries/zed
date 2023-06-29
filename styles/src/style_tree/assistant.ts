import { ColorScheme } from "../theme/color_scheme"
import { text, border, background, foreground } from "./components"
import { interactive } from "../element"

export default function assistant(theme: ColorScheme): any {
    return {
        container: {
            background: background(theme.highest),
            padding: { left: 12 },
        },
        message_header: {
            margin: { bottom: 6, top: 6 },
            background: background(theme.highest),
        },
        hamburger_button: interactive({
            base: {
                icon: {
                    color: foreground(theme.highest, "variant"),
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
                        color: foreground(theme.highest, "hovered"),
                    },
                },
            },
        }),
        split_button: interactive({
            base: {
                icon: {
                    color: foreground(theme.highest, "variant"),
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
                        color: foreground(theme.highest, "hovered"),
                    },
                },
            },
        }),
        quote_button: interactive({
            base: {
                icon: {
                    color: foreground(theme.highest, "variant"),
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
                        color: foreground(theme.highest, "hovered"),
                    },
                },
            },
        }),
        assist_button: interactive({
            base: {
                icon: {
                    color: foreground(theme.highest, "variant"),
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
                        color: foreground(theme.highest, "hovered"),
                    },
                },
            },
        }),
        zoom_in_button: interactive({
            base: {
                icon: {
                    color: foreground(theme.highest, "variant"),
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
                        color: foreground(theme.highest, "hovered"),
                    },
                },
            },
        }),
        zoom_out_button: interactive({
            base: {
                icon: {
                    color: foreground(theme.highest, "variant"),
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
                        color: foreground(theme.highest, "hovered"),
                    },
                },
            },
        }),
        plus_button: interactive({
            base: {
                icon: {
                    color: foreground(theme.highest, "variant"),
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
                        color: foreground(theme.highest, "hovered"),
                    },
                },
            },
        }),
        title: {
            ...text(theme.highest, "sans", "default", { size: "sm" }),
        },
        saved_conversation: {
            container: interactive({
                base: {
                    background: background(theme.highest, "on"),
                    padding: { top: 4, bottom: 4 },
                },
                state: {
                    hovered: {
                        background: background(theme.highest, "on", "hovered"),
                    },
                },
            }),
            saved_at: {
                margin: { left: 8 },
                ...text(theme.highest, "sans", "default", { size: "xs" }),
            },
            title: {
                margin: { left: 16 },
                ...text(theme.highest, "sans", "default", {
                    size: "sm",
                    weight: "bold",
                }),
            },
        },
        user_sender: {
            default: {
                ...text(theme.highest, "sans", "default", {
                    size: "sm",
                    weight: "bold",
                }),
            },
        },
        assistant_sender: {
            default: {
                ...text(theme.highest, "sans", "accent", {
                    size: "sm",
                    weight: "bold",
                }),
            },
        },
        system_sender: {
            default: {
                ...text(theme.highest, "sans", "variant", {
                    size: "sm",
                    weight: "bold",
                }),
            },
        },
        sent_at: {
            margin: { top: 2, left: 8 },
            ...text(theme.highest, "sans", "default", { size: "2xs" }),
        },
        model: interactive({
            base: {
                background: background(theme.highest, "on"),
                margin: { left: 12, right: 12, top: 12 },
                padding: 4,
                corner_radius: 4,
                ...text(theme.highest, "sans", "default", { size: "xs" }),
            },
            state: {
                hovered: {
                    background: background(theme.highest, "on", "hovered"),
                    border: border(theme.highest, "on", { overlay: true }),
                },
            },
        }),
        remaining_tokens: {
            background: background(theme.highest, "on"),
            margin: { top: 12, right: 24 },
            padding: 4,
            corner_radius: 4,
            ...text(theme.highest, "sans", "positive", { size: "xs" }),
        },
        no_remaining_tokens: {
            background: background(theme.highest, "on"),
            margin: { top: 12, right: 24 },
            padding: 4,
            corner_radius: 4,
            ...text(theme.highest, "sans", "negative", { size: "xs" }),
        },
        error_icon: {
            margin: { left: 8 },
            color: foreground(theme.highest, "negative"),
            width: 12,
        },
        api_key_editor: {
            background: background(theme.highest, "on"),
            corner_radius: 6,
            text: text(theme.highest, "mono", "on"),
            placeholder_text: text(theme.highest, "mono", "on", "disabled", {
                size: "xs",
            }),
            selection: theme.players[0],
            border: border(theme.highest, "on"),
            padding: {
                bottom: 4,
                left: 8,
                right: 8,
                top: 4,
            },
        },
        api_key_prompt: {
            padding: 10,
            ...text(theme.highest, "sans", "default", { size: "xs" }),
        },
    }
}
