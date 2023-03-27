import { withOpacity } from "../utils/color"
import { ColorScheme, Layer, StyleSets } from "../themes/common/colorScheme"
import { background, border, borderColor, foreground, text } from "./components"
import hoverPopover from "./hoverPopover"

import { buildSyntax } from "../themes/common/syntax"

export default function editor(colorScheme: ColorScheme) {
    let layer = colorScheme.highest

    const autocompleteItem = {
        cornerRadius: 6,
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
        suggestion: {
            color: syntax.predictive.color,
        },
        codeActions: {
            indicator: {
                color: foreground(layer, "variant"),

                clicked: {
                    color: foreground(layer, "base"),
                },
                hover: {
                    color: foreground(layer, "on"),
                },
                active: {
                    color: foreground(layer, "on"),
                },
            },
            verticalScale: 0.55,
        },
        folds: {
            iconMarginScale: 2.5,
            foldedIcon: "icons/chevron_right_8.svg",
            foldableIcon: "icons/chevron_down_8.svg",
            indicator: {
                color: foreground(layer, "variant"),

                clicked: {
                    color: foreground(layer, "base"),
                },
                hover: {
                    color: foreground(layer, "on"),
                },
                active: {
                    color: foreground(layer, "on"),
                },
            },
            ellipses: {
                textColor: colorScheme.ramps.neutral(0.71).hex(),
                cornerRadiusFactor: 0.15,
                background: {
                    // Copied from hover_popover highlight
                    color: colorScheme.ramps.neutral(0.5).alpha(0.0).hex(),

                    hover: {
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
            deleted: foreground(layer, "negative"),
            modified: foreground(layer, "warning"),
            inserted: foreground(layer, "positive"),
            removedWidthEm: 0.275,
            widthEm: 0.16,
            cornerRadius: 0.05,
        },
        /** Highlights matching occurences of what is under the cursor
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
            cornerRadius: 8,
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
            iconWidthFactor: 1.5,
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
        jumpIcon: {
            color: foreground(layer, "on"),
            iconWidth: 20,
            buttonWidth: 20,
            cornerRadius: 6,
            padding: {
                top: 6,
                bottom: 6,
                left: 6,
                right: 6,
            },
            hover: {
                background: background(layer, "on", "hovered"),
            },
        },
        scrollbar: {
            width: 12,
            minHeightFactor: 1.0,
            track: {
                border: border(layer, "variant", { left: true }),
            },
            thumb: {
                background: withOpacity(background(layer, "inverted"), 0.4),
                border: {
                    width: 1,
                    color: borderColor(layer, "variant"),
                },
            },
        },
        compositionMark: {
            underline: {
                thickness: 1.0,
                color: borderColor(layer),
            },
        },
        syntax,
    }
}
