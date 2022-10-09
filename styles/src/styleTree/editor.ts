import { fontWeights } from "../common";
import {
  ColorScheme,
  Elevation,
  Layer,
  StyleSets,
} from "../themes/common/colorScheme";
import { withOpacity } from "../utils/color";
import {
  background,
  border,
  borderColor,
  foreground,
  text,
} from "./components";
import hoverPopover from "./hoverPopover";

export default function editor(colorScheme: ColorScheme) {
  let elevation = colorScheme.lowest;
  let layer = elevation.top;

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
        text: text(layer, "sans", styleSet, { size: "sm" }),
        highlightText: text(layer, "sans", styleSet, {
          size: "sm",
          weight: "bold",
        }),
      },
    };
  }

  const syntax = {
    primary: {
      color: elevation.ramps.neutral(1).hex(),
      weight: fontWeights.normal,
    },
    comment: {
      color: elevation.ramps.neutral(0.71).hex(),
      weight: fontWeights.normal,
    },
    punctuation: {
      color: elevation.ramps.neutral(0.86).hex(),
      weight: fontWeights.normal,
    },
    constant: {
      color: elevation.ramps.neutral(0.57).hex(),
      weight: fontWeights.normal,
    },
    keyword: {
      color: elevation.ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    function: {
      color: elevation.ramps.yellow(0.5).hex(),
      weight: fontWeights.normal,
    },
    type: {
      color: elevation.ramps.cyan(0.5).hex(),
      weight: fontWeights.normal,
    },
    constructor: {
      color: elevation.ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    variant: {
      color: elevation.ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    property: {
      color: elevation.ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    enum: {
      color: elevation.ramps.orange(0.5).hex(),
      weight: fontWeights.normal,
    },
    operator: {
      color: elevation.ramps.orange(0.5).hex(),
      weight: fontWeights.normal,
    },
    string: {
      color: elevation.ramps.orange(0.5).hex(),
      weight: fontWeights.normal,
    },
    number: {
      color: elevation.ramps.green(0.5).hex(),
      weight: fontWeights.normal,
    },
    boolean: {
      color: elevation.ramps.green(0.5).hex(),
      weight: fontWeights.normal,
    },
    predictive: {
      color: elevation.ramps.neutral(0.57).hex(),
      weight: fontWeights.normal,
    },
    title: {
      color: elevation.ramps.yellow(0.5).hex(),
      weight: fontWeights.bold,
    },
    emphasis: {
      color: elevation.ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    "emphasis.strong": {
      color: elevation.ramps.blue(0.5).hex(),
      weight: fontWeights.bold,
    },
    linkUri: {
      color: elevation.ramps.green(0.5).hex(),
      weight: fontWeights.normal,
      underline: true,
    },
    linkText: {
      color: elevation.ramps.orange(0.5).hex(),
      weight: fontWeights.normal,
      italic: true,
    },
  };

  return {
    textColor: syntax.primary.color,
    background: background(layer),
    activeLineBackground: background(layer, "on"),
    highlightedLineBackground: background(layer, "on"),
    codeActions: {
      indicator: foreground(layer, "variant"),
      verticalScale: 0.55,
    },
    diffBackgroundDeleted: background(layer, "negative"),
    diffBackgroundInserted: background(layer, "positive"),
    documentHighlightReadBackground: elevation.ramps
      .neutral(0.5)
      .alpha(0.2)
      .hex(), // TODO: This was blend
    documentHighlightWriteBackground: elevation.ramps
      .neutral(0.5)
      .alpha(0.4)
      .hex(), // TODO: This was blend * 2
    errorColor: foreground(layer, "negative"),
    gutterBackground: background(layer),
    gutterPaddingFactor: 3.5,
    lineNumber: foreground(layer, "disabled"),
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
      background: background(elevation.above.top),
      cornerRadius: 8,
      padding: 4,
      margin: {
        left: -14,
      },
      border: border(elevation.above.top),
      shadow: elevation.above.shadow,
      matchHighlight: elevation.above.ramps.blue(0.5).hex(),
      item: autocompleteItem,
      hoveredItem: {
        ...autocompleteItem,
        background: background(elevation.above.top, "hovered"),
      },
      selectedItem: {
        ...autocompleteItem,
        background: withOpacity(background(elevation.above.top, "active"), 0.2),
      },
    },
    diagnosticHeader: {
      background: background(elevation.middle),
      iconWidthFactor: 1.5,
      textScaleFactor: 0.857,
      border: border(elevation.middle, {
        bottom: true,
        top: true,
      }),
      code: {
        ...text(elevation.middle, "mono", { size: "sm" }),
        margin: {
          left: 10,
        },
      },
      message: {
        highlightText: text(elevation.middle, "sans", {
          size: "sm",
          weight: "bold",
        }),
        text: text(elevation.middle, "sans", { size: "sm" }),
      },
    },
    diagnosticPathHeader: {
      background: background(elevation.middle),
      textScaleFactor: 0.857,
      filename: text(elevation.middle, "mono", { size: "sm" }),
      path: {
        ...text(elevation.middle, "mono", { size: "sm" }),
        margin: {
          left: 12,
        },
      },
    },
    errorDiagnostic: diagnostic(elevation.middle, "negative"),
    warningDiagnostic: diagnostic(elevation.middle, "warning"),
    informationDiagnostic: diagnostic(elevation.middle, "info"),
    hintDiagnostic: diagnostic(elevation.middle, "positive"),
    invalidErrorDiagnostic: diagnostic(elevation.middle, "base"),
    invalidHintDiagnostic: diagnostic(elevation.middle, "base"),
    invalidInformationDiagnostic: diagnostic(elevation.middle, "base"),
    invalidWarningDiagnostic: diagnostic(elevation.middle, "base"),
    hoverPopover: hoverPopover(elevation.above),
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
    compositionMark: {
      underline: {
        thickness: 1.0,
        color: borderColor(layer),
      },
    },
    syntax,
  };
}
