import { fontWeights } from "../common";
import { withOpacity } from "../utils/color";
import {
  ColorScheme,
  Layer,
  StyleSets,
} from "../themes/common/colorScheme";
import {
  background,
  border,
  borderColor,
  foreground,
  text,
} from "./components";
import hoverPopover from "./hoverPopover";

export default function editor(colorScheme: ColorScheme) {
  let layer = colorScheme.highest;

  const autocompleteItem = {
    cornerRadius: 6,
    padding: {
      bottom: 2,
      left: 6,
      right: 6,
      top: 2,
    },
  };

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
    };
  }

  const syntax = {
    primary: {
      color: colorScheme.ramps.neutral(1).hex(),
      weight: fontWeights.normal,
    },
    comment: {
      color: colorScheme.ramps.neutral(0.71).hex(),
      weight: fontWeights.normal,
    },
    punctuation: {
      color: colorScheme.ramps.neutral(0.86).hex(),
      weight: fontWeights.normal,
    },
    constant: {
      color: colorScheme.ramps.green(0.5).hex(),
      weight: fontWeights.normal,
    },
    keyword: {
      color: colorScheme.ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    function: {
      color: colorScheme.ramps.yellow(0.5).hex(),
      weight: fontWeights.normal,
    },
    type: {
      color: colorScheme.ramps.cyan(0.5).hex(),
      weight: fontWeights.normal,
    },
    constructor: {
      color: colorScheme.ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    variant: {
      color: colorScheme.ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    property: {
      color: colorScheme.ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    enum: {
      color: colorScheme.ramps.orange(0.5).hex(),
      weight: fontWeights.normal,
    },
    operator: {
      color: colorScheme.ramps.orange(0.5).hex(),
      weight: fontWeights.normal,
    },
    string: {
      color: colorScheme.ramps.orange(0.5).hex(),
      weight: fontWeights.normal,
    },
    number: {
      color: colorScheme.ramps.green(0.5).hex(),
      weight: fontWeights.normal,
    },
    boolean: {
      color: colorScheme.ramps.green(0.5).hex(),
      weight: fontWeights.normal,
    },
    predictive: {
      color: colorScheme.ramps.neutral(0.57).hex(),
      weight: fontWeights.normal,
    },
    title: {
      color: colorScheme.ramps.yellow(0.5).hex(),
      weight: fontWeights.bold,
    },
    emphasis: {
      color: colorScheme.ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    "emphasis.strong": {
      color: colorScheme.ramps.blue(0.5).hex(),
      weight: fontWeights.bold,
    },
    linkUri: {
      color: colorScheme.ramps.green(0.5).hex(),
      weight: fontWeights.normal,
      underline: true,
    },
    linkText: {
      color: colorScheme.ramps.orange(0.5).hex(),
      weight: fontWeights.normal,
      italic: true,
    },
  };

  return {
    textColor: syntax.primary.color,
    background: background(layer),
    activeLineBackground: withOpacity(background(layer, "on"), 0.75),
    highlightedLineBackground: background(layer, "on"),
    codeActions: {
      indicator: foreground(layer, "variant"),
      verticalScale: 0.55,
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
    documentHighlightReadBackground: withOpacity(foreground(layer, "accent"), 0.1),
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
        matchHighlight: foreground(colorScheme.middle, "accent", "hovered"),
        background: background(colorScheme.middle, "hovered"),
      },
      selectedItem: {
        ...autocompleteItem,
        matchHighlight: foreground(colorScheme.middle, "accent", "active"),
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
        color: foreground(layer, "on", "hovered"),
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
          color: borderColor(layer, 'variant'),
        },
      }
    },
    compositionMark: {
      underline: {
        thickness: 1.0,
        color: borderColor(layer),
      },
    },
    syntax,
  };
}
