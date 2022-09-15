import Theme from "../themes/common/theme";
import { backgroundColor } from "./components";

export default function workspace(theme: Theme) {
  return {
    background: backgroundColor(theme, 300),
  }
}
