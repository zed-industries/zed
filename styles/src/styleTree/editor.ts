import Theme from "../themes/common/theme";
import { withOpacity } from "../utils/color";
import {
  backgroundColor,
  border,
  borderColor,
  iconColor,
  player,
  popoverShadow,
  text,
  textColor,
  TextColor,
} from "./components";
import hoverPopover from "./hoverPopover";

export default function editor(theme: Theme) {
  const autocompleteItem = {
    cornerRadius: 6,
    padding: {
      bottom: 2,
      left: 6,
      right: 6,
      top: 2,
    },
  };

  function diagnostic(theme: Theme, color: TextColor) {
    return {
      textScaleFactor: 0.857,
      header: {
        border: border(theme, "primary", {
          top: true,
        }),
      },
      message: {
        text: text(theme, "sans", color, { size: "sm" }),
        highlightText: text(theme, "sans", color, {
          size: "sm",
          weight: "bold",
        }),
      },
    };
  }

  const syntax: any = {};
  for (const syntaxKey in theme.syntax) {
    const style = theme.syntax[syntaxKey];
    syntax[syntaxKey] = {
      color: style.color,
      weight: style.weight,
      underline: style.underline,
      italic: style.italic,
    };
  }

  return {
    textColor: theme.syntax.primary.color,
    background: backgroundColor(theme, 500),
    activeLineBackground: theme.editor.line.active,
    codeActions: {
      indicator: iconColor(theme, "secondary"),
      verticalScale: 0.618
    },
    diff: {
      deleted: theme.iconColor.error,
      inserted: theme.iconColor.ok,
      modified: theme.iconColor.warning,
      removedWidthEm: 0.275,
      widthEm: 0.16,
      cornerRadius: 0.05,
    },
    documentHighlightReadBackground: theme.editor.highlight.occurrence,
    documentHighlightWriteBackground: theme.editor.highlight.activeOccurrence,
    errorColor: theme.textColor.error,
    gutterBackground: backgroundColor(theme, 500),
    gutterPaddingFactor: 3.5,
    highlightedLineBackground: theme.editor.line.highlighted,
    lineNumber: theme.editor.gutter.primary,
    lineNumberActive: theme.editor.gutter.active,
    renameFade: 0.6,
    unnecessaryCodeFade: 0.5,
    selection: player(theme, 1).selection,
    guestSelections: [
      player(theme, 2).selection,
      player(theme, 3).selection,
      player(theme, 4).selection,
      player(theme, 5).selection,
      player(theme, 6).selection,
      player(theme, 7).selection,
      player(theme, 8).selection,
    ],
    autocomplete: {
      background: backgroundColor(theme, 500),
      cornerRadius: 8,
      padding: 4,
      border: border(theme, "secondary"),
      shadow: popoverShadow(theme),
      item: autocompleteItem,
      hoveredItem: {
        ...autocompleteItem,
        background: backgroundColor(theme, 500, "hovered"),
      },
      margin: {
        left: -14,
      },
      matchHighlight: text(theme, "mono", "feature"),
      selectedItem: {
        ...autocompleteItem,
        background: backgroundColor(theme, 500, "active"),
      },
    },
    diagnosticHeader: {
      background: backgroundColor(theme, 300),
      iconWidthFactor: 1.5,
      textScaleFactor: 0.857, // NateQ: Will we need dynamic sizing for text? If so let's create tokens for these.
      border: border(theme, "secondary", {
        bottom: true,
        top: true,
      }),
      code: {
        ...text(theme, "mono", "secondary", { size: "sm" }),
        margin: {
          left: 10,
        },
      },
      message: {
        highlightText: text(theme, "sans", "primary", {
          size: "sm",
          weight: "bold",
        }),
        text: text(theme, "sans", "secondary", { size: "sm" }),
      },
    },
    diagnosticPathHeader: {
      background: theme.editor.line.active,
      textScaleFactor: 0.857,
      filename: text(theme, "mono", "primary", { size: "sm" }),
      path: {
        ...text(theme, "mono", "muted", { size: "sm" }),
        margin: {
          left: 12,
        },
      },
    },
    errorDiagnostic: diagnostic(theme, "error"),
    warningDiagnostic: diagnostic(theme, "warning"),
    informationDiagnostic: diagnostic(theme, "info"),
    hintDiagnostic: diagnostic(theme, "info"),
    invalidErrorDiagnostic: diagnostic(theme, "secondary"),
    invalidHintDiagnostic: diagnostic(theme, "secondary"),
    invalidInformationDiagnostic: diagnostic(theme, "secondary"),
    invalidWarningDiagnostic: diagnostic(theme, "secondary"),
    hoverPopover: hoverPopover(theme),
    linkDefinition: {
      color: theme.syntax.linkUri.color,
      underline: theme.syntax.linkUri.underline,
    },
    jumpIcon: {
      color: iconColor(theme, "secondary"),
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
        color: iconColor(theme, "active"),
        background: backgroundColor(theme, "on500"),
      },
    },
    scrollbar: {
      width: 12,
      minHeightFactor: 1.0,
      track: {
        border: {
          left: true,
          width: 1,
          color: borderColor(theme, "secondary"),
        },
      },
      thumb: {
        background: withOpacity(borderColor(theme, "secondary"), 0.5),
        border: {
          width: 1,
          color: withOpacity(borderColor(theme, 'muted'), 0.5),
        }
      }
    },
    compositionMark: {
      underline: {
        thickness: 1.0,
        color: borderColor(theme, "active")
      },
    },
    syntax,
  };
}
