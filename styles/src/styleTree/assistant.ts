import { ColorScheme } from "../themes/common/colorScheme"
import editor from "./editor"

export default function assistant(colorScheme: ColorScheme) {
    return {
      container: {
        background: editor(colorScheme).background,
        padding: {
          left: 10,
        }
      }
    }
}
