import { ColorScheme } from "../themes/common/colorScheme";
import { background, border, text } from "./components";

export const assistant = (colorScheme: ColorScheme) => {
    const layer = colorScheme.highest

    const message = {
        margin: 8
    }

    const messageContainer = {
        background: background(layer, "on"),
        cornerRadius: 6,
        padding: 8,
        margin: 8,
    }

    const messageHeader = {
        image: {
            width: 20,
            height: 20,
            corderRadius: 10
        },
        name: {
            ...text(layer, "sans", "default", { size: "sm" }),
        },
        time: {
            ...text(layer, "sans", "variant", { size: "sm" }),
        }
    }

    return {
        composer: {
            container: {
                padding: 8,
            },
            editor: {
                minWidth: 200,
                maxWidth: 500,
                padding: 8,
                cornerRadius: 8,
                border: border(layer, "on"),
                background: background(layer, "on"),
                text: text(layer, "sans", "default", { size: "sm" }),
                // placeholderText: text(layer, "mono", "disabled"),
                selection: colorScheme.players[0],
            }
        },
        assistant_message: {
            ...messageContainer,
            ...text(layer, "sans", "accent", { size: "sm" }),
        },
        player_message: {
            ...messageContainer,
            ...text(layer, "sans", "default", { size: "sm" }),
        },
        error_message: {
            ...messageContainer,
            background: background(layer, "negative"),
            ...text(layer, "sans", "accent", { size: "sm" }),
        },
    }
}
