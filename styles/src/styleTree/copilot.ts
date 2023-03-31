import { ColorScheme } from "../themes/common/colorScheme"
import { background, border, foreground, svg, text } from "./components";


export default function copilot(colorScheme: ColorScheme) {
    let layer = colorScheme.middle;

    let content_width = 264;

    let ctaButton = { // Copied from welcome screen. FIXME: Move this into a ZDS component
        background: background(layer),
        border: border(layer, "default"),
        cornerRadius: 4,
        margin: {
            top: 4,
            bottom: 4,
            left: 8,
            right: 8
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
            icon: svg(foreground(layer, "variant"), "icons/link_out_12.svg", 12, 12),
            container: {
                cornerRadius: 6,
                padding: { left: 6 },
            },
            hover: {
                icon: svg(foreground(layer, "hovered"), "icons/link_out_12.svg", 12, 12)
            },
        },
        modal: {
            titleText: {
                ...text(layer, "sans", { size: "xs", "weight": "bold" })
            },
            titlebar: {
                background: background(colorScheme.lowest),
                border: border(layer, "active"),
                padding: {
                    top: 4,
                    bottom: 4,
                    left: 8,
                    right: 8,
                }
            },
            container: {
                background: background(colorScheme.lowest),
                padding: {
                    top: 0,
                    left: 0,
                    right: 0,
                    bottom: 8,
                }
            },
            closeIcon: {
                icon: svg(foreground(layer, "variant"), "icons/x_mark_8.svg", 8, 8),
                container: {
                    cornerRadius: 2,
                    padding: {
                        top: 4,
                        bottom: 4,
                        left: 4,
                        right: 4,
                    },
                    margin: {
                        right: 0
                    }
                },
                hover: {
                    icon: svg(foreground(layer, "on"), "icons/x_mark_8.svg", 8, 8),
                },
                clicked: {
                    icon: svg(foreground(layer, "base"), "icons/x_mark_8.svg", 8, 8),
                }
            },
            dimensions: {
                width: 280,
                height: 280,
            },
        },

        auth: {
            content_width,

            ctaButton,

            header: {
                icon: svg(foreground(layer, "default"), "icons/zed_plus_copilot_32.svg", 92, 32),
                container: {
                    margin: {
                        top: 35,
                        bottom: 5,
                        left: 0,
                        right: 0
                    }
                },
            },

            prompting: {
                subheading: {
                    ...text(layer, "sans", { size: "xs" }),
                    margin: {
                        top: 6,
                        bottom: 12,
                        left: 0,
                        right: 0
                    }
                },

                hint: {
                    ...text(layer, "sans", { size: "xs", color: "#838994" }),
                    margin: {
                        top: 6,
                        bottom: 2
                    }
                },

                deviceCode: {
                    text:
                        text(layer, "mono", { size: "sm" }),
                    cta: {
                        ...ctaButton,
                        background: background(colorScheme.lowest),
                        border: border(colorScheme.lowest, "inverted"),
                        padding: {
                            top: 0,
                            bottom: 0,
                            left: 16,
                            right: 16,
                        },
                        margin: {
                            left: 16,
                            right: 16,
                        }
                    },
                    left: content_width / 2,
                    leftContainer: {
                        padding: {
                            top: 3,
                            bottom: 3,
                            left: 0,
                            right: 6,
                        },
                    },
                    right: content_width * 1 / 3,
                    rightContainer: {
                        border: border(colorScheme.lowest, "inverted", { bottom: false, right: false, top: false, left: true }),
                        padding: {
                            top: 3,
                            bottom: 5,
                            left: 8,
                            right: 0,
                        },
                        hover: {
                            border: border(layer, "active", { bottom: false, right: false, top: false, left: true }),
                        },
                    }
                },
            },

            notAuthorized: {
                subheading: {
                    ...text(layer, "sans", { size: "xs" }),

                    margin: {
                        top: 16,
                        bottom: 16,
                        left: 0,
                        right: 0
                    }
                },

                warning: {
                    ...text(layer, "sans", { size: "xs", color: foreground(layer, "warning") }),
                    border: border(layer, "warning"),
                    background: background(layer, "warning"),
                    cornerRadius: 2,
                    padding: {
                        top: 4,
                        left: 4,
                        bottom: 4,
                        right: 4,
                    },
                    margin: {
                        bottom: 16,
                        left: 8,
                        right: 8
                    }
                },
            },

            authorized: {
                subheading: {
                    ...text(layer, "sans", { size: "xs" }),

                    margin: {
                        top: 16,
                        bottom: 16
                    }
                },

                hint: {
                    ...text(layer, "sans", { size: "xs", color: "#838994" }),
                    margin: {
                        top: 24,
                        bottom: 4
                    }
                },

            },
        }
    }
}
