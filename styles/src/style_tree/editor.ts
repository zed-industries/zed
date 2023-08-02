import { with_opacity } from "../theme/color"
import { Layer, StyleSets } from "../theme/create_theme"
import {
    background,
    border,
    border_color,
    foreground,
    text,
} from "./components"
import hover_popover from "./hover_popover"

import { interactive, toggleable } from "../element"
import { useTheme } from "../theme"
import chroma from "chroma-js"

export default function editor(): any {
    const theme = useTheme()

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

    function diagnostic(layer: Layer, style_set: StyleSets) {
        return {
            text_scale_factor: 0.857,
            header: {
                border: border(layer, {
                    top: true,
                }),
            },
            message: {
                text: text(layer, "sans", style_set, "default", { size: "sm" }),
                highlight_text: text(layer, "sans", style_set, "default", {
                    size: "sm",
                    weight: "bold",
                }),
            },
        }
    }

    return {
        text_color: theme.syntax.primary.color,
        background: background(layer),
        active_line_background: with_opacity(background(layer, "on"), 0.75),
        highlighted_line_background: background(layer, "on"),
        // Inline autocomplete suggestions, Co-pilot suggestions, etc.
        hint: chroma
            .mix(
                theme.ramps.neutral(0.6).hex(),
                theme.ramps.blue(0.4).hex(),
                0.45,
                "lch"
            )
            .hex(),
        suggestion: chroma
            .mix(
                theme.ramps.neutral(0.4).hex(),
                theme.ramps.blue(0.4).hex(),
                0.45,
                "lch"
            )
            .hex(),
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
                            color: foreground(layer, "on"),
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
            fold_background: foreground(layer, "variant"),
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
            removed_width_em: 0.275,
            width_em: 0.15,
            corner_radius: 0.05,
        },
        /** Highlights matching occurrences of what is under the cursor
         * as well as matched brackets
         */
        document_highlight_read_background: with_opacity(
            foreground(layer, "accent"),
            0.1
        ),
        document_highlight_write_background: theme.ramps
            .neutral(0.5)
            .alpha(0.4)
            .hex(), // TODO: This was blend * 2
        error_color: background(layer, "negative"),
        gutter_background: background(layer),
        gutter_padding_factor: 3.5,
        line_number: with_opacity(foreground(layer), 0.35),
        line_number_active: foreground(layer),
        rename_fade: 0.6,
        wrap_guide: with_opacity(foreground(layer), 0.05),
        active_wrap_guide: with_opacity(foreground(layer), 0.1),
        unnecessary_code_fade: 0.5,
        selection: theme.players[0],
        whitespace: theme.ramps.neutral(0.5).hex(),
        guest_selections: [
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
            shadow: theme.popover_shadow,
            match_highlight: foreground(theme.middle, "accent"),
            item: autocomplete_item,
            hovered_item: {
                ...autocomplete_item,
                match_highlight: foreground(theme.middle, "accent", "hovered"),
                background: background(theme.middle, "hovered"),
            },
            selected_item: {
                ...autocomplete_item,
                match_highlight: foreground(theme.middle, "accent", "active"),
                background: background(theme.middle, "active"),
            },
        },
        diagnostic_header: {
            background: background(theme.middle),
            icon_width_factor: 1.5,
            text_scale_factor: 0.857,
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
                highlight_text: text(theme.middle, "sans", {
                    size: "sm",
                    weight: "bold",
                }),
                text: text(theme.middle, "sans", { size: "sm" }),
            },
        },
        diagnostic_path_header: {
            background: background(theme.middle),
            text_scale_factor: 0.857,
            filename: text(theme.middle, "mono", { size: "sm" }),
            path: {
                ...text(theme.middle, "mono", { size: "sm" }),
                margin: {
                    left: 12,
                },
            },
        },
        error_diagnostic: diagnostic(theme.middle, "negative"),
        warning_diagnostic: diagnostic(theme.middle, "warning"),
        information_diagnostic: diagnostic(theme.middle, "accent"),
        hint_diagnostic: diagnostic(theme.middle, "warning"),
        invalid_error_diagnostic: diagnostic(theme.middle, "base"),
        invalid_hint_diagnostic: diagnostic(theme.middle, "base"),
        invalid_information_diagnostic: diagnostic(theme.middle, "base"),
        invalid_warning_diagnostic: diagnostic(theme.middle, "base"),
        hover_popover: hover_popover(),
        link_definition: {
            color: theme.syntax.link_uri.color,
            underline: theme.syntax.link_uri.underline,
        },
        jump_icon: interactive({
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
            min_height_factor: 1.0,
            track: {
                border: border(layer, "variant", { left: true }),
            },
            thumb: {
                background: with_opacity(background(layer, "inverted"), 0.3),
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
                    ? with_opacity(theme.ramps.red(0.5).hex(), 0.8)
                    : with_opacity(theme.ramps.red(0.4).hex(), 0.8),
                modified: is_light
                    ? with_opacity(theme.ramps.yellow(0.5).hex(), 0.8)
                    : with_opacity(theme.ramps.yellow(0.4).hex(), 0.8),
                inserted: is_light
                    ? with_opacity(theme.ramps.green(0.5).hex(), 0.8)
                    : with_opacity(theme.ramps.green(0.4).hex(), 0.8),
            },
            selections: foreground(layer, "accent"),
        },
        composition_mark: {
            underline: {
                thickness: 1.0,
                color: border_color(layer),
            },
        },
        syntax: theme.syntax,
    }
}
