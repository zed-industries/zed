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
        outLinkIcon: {
            icon: svg(foreground(layer, "variant"), "icons/maybe_link_out.svg", 12, 12),
            container: {
                cornerRadius: 6,
                padding: { top: 6, bottom: 6, left: 6, right: 6 },
            },
            hover: {
                icon: svg(foreground(layer, "hovered"), "icons/maybe_link_out.svg", 12, 12)
            },
        },
        modal: {
            titleText: {
                ...text(layer, "sans", { size: "md", color: background(layer, "default") }),
                active: {
                    ...text(layer, "sans", { size: "md" }),
                }
            },
            titlebar: {
                border: border(layer, "active"),
                padding: {
                    top: 8,
                    bottom: 8,
                    left: 8,
                    right: 8,
                },
                margin: {
                    top: 0,
                    left: 0,
                    right: 0,
                    bottom: 16
                }
            },
            container: {
                background: background(colorScheme.highest),

            },
            closeIcon: {
                icon: svg(background(layer, "on"), "icons/x_mark_16.svg", 16, 16),
                container: {
                    cornerRadius: 2,
                    padding: {
                        top: 3,
                        bottom: 3,
                        left: 7,
                        right: 0,
                    }
                },
                active: {
                    icon: svg(foreground(colorScheme.lowest, "warning"), "icons/x_mark_16.svg", 16, 16),
                },
                hoverAndActive: {
                    icon: svg(foreground(layer, "on", "hovered"), "icons/x_mark_16.svg", 16, 16),
                },
                clickedAndactive: {
                    icon: svg(foreground(layer, "on", "pressed"), "icons/x_mark_16.svg", 16, 16),
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
            copilotIcon: svg(foreground(layer, "default"), "icons/github-copilot-dummy.svg", 32, 32),
            plusIcon: {
                icon: svg(foreground(layer, "default"), "icons/plus_12.svg", 12, 12),
                container: {
                    padding: {
                        top: 12,
                        bottom: 12,
                        left: 12,
                        right: 12,
                    }
                }
            },
            zedIcon: svg(foreground(layer, "default"), "icons/logo_96.svg", 32, 32),
            enableText: text(layer, "sans", { size: "md" }),
            enableGroup: {
                margin: {
                    top: 5,
                    bottom: 5,
                    left: 0,
                    right: 0
                }
            },

            instructionText: text(layer, "sans"),

            deviceCodeGroup: {
                margin: {
                    top: 20,
                    bottom: 20,
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
            hint: {
                ...text(layer, "sans", { size: "xs" }),
                margin: {
                    top: -5,
                }
            },
            enabledHint: {
                margin: {
                    top: 10,
                    bottom: 10
                }
            },
            notAuthorizedHint: {
                margin: {
                    top: 10,
                    bottom: 10
                }
            },

            warning: {
                ...text(layer, "sans", { size: "md", color: foreground(layer, "warning") }),
                border: border(layer, "warning"),
                background_color: background(layer, "warning"),
                cornerRadius: 2,
            },

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
