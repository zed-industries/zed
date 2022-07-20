import Theme from "../themes/common/theme";
import { backgroundColor, border, popoverShadow, text } from "./components";

export default function HoverPopover(theme: Theme) {
  let baseContainer = {
    background: backgroundColor(theme, "on500"),
    cornerRadius: 8,
    padding: {
      left: 8,
      right: 8,
      top: 4,
      bottom: 4
    },
    shadow: popoverShadow(theme),
    border: border(theme, "secondary"),
    margin: {
      left: -8,
    },
  };

  return {
    container: baseContainer,
    infoContainer: {
      ...baseContainer,
      background: backgroundColor(theme, "on500Info"),
      border: {
        color: theme.ramps.blue(0).hex(),
        width: 1,
      },
    },
    warningContainer: {
      ...baseContainer,
      background: backgroundColor(theme, "on500Warning"),
      border: {
        color: theme.ramps.yellow(0).hex(),
        width: 1,
      },
    },
    errorContainer: {
      ...baseContainer,
      background: backgroundColor(theme, "on500Error"),
      border: {
        color: theme.ramps.red(0).hex(),
        width: 1,
      }
    },
    block_style: {
      padding: { top: 4 },
    },
    prose: text(theme, "sans", "primary", { size: "sm" }),
    highlight: theme.editor.highlight.occurrence,
  };
}
