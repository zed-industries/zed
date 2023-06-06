import { ColorScheme } from "../themes/common/colorScheme"
import { text, border, background } from "./components"
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
        margin: { bottom: 6, top: 6 },
        background: editor(colorScheme).background
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
      },
      model_info_container: {
        margin: { right: 16, top: 4 },
      },
      model: {
        background: background(layer, "on"),
        border: border(layer, "on", { overlay: true }),
        padding: 4,
        cornerRadius: 4,
        ...text(layer, "sans", "default", { size: "xs" }),
        hover: {
          background: background(layer, "on", "hovered"),
        }
      },
      remaining_tokens: {
        background: background(layer, "on"),
        border: border(layer, "on", { overlay: true }),
        padding: 4,
        margin: { left: 4 },
        cornerRadius: 4,
        ...text(layer, "sans", "positive", { size: "xs" }),
      },
      no_remaining_tokens: {
        background: background(layer, "on"),
        border: border(layer, "on", { overlay: true }),
        padding: 4,
        margin: { left: 4 },
        cornerRadius: 4,
        ...text(layer, "sans", "negative", { size: "xs" }),
      },
      apiKeyEditor: {
          background: background(layer, "on"),
          cornerRadius: 6,
          text: text(layer, "mono", "on"),
          placeholderText: text(layer, "mono", "on", "disabled", {
              size: "xs",
          }),
          selection: colorScheme.players[0],
          border: border(layer, "on"),
          padding: {
              bottom: 4,
              left: 8,
              right: 8,
              top: 4,
          },
      },
      apiKeyPrompt: {
        padding: 10,
        ...text(layer, "sans", "default", { size: "xs" }),
      }
    }
}
