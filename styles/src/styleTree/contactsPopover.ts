import { ColorScheme } from "../themes/common/colorScheme";
import { background } from "./components";

export default function workspace(colorScheme: ColorScheme) {
  return {
    background: background(colorScheme.lowest.middle),
  }
}
