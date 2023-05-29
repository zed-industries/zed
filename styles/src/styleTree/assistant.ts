import { ColorScheme } from "../themes/common/colorScheme"
import { text, border } from "./components"
import editor from "./editor"

export default function assistant(colorScheme: ColorScheme) {
    const layer = colorScheme.highest;
    return {
      container: {
        background: editor(colorScheme).background,
        padding: { left: 12 }
      },
      header: {
        border: border(layer, "default", { bottom: true, top: true }),
        margin: { bottom: 6, top: 6 }
      },
      user_sender: {
        ...text(layer, "sans", "default", { size: "sm", weight: "bold" }),
      },
      assistant_sender: {
        ...text(layer, "sans", "accent", { size: "sm", weight: "bold" }),
      },
      sent_at: {
        margin: { top: 2, left: 8 },
        ...text(layer, "sans", "default", { size: "2xs" }),
      }
    }
}
