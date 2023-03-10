
import { ColorScheme } from "../themes/common/colorScheme";
import { withOpacity } from "../utils/color";
import { border, background, foreground, text, TextProperties } from "./components";


export default function welcome(colorScheme: ColorScheme) {
    let layer = colorScheme.highest;

    let checkboxBase = {
        cornerRadius: 4,
        padding: {
            left: 3,
            right: 3,
            top: 3,
            bottom: 3,
        },
        // shadow: colorScheme.popoverShadow,
        border: border(layer),
        margin: {
            right: 8,
            top: 5,
            bottom: 5
        },
    };

    let interactive_text_size: TextProperties = { size: "sm" }

    return {
        pageWidth: 320,
        logo: {
            color: foreground(layer, "default"),
            icon: "icons/logo_96.svg",
            dimensions: {
                width: 64,
                height: 64,
            }
        },
        logoSubheading: {
            ...text(layer, "sans", "variant", { size: "md" }),
            margin: {
                top: 10,
                bottom: 7,
            },
        },
        buttonGroup: {
            margin: {
                top: 8,
                bottom: 16
            },
        },
        headingGroup: {
            margin: {
                top: 8,
                bottom: 12
            },
        },
        checkboxGroup: {
            border: border(layer, "variant"),
            background: withOpacity(background(layer, "hovered"), 0.25),
            cornerRadius: 4,
            padding: {
                left: 12,
                top: 2,
                bottom: 2
            },
        },
        button: {
            background: background(layer),
            border: border(layer, "active"),
            cornerRadius: 4,
            margin: {
                top: 4,
                bottom: 4
            },
            padding: {
                top: 3,
                bottom: 3,
                left: 7,
                right: 7,
            },
            ...text(layer, "sans", "default", interactive_text_size),
            hover: {
                ...text(layer, "sans", "default", interactive_text_size),
                background: background(layer, "hovered"),
                border: border(layer, "active"),
            },
        },
        usageNote: {
            ...text(layer, "sans", "variant", { size: "2xs" }),
            padding: {
                top: -4,

            }
        },
        checkboxContainer: {
            margin: {
                top: 4,
            },
            padding: {
                bottom: 8,
            }
        },
        checkbox: {
            label: {
                ...text(layer, "sans", interactive_text_size),
                // Also supports margin, container, border, etc.
            },
            icon: {
                color: foreground(layer, "on"),
                icon: "icons/check_12.svg",
                dimensions: {
                    width: 12,
                    height: 12,
                }
            },
            default: {
                ...checkboxBase,
                background: background(layer, "default"),
                border: border(layer, "active")
            },
            checked: {
                ...checkboxBase,
                background: background(layer, "hovered"),
                border: border(layer, "active")
            },
            hovered: {
                ...checkboxBase,
                background: background(layer, "hovered"),
                border: border(layer, "active")
            },
            hoveredAndChecked: {
                ...checkboxBase,
                background: background(layer, "hovered"),
                border: border(layer, "active")
            }
        }
    }
}
