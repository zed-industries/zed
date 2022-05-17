import Theme from "../themes/common/theme";
import {
  backgroundColor,
  text,
} from "./components";

export default function projectDiagnostics(theme: Theme) {
  return {
    background: backgroundColor(theme, 500),
    tabIconSpacing: 4,
    tabIconWidth: 13,
    tabSummarySpacing: 10,
    emptyMessage: text(theme, "sans", "secondary", { size: "md" }),
  }
}
