import { withOpacity } from "../theme/color"
import { ColorScheme, Layer, StyleSets } from "../theme/color_scheme"
import {
    background,
    border,
    border_color,
    foreground,
    text,
} from "./components"
import hover_popover from "./hover_popover"

import { build_syntax } from "../theme/syntax"
import { interactive, toggleable } from "../element"

export default function editor(theme: ColorScheme): any {
    const { is_light } = theme

    const layer = theme.highest

    const autocomplete_item = {
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
            text_scale_factor: 0.857,
            header: {
                border: border(layer, {
                    top: true,
                }),
            },
            message: {
                text: text(layer, "sans", styleSet, "default", { size: "sm" }),
                highlight_text: text(layer, "sans", styleSet, "default", {
                    size: "sm",
                    weight: "bold",
                }),
            },
        }
    }

    const syntax = build_syntax(theme)

    return {
        text_color: syntax.primary.color,
        background: background(layer),
        active_line_background: withOpacity(background(layer, "on"), 0.75),
        highlighted_line_background: background(layer, "on"),
        // Inline autocomplete suggestions, Co-pilot suggestions, etc.
        suggestion: syntax.predictive,
        code_actions: {
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

            vertical_scale: 0.55,
        },
        folds: {
            icon_margin_scale: 2.5,
            folded_icon: "icons/chevron_right_8.svg",
            foldable_icon: "icons/chevron_down_8.svg",
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
                text_color: theme.ramps.neutral(0.71).hex(),
                corner_radius_factor: 0.15,
                background: {
                    // Copied from hover_popover highlight
                    default: {
                        color: theme.ramps.neutral(0.5).alpha(0.0).hex(),
                    },

                    hovered: {
                        color: theme.ramps.neutral(0.5).alpha(0.5).hex(),
                    },

                    clicked: {
                        color: theme.ramps.neutral(0.5).alpha(0.7).hex(),
                    },
                },
            },
            foldBackground: foreground(layer, "variant"),
        },
        diff: {
            deleted: is_light
                ? theme.ramps.red(0.5).hex()
                : theme.ramps.red(0.4).hex(),
            modified: is_light
                ? theme.ramps.yellow(0.5).hex()
                : theme.ramps.yellow(0.5).hex(),
            inserted: is_light
                ? theme.ramps.green(0.4).hex()
                : theme.ramps.green(0.5).hex(),
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
        documentHighlightWriteBackground: theme.ramps
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
        selection: theme.players[0],
        whitespace: theme.ramps.neutral(0.5).hex(),
        guestSelections: [
            theme.players[1],
            theme.players[2],
            theme.players[3],
            theme.players[4],
            theme.players[5],
            theme.players[6],
            theme.players[7],
        ],
        autocomplete: {
            background: background(theme.middle),
            corner_radius: 8,
            padding: 4,
            margin: {
                left: -14,
            },
            border: border(theme.middle),
            shadow: theme.popoverShadow,
            matchHighlight: foreground(theme.middle, "accent"),
            item: autocomplete_item,
            hoveredItem: {
                ...autocomplete_item,
                matchHighlight: foreground(
                    theme.middle,
                    "accent",
                    "hovered"
                ),
                background: background(theme.middle, "hovered"),
            },
            selectedItem: {
                ...autocomplete_item,
                matchHighlight: foreground(
                    theme.middle,
                    "accent",
                    "active"
                ),
                background: background(theme.middle, "active"),
            },
        },
        diagnosticHeader: {
            background: background(theme.middle),
            icon_widthFactor: 1.5,
            textScaleFactor: 0.857,
            border: border(theme.middle, {
                bottom: true,
                top: true,
            }),
            code: {
                ...text(theme.middle, "mono", { size: "sm" }),
                margin: {
                    left: 10,
                },
            },
            source: {
                text: text(theme.middle, "sans", {
                    size: "sm",
                    weight: "bold",
                }),
            },
            message: {
                highlightText: text(theme.middle, "sans", {
                    size: "sm",
                    weight: "bold",
                }),
                text: text(theme.middle, "sans", { size: "sm" }),
            },
        },
        diagnosticPathHeader: {
            background: background(theme.middle),
            textScaleFactor: 0.857,
            filename: text(theme.middle, "mono", { size: "sm" }),
            path: {
                ...text(theme.middle, "mono", { size: "sm" }),
                margin: {
                    left: 12,
                },
            },
        },
        errorDiagnostic: diagnostic(theme.middle, "negative"),
        warningDiagnostic: diagnostic(theme.middle, "warning"),
        informationDiagnostic: diagnostic(theme.middle, "accent"),
        hintDiagnostic: diagnostic(theme.middle, "warning"),
        invalidErrorDiagnostic: diagnostic(theme.middle, "base"),
        invalidHintDiagnostic: diagnostic(theme.middle, "base"),
        invalidInformationDiagnostic: diagnostic(theme.middle, "base"),
        invalidWarningDiagnostic: diagnostic(theme.middle, "base"),
        hover_popover: hover_popover(theme),
        linkDefinition: {
            color: syntax.link_uri.color,
            underline: syntax.link_uri.underline,
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
                    ? withOpacity(theme.ramps.red(0.5).hex(), 0.8)
                    : withOpacity(theme.ramps.red(0.4).hex(), 0.8),
                modified: is_light
                    ? withOpacity(theme.ramps.yellow(0.5).hex(), 0.8)
                    : withOpacity(theme.ramps.yellow(0.4).hex(), 0.8),
                inserted: is_light
                    ? withOpacity(theme.ramps.green(0.5).hex(), 0.8)
                    : withOpacity(theme.ramps.green(0.4).hex(), 0.8),
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
