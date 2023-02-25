
import { ColorScheme } from "../themes/common/colorScheme";
import { border } from "./components";

export default function welcome(colorScheme: ColorScheme) {
    let layer = colorScheme.highest;

    // TODO
    let checkboxBase = {
        cornerRadius: 4,
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
            default: {
                ...checkboxBase,
                background: colorScheme.ramps.blue(0.5).hex(),
            },
            checked: {
                ...checkboxBase,
                background: colorScheme.ramps.red(0.5).hex(),
            },
            hovered: {
                ...checkboxBase,
                background: colorScheme.ramps.blue(0.5).hex(),

                border: {
                    color: colorScheme.ramps.green(0.5).hex(),
                    width: 1,
                }
            },
            hoveredAndChecked: {
                ...checkboxBase,
                background: colorScheme.ramps.red(0.5).hex(),
                border: {
                    color: colorScheme.ramps.green(0.5).hex(),
                    width: 1,
                }
            }
        }
    }
}