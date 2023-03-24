import { ColorScheme } from "../themes/common/colorScheme"
import { background, border, text } from "./components";


export default function copilot(colorScheme: ColorScheme) {
    let layer = colorScheme.highest;


    return {
        authModal: {
            background: background(colorScheme.lowest),
            border: border(colorScheme.lowest),
            shadow: colorScheme.modalShadow,
            cornerRadius: 12,
            padding: {
                bottom: 4,
            },
        },
        authText: text(layer, "sans")
    }
}
