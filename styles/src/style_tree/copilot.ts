import { ColorScheme } from "../theme/color_scheme"
import { background, border, foreground, svg, text } from "./components"
import { interactive } from "../element"
export default function copilot(theme: ColorScheme): any {
    const content_width = 264

    const cta_button =
        // Copied from welcome screen. FIXME: Move this into a ZDS component
        interactive({
            base: {
                background: background(theme.middle),
                border: border(theme.middle, "default"),
                corner_radius: 4,
                margin: {
                    top: 4,
                    bottom: 4,
                    left: 8,
                    right: 8,
                },
                padding: {
                    top: 3,
                    bottom: 3,
                    left: 7,
                    right: 7,
                },
                ...text(theme.middle, "sans", "default", { size: "sm" }),
            },
            state: {
                hovered: {
                    ...text(theme.middle, "sans", "default", { size: "sm" }),
                    background: background(theme.middle, "hovered"),
                    border: border(theme.middle, "active"),
                },
            },
        })

    return {
        out_link_icon: interactive({
            base: {
                icon: svg(
                    foreground(theme.middle, "variant"),
                    "icons/link_out_12.svg",
                    12,
                    12
                ),
                container: {
                    corner_radius: 6,
                    padding: { left: 6 },
                },
            },
            state: {
                hovered: {
                    icon: {
                        color: foreground(theme.middle, "hovered"),
                    },
                },
            },
        }),

        modal: {
            title_text: {
                default: {
                    ...text(theme.middle, "sans", {
                        size: "xs",
                        weight: "bold",
                    }),
                },
            },
            titlebar: {
                background: background(theme.lowest),
                border: border(theme.middle, "active"),
                padding: {
                    top: 4,
                    bottom: 4,
                    left: 8,
                    right: 8,
                },
            },
            container: {
                background: background(theme.lowest),
                padding: {
                    top: 0,
                    left: 0,
                    right: 0,
                    bottom: 8,
                },
            },
            close_icon: interactive({
                base: {
                    icon: svg(
                        foreground(theme.middle, "variant"),
                        "icons/x_mark_8.svg",
                        8,
                        8
                    ),
                    container: {
                        corner_radius: 2,
                        padding: {
                            top: 4,
                            bottom: 4,
                            left: 4,
                            right: 4,
                        },
                        margin: {
                            right: 0,
                        },
                    },
                },
                state: {
                    hovered: {
                        icon: svg(
                            foreground(theme.middle, "on"),
                            "icons/x_mark_8.svg",
                            8,
                            8
                        ),
                    },
                    clicked: {
                        icon: svg(
                            foreground(theme.middle, "base"),
                            "icons/x_mark_8.svg",
                            8,
                            8
                        ),
                    },
                },
            }),
            dimensions: {
                width: 280,
                height: 280,
            },
        },

        auth: {
            content_width,

            cta_button,

            header: {
                icon: svg(
                    foreground(theme.middle, "default"),
                    "icons/zed_plus_copilot_32.svg",
                    92,
                    32
                ),
                container: {
                    margin: {
                        top: 35,
                        bottom: 5,
                        left: 0,
                        right: 0,
                    },
                },
            },

            prompting: {
                subheading: {
                    ...text(theme.middle, "sans", { size: "xs" }),
                    margin: {
                        top: 6,
                        bottom: 12,
                        left: 0,
                        right: 0,
                    },
                },

                hint: {
                    ...text(theme.middle, "sans", {
                        size: "xs",
                        color: "#838994",
                    }),
                    margin: {
                        top: 6,
                        bottom: 2,
                    },
                },

                device_code: {
                    text: text(theme.middle, "mono", { size: "sm" }),
                    cta: {
                        ...cta_button,
                        background: background(theme.lowest),
                        border: border(theme.lowest, "inverted"),
                        padding: {
                            top: 0,
                            bottom: 0,
                            left: 16,
                            right: 16,
                        },
                        margin: {
                            left: 16,
                            right: 16,
                        },
                    },
                    left: content_width / 2,
                    left_container: {
                        padding: {
                            top: 3,
                            bottom: 3,
                            left: 0,
                            right: 6,
                        },
                    },
                    right: (content_width * 1) / 3,
                    right_container: interactive({
                        base: {
                            border: border(theme.lowest, "inverted", {
                                bottom: false,
                                right: false,
                                top: false,
                                left: true,
                            }),
                            padding: {
                                top: 3,
                                bottom: 5,
                                left: 8,
                                right: 0,
                            },
                        },
                        state: {
                            hovered: {
                                border: border(theme.middle, "active", {
                                    bottom: false,
                                    right: false,
                                    top: false,
                                    left: true,
                                }),
                            },
                        },
                    }),
                },
            },

            not_authorized: {
                subheading: {
                    ...text(theme.middle, "sans", { size: "xs" }),

                    margin: {
                        top: 16,
                        bottom: 16,
                        left: 0,
                        right: 0,
                    },
                },

                warning: {
                    ...text(theme.middle, "sans", {
                        size: "xs",
                        color: foreground(theme.middle, "warning"),
                    }),
                    border: border(theme.middle, "warning"),
                    background: background(theme.middle, "warning"),
                    corner_radius: 2,
                    padding: {
                        top: 4,
                        left: 4,
                        bottom: 4,
                        right: 4,
                    },
                    margin: {
                        bottom: 16,
                        left: 8,
                        right: 8,
                    },
                },
            },

            authorized: {
                subheading: {
                    ...text(theme.middle, "sans", { size: "xs" }),

                    margin: {
                        top: 16,
                        bottom: 16,
                    },
                },

                hint: {
                    ...text(theme.middle, "sans", {
                        size: "xs",
                        color: "#838994",
                    }),
                    margin: {
                        top: 24,
                        bottom: 4,
                    },
                },
            },
        },
    }
}
