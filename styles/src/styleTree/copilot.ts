import { ColorScheme } from "../themes/common/colorScheme"
import { background, border, foreground, svg, text } from "./components";


export default function copilot(colorScheme: ColorScheme) {
    let layer = colorScheme.highest;

    let content_width = 304;

    let ctaButton = { // Copied from welcome screen. FIXME: Move this into a ZDS component
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
    };

    return {
        modal: {
            titleText: text(layer, "sans", { size: "md" }),
            titlebar: {
                border: border(layer, "active"),
                padding: {
                    top: 4,
                    bottom: 4,
                    left: 8,
                    right: 8,
                },
                margin: {
                    top: 0,
                    left: 0,
                    right: 0,
                    bottom: 8
                }
            },
            container: {
                background: background(colorScheme.highest),

            },
            closeIcon: {
                icon: svg(background(layer, "on"), "icons/x_mark_16.svg", 16, 16),
                container: {
                    padding: {
                        top: 3,
                        bottom: 3,
                        left: 7,
                        right: 0,
                    }
                },
                hover: {
                    icon: svg(foreground(layer, "on"), "icons/x_mark_16.svg", 16, 16),
                }
            },
            dimensions: {
                width: 400,
                height: 500,
            },
        },
        auth: {
            content_width,

            headerGroup: {
                margin: {
                    top: 5,
                    bottom: 5,
                    left: 0,
                    right: 0
                }
            },
            headerText: text(layer, "sans", { size: "lg" }),
            copilotIcon: svg(foreground(layer, "default"), "icons/github-copilot-dummy.svg", 36, 36),
            plusIcon: svg(foreground(layer, "default"), "icons/plus_16.svg", 36, 36),
            zedIcon: svg(foreground(layer, "default"), "icons/logo_96.svg", 36, 36),

            instructionText: text(layer, "sans"),

            deviceCodeGroup: {
                margin: {
                    top: 5,
                    bottom: 5,
                    left: 0,
                    right: 0
                }
            },
            deviceCode:
                text(layer, "mono", { size: "md" }),
            deviceCodeCta: {
                ...ctaButton,
                padding: {
                    top: 0,
                    bottom: 0,
                    left: 0,
                    right: 0,
                },
            },
            deviceCodeLeft: content_width * 2 / 3,
            deviceCodeLeftContainer: {
                padding: {
                    top: 3,
                    bottom: 3,
                    left: 0,
                    right: 0,
                },
            },
            deviceCodeRight: content_width * 1 / 3,
            deviceCodeRightContainer: {
                border: border(layer, "active", { bottom: false, right: false, top: false, left: true }),
                padding: {
                    top: 3,
                    bottom: 5,
                    left: 0,
                    right: 0,
                },
            },
            deviceCodeSeperatorHeight: 0,

            githubGroup: {
                margin: {
                    top: 3,
                    bottom: 3,
                    left: 0,
                    right: 0
                }
            },

            ctaButton
        }
    }
}
