import { ColorScheme } from "../themes/common/colorScheme"
import { background, border, foreground, svg, text } from "./components";


export default function copilot(colorScheme: ColorScheme) {
    let layer = colorScheme.highest;

    return {
        auth: {
            popupContainer: {
                background: background(colorScheme.highest),
            },
            popupDimensions: {
                width: 336,
                height: 256,
            },
            instructionText: text(layer, "sans"),
            userCode:
                text(layer, "sans", { size: "lg" }),
            button: { // Copied from welcome screen. FIXME: Move this into a ZDS component
                background: background(layer),
                border: border(layer, "active"),
                cornerRadius: 4,
                margin: {
                    top: 4,
                    bottom: 4,
                },
                padding: {
                    top: 3,
                    bottom: 3,
                    left: 7,
                    right: 7,
                },
                ...text(layer, "sans", "default", { size: "sm" }),
                hover: {
                    ...text(layer, "sans", "default", { size: "sm" }),
                    background: background(layer, "hovered"),
                    border: border(layer, "active"),
                },
            },
            buttonWidth: 320,
            copilotIcon: svg(foreground(layer, "default"), "icons/github-copilot-dummy.svg", 64, 64),
            closeIcon: {
                icon: svg(background(layer, "on"), "icons/x_mark_16.svg", 16, 16),
                container: {
                    padding: {
                        top: 3,
                        bottom: 3,
                        left: 7,
                        right: 7,
                    }
                },
                hover: {
                    icon: svg(foreground(layer, "on"), "icons/x_mark_16.svg", 16, 16),
                }
            },
        }
    }
}
