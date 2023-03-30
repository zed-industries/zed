import { ColorScheme } from "../themes/common/colorScheme";
import { background, border, text } from "./components";
import { ContainerStyle } from "../../../crates/gpui/bindings/ContainerStyle"
import { TextStyle } from "../../../crates/gpui/bindings/TextStyle"

type Container = Partial<ContainerStyle>
type Text = Partial<TextStyle>
interface ContainedText extends Container, Text { }

export const assistant = (colorScheme: ColorScheme) => {
    const layer = colorScheme.highest

    const message: Container = {
        margin: {
            top: 8,
            right: 8,
            bottom: 8,
            left: 8
        }
    }

    const messageContainer: Container = {
        background: background(layer, "on"),
        corner_radius: 6,
        padding: {
            top: 8,
            right: 8,
            bottom: 8,
            left: 8
        },
        margin: {
            top: 8,
            right: 8,
            bottom: 8,
            left: 8
        },
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

    const editor = {
        minWidth: 200,
        maxWidth: 500,
        padding: {
            top: 8,
            right: 8,
            bottom: 8,
            left: 8
        },
        cornerRadius: 8,
        border: border(layer, "on"),
        background: background(layer, "on"),
        text: text(layer, "sans", "default", { size: "sm" }),
        // placeholderText: text(layer, "mono", "disabled"),
        selection: colorScheme.players[0],
    }

    return {
        composer: {
            container: {
                padding: 8,
            },
            editor: editor,
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
