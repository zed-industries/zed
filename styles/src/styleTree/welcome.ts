
import { ColorScheme } from "../themes/common/colorScheme";
import { border } from "./components";

export default function welcome(colorScheme: ColorScheme) {
    let layer = colorScheme.highest;

    // TODO
    let checkbox_base = {
        background: colorScheme.ramps.red(0.5).hex(),
        cornerRadius: 8,
        padding: {
            left: 8,
            right: 8,
            top: 4,
            bottom: 4,
        },
        shadow: colorScheme.popoverShadow,
        border: border(layer),
        margin: {
            left: -8,
        },
    };

    return {
        checkbox: {
            width: 9,
            height: 9,
            unchecked: checkbox_base,
            checked: checkbox_base,
            hovered: checkbox_base
        }
    }
}