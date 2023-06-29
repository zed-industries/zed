import { withOpacity } from "../theme/color"
import { ColorScheme, Layer, StyleSets } from "../theme/color_scheme"
import {
    background,
    border,
    border_color,
    foreground,
    text,
} from "./components"
import hoverPopover from "./hover_popover"

import { buildSyntax } from "../theme/syntax"
import { interactive, toggleable } from "../element"

export default function editor(colorScheme: ColorScheme): any {
    const { is_light } = colorScheme

    const layer = colorScheme.highest

    const autocompleteItem = {
        corner_radius: 6,
        padding: {
            bottom: 2,
            left: 6,
            right: 6,
            top: 2,
        },
    }

    function diagnostic(layer: Layer, styleSet: StyleSets) {
        return {
            textScaleFactor: 0.857,
            header: {
                border: border(layer, {
                    top: true,
                }),
            },
            message: {
                text: text(layer, "sans", styleSet, "default", { size: "sm" }),
                highlightText: text(layer, "sans", styleSet, "default", {
                    size: "sm",
                    weight: "bold",
                }),
            },
        }
    }

    const syntax = buildSyntax(colorScheme)

    return {
        textColor: syntax.primary.color,
        background: background(layer),
        activeLineBackground: withOpacity(background(layer, "on"), 0.75),
        highlightedLineBackground: background(layer, "on"),
        // Inline autocomplete suggestions, Co-pilot suggestions, etc.
        suggestion: syntax.predictive,
        codeActions: {
            indicator: toggleable({
                base: interactive({
                    base: {
                        color: foreground(layer, "variant"),
                    },
                    state: {
                        hovered: {
                            color: foreground(layer, "variant", "hovered"),
                        },
                        clicked: {
                            color: foreground(layer, "variant", "pressed"),
                        },
                    },
                }),
                state: {
                    active: {
                        default: {
                            color: foreground(layer, "accent"),
                        },
                        hovered: {
                            color: foreground(layer, "accent", "hovered"),
                        },
                        clicked: {
                            color: foreground(layer, "accent", "pressed"),
                        },
                    },
                },
            }),

            verticalScale: 0.55,
        },
        folds: {
            iconMarginScale: 2.5,
            foldedIcon: "icons/chevron_right_8.svg",
            foldableIcon: "icons/chevron_down_8.svg",
            indicator: toggleable({
                base: interactive({
                    base: {
                        color: foreground(layer, "variant"),
                    },
                    state: {
                        hovered: {
                            color: foreground(layer, "on"),
                        },
                        clicked: {
                            color: foreground(layer, "base"),
                        },
                    },
                }),
                state: {
                    active: {
                        default: {
                            color: foreground(layer, "default"),
                        },
                        hovered: {
                            color: foreground(layer, "variant"),
                        },
                    },
                },
            }),
            ellipses: {
                textColor: colorScheme.ramps.neutral(0.71).hex(),
                corner_radiusFactor: 0.15,
                background: {
                    // Copied from hover_popover highlight
                    default: {
                        color: colorScheme.ramps.neutral(0.5).alpha(0.0).hex(),
                    },

                    hovered: {
                        color: colorScheme.ramps.neutral(0.5).alpha(0.5).hex(),
                    },

                    clicked: {
                        color: colorScheme.ramps.neutral(0.5).alpha(0.7).hex(),
                    },
                },
            },
            foldBackground: foreground(layer, "variant"),
        },
        diff: {
            deleted: is_light
                ? colorScheme.ramps.red(0.5).hex()
                : colorScheme.ramps.red(0.4).hex(),
            modified: is_light
                ? colorScheme.ramps.yellow(0.5).hex()
                : colorScheme.ramps.yellow(0.5).hex(),
            inserted: is_light
                ? colorScheme.ramps.green(0.4).hex()
                : colorScheme.ramps.green(0.5).hex(),
            removedWidthEm: 0.275,
            widthEm: 0.15,
            corner_radius: 0.05,
        },
        /** Highlights matching occurrences of what is under the cursor
         * as well as matched brackets
         */
        documentHighlightReadBackground: withOpacity(
            foreground(layer, "accent"),
            0.1
        ),
        documentHighlightWriteBackground: colorScheme.ramps
            .neutral(0.5)
            .alpha(0.4)
            .hex(), // TODO: This was blend * 2
        errorColor: background(layer, "negative"),
        gutterBackground: background(layer),
        gutterPaddingFactor: 3.5,
        lineNumber: withOpacity(foreground(layer), 0.35),
        lineNumberActive: foreground(layer),
        renameFade: 0.6,
        unnecessaryCodeFade: 0.5,
        selection: colorScheme.players[0],
        whitespace: colorScheme.ramps.neutral(0.5).hex(),
        guestSelections: [
            colorScheme.players[1],
            colorScheme.players[2],
            colorScheme.players[3],
            colorScheme.players[4],
            colorScheme.players[5],
            colorScheme.players[6],
            colorScheme.players[7],
        ],
        autocomplete: {
            background: background(colorScheme.middle),
            corner_radius: 8,
            padding: 4,
            margin: {
                left: -14,
            },
            border: border(colorScheme.middle),
            shadow: colorScheme.popoverShadow,
            matchHighlight: foreground(colorScheme.middle, "accent"),
            item: autocompleteItem,
            hoveredItem: {
                ...autocompleteItem,
                matchHighlight: foreground(
                    colorScheme.middle,
                    "accent",
                    "hovered"
                ),
                background: background(colorScheme.middle, "hovered"),
            },
            selectedItem: {
                ...autocompleteItem,
                matchHighlight: foreground(
                    colorScheme.middle,
                    "accent",
                    "active"
                ),
                background: background(colorScheme.middle, "active"),
            },
        },
        diagnosticHeader: {
            background: background(colorScheme.middle),
            icon_widthFactor: 1.5,
            textScaleFactor: 0.857,
            border: border(colorScheme.middle, {
                bottom: true,
                top: true,
            }),
            code: {
                ...text(colorScheme.middle, "mono", { size: "sm" }),
                margin: {
                    left: 10,
                },
            },
            source: {
                text: text(colorScheme.middle, "sans", {
                    size: "sm",
                    weight: "bold",
                }),
            },
            message: {
                highlightText: text(colorScheme.middle, "sans", {
                    size: "sm",
                    weight: "bold",
                }),
                text: text(colorScheme.middle, "sans", { size: "sm" }),
            },
        },
        diagnosticPathHeader: {
            background: background(colorScheme.middle),
            textScaleFactor: 0.857,
            filename: text(colorScheme.middle, "mono", { size: "sm" }),
            path: {
                ...text(colorScheme.middle, "mono", { size: "sm" }),
                margin: {
                    left: 12,
                },
            },
        },
        errorDiagnostic: diagnostic(colorScheme.middle, "negative"),
        warningDiagnostic: diagnostic(colorScheme.middle, "warning"),
        informationDiagnostic: diagnostic(colorScheme.middle, "accent"),
        hintDiagnostic: diagnostic(colorScheme.middle, "warning"),
        invalidErrorDiagnostic: diagnostic(colorScheme.middle, "base"),
        invalidHintDiagnostic: diagnostic(colorScheme.middle, "base"),
        invalidInformationDiagnostic: diagnostic(colorScheme.middle, "base"),
        invalidWarningDiagnostic: diagnostic(colorScheme.middle, "base"),
        hoverPopover: hoverPopover(colorScheme),
        linkDefinition: {
            color: syntax.linkUri.color,
            underline: syntax.linkUri.underline,
        },
        jumpIcon: interactive({
            base: {
                color: foreground(layer, "on"),
                icon_width: 20,
                button_width: 20,
                corner_radius: 6,
                padding: {
                    top: 6,
                    bottom: 6,
                    left: 6,
                    right: 6,
                },
            },
            state: {
                hovered: {
                    background: background(layer, "on", "hovered"),
                },
            },
        }),

        scrollbar: {
            width: 12,
            minHeightFactor: 1.0,
            track: {
                border: border(layer, "variant", { left: true }),
            },
            thumb: {
                background: withOpacity(background(layer, "inverted"), 0.3),
                border: {
                    width: 1,
                    color: border_color(layer, "variant"),
                    top: false,
                    right: true,
                    left: true,
                    bottom: false,
                },
            },
            git: {
                deleted: is_light
                    ? withOpacity(colorScheme.ramps.red(0.5).hex(), 0.8)
                    : withOpacity(colorScheme.ramps.red(0.4).hex(), 0.8),
                modified: is_light
                    ? withOpacity(colorScheme.ramps.yellow(0.5).hex(), 0.8)
                    : withOpacity(colorScheme.ramps.yellow(0.4).hex(), 0.8),
                inserted: is_light
                    ? withOpacity(colorScheme.ramps.green(0.5).hex(), 0.8)
                    : withOpacity(colorScheme.ramps.green(0.4).hex(), 0.8),
            },
        },
        compositionMark: {
            underline: {
                thickness: 1.0,
                color: border_color(layer),
            },
        },
        syntax,
    }
}
