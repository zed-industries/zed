import { ColorScheme } from "../theme/colorScheme"
import { withOpacity } from "../theme/color"
import { background, border, foreground, text } from "./components"
import { interactive } from "./interactive"
import { toggleable } from "./toggle"

export default function search(colorScheme: ColorScheme) {
  let layer = colorScheme.highest

  // Search input
  const editor = {
    background: background(layer),
    cornerRadius: 8,
    minWidth: 200,
    maxWidth: 500,
    placeholderText: text(layer, "mono", "disabled"),
    selection: colorScheme.players[0],
    text: text(layer, "mono", "default"),
    border: border(layer),
    margin: {
      right: 12,
    },
    padding: {
      top: 3,
      bottom: 3,
      left: 12,
      right: 8,
    },
  }

  const includeExcludeEditor = {
    ...editor,
    minWidth: 100,
    maxWidth: 250,
  }

  return {
    // TODO: Add an activeMatchBackground on the rust side to differentiate between active and inactive
    matchBackground: withOpacity(foreground(layer, "accent"), 0.4),
    optionButton: toggleable(interactive({
      base: {
        ...text(layer, "mono", "on"),
        background: background(layer, "on"),
        cornerRadius: 6,
        border: border(layer, "on"),
        margin: {
          right: 4,
        },
        padding: {
          bottom: 2,
          left: 10,
          right: 10,
          top: 2,
        },
      }, state: {
        clicked: {
          ...text(layer, "mono", "on", "pressed"),
          background: background(layer, "on", "pressed"),
          border: border(layer, "on", "pressed"),
        },
        hovered: {
          ...text(layer, "mono", "on", "hovered"),
          background: background(layer, "on", "hovered"),
          border: border(layer, "on", "hovered"),
        },
      }
    }), {
      default: {
        ...text(layer, "mono", "on", "inverted"),
        background: background(layer, "on", "inverted"),
        border: border(layer, "on", "inverted"),
      },

    }),
    editor,
    invalidEditor: {
      ...editor,
      border: border(layer, "negative"),
    },
    includeExcludeEditor,
    invalidIncludeExcludeEditor: {
      ...includeExcludeEditor,
      border: border(layer, "negative"),
    },
    matchIndex: {
      ...text(layer, "mono", "variant"),
      padding: {
        left: 6,
      },
    },
    optionButtonGroup: {
      padding: {
        left: 12,
        right: 12,
      },
    },
    includeExcludeInputs: {
      ...text(layer, "mono", "variant"),
      padding: {
        right: 6,
      },
    },
    resultsStatus: {
      ...text(layer, "mono", "on"),
      size: 18,
    },
    dismissButton: interactive({
      base: {
        color: foreground(layer, "variant"),
        iconWidth: 12,
        buttonWidth: 14,
        padding: {
          left: 10,
          right: 10,
        },
      }, state: {
        hovered: {
          color: foreground(layer, "hovered"),
        }
      }
    }),
  }
}
