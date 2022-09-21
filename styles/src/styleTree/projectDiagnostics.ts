import { ColorScheme } from "../themes/common/colorScheme";
import { background, text } from "./components";

export default function projectDiagnostics(colorScheme: ColorScheme) {
  let layer = colorScheme.lowest.top;
  return {
    background: background(layer),
    tabIconSpacing: 4,
    tabIconWidth: 13,
    tabSummarySpacing: 10,
    emptyMessage: text(layer, "sans", "base", "variant", { size: "md" }),
  };
}
