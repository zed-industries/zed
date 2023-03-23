import { ColorScheme } from "../themes/common/colorScheme"
import { text } from "./components";


export default function copilot(colorScheme: ColorScheme) {
    let layer = colorScheme.highest;

    return {
        authModal: text(layer, "sans")
    }
}
