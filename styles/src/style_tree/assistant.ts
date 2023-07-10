import { text, border, background, foreground, TextStyle } from "./components"
import { Interactive, interactive } from "../element"
import { tab_bar_button } from "../component/tab_bar_button"
import { StyleSets, useTheme } from "../theme"

type RoleCycleButton = TextStyle & {
    background?: string
}
// TODO: Replace these with zed types
type RemainingTokens = TextStyle & {
    background: string,
    margin: { top: number, right: number },
    padding: {
        right: number,
        left: number,
        top: number,
        bottom: number,
    },
    corner_radius: number,
}

export default function assistant(): any {
    const theme = useTheme()

    const interactive_role = (color: StyleSets): Interactive<RoleCycleButton> => {
        return (
            interactive({
                base: {
                    ...text(theme.highest, "sans", color, { size: "sm" }),
                },
                state: {
                    hovered: {
                        ...text(theme.highest, "sans", color, { size: "sm" }),
                        background: background(theme.highest, color, "hovered"),
                    },
                    clicked: {
                        ...text(theme.highest, "sans", color, { size: "sm" }),
                        background: background(theme.highest, color, "pressed"),
                    }
                },
            })
        )
    }

    const tokens_remaining = (color: StyleSets): RemainingTokens => {
        return (
            {
                ...text(theme.highest, "mono", color, { size: "xs" }),
                background: background(theme.highest, "on", "default"),
                margin: { top: 12, right: 20 },
                padding: { right: 4, left: 4, top: 1, bottom: 1 },
                corner_radius: 6,
            }
        )
    }

    return {
        container: {
            background: background(theme.highest),
            padding: { left: 12 },
        },
        message_header: {
            margin: { bottom: 4, top: 4 },
            background: background(theme.highest),
        },
        hamburger_button: tab_bar_button(theme, {
            icon: "icons/hamburger_15.svg",
        }),

        split_button: tab_bar_button(theme, {
            icon: "icons/split_message_15.svg",
        }),
        quote_button: tab_bar_button(theme, {
            icon: "icons/radix/quote.svg",
        }),
        assist_button: tab_bar_button(theme, {
            icon: "icons/radix/magic-wand.svg",
        }),
        zoom_in_button: tab_bar_button(theme, {
            icon: "icons/radix/maximize.svg",
        }),
        zoom_out_button: tab_bar_button(theme, {
            icon: "icons/radix/minimize.svg",
        }),
        plus_button: tab_bar_button(theme, {
            icon: "icons/radix/plus.svg",
        }),
        title: {
            ...text(theme.highest, "sans", "default", { size: "xs" }),
        },
        saved_conversation: {
            container: interactive({
                base: {
                    background: background(theme.middle),
                    padding: { top: 4, bottom: 4 },
                    border: border(theme.middle, "default", { top: true, overlay: true }),
                },
                state: {
                    hovered: {
                        background: background(theme.middle, "hovered"),
                    },
                    clicked: {
                        background: background(theme.middle, "pressed"),
                    }
                },
            }),
            saved_at: {
                margin: { left: 8 },
                ...text(theme.highest, "sans", "variant", { size: "xs" }),
            },
            title: {
                margin: { left: 12 },
                ...text(theme.highest, "sans", "default", {
                    size: "sm",
                    weight: "bold",
                }),
            },
        },
        user_sender: interactive_role("base"),
        assistant_sender: interactive_role("accent"),
        system_sender: interactive_role("warning"),
        sent_at: {
            margin: { top: 2, left: 8 },
            ...text(theme.highest, "sans", "variant", { size: "2xs" }),
        },
        model: interactive({
            base: {
                background: background(theme.highest),
                margin: { left: 12, right: 4, top: 12 },
                padding: { right: 4, left: 4, top: 1, bottom: 1 },
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
        remaining_tokens: tokens_remaining("positive"),
        low_remaining_tokens: tokens_remaining("warning"),
        no_remaining_tokens: tokens_remaining("negative"),
        error_icon: {
            margin: { left: 8 },
            color: foreground(theme.highest, "negative"),
            width: 12,
        },
        api_key_editor: {
            background: background(theme.highest, "on"),
            corner_radius: 4,
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
