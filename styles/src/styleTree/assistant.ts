import { ColorScheme } from "../themes/common/colorScheme";
import { text } from "./components";

export const assistant = (colorScheme: ColorScheme) => {
    const layer = colorScheme.highest

    return {
        text: text(layer, "sans", "accent", { size: "2xs" }),
        message: {
            assistant: {
                ...text(layer, "sans", "accent", { size: "2xs" }),
            },
            player: {
                ...text(layer, "sans", "disabled", { size: "2xs" }),
            },
        }
    }
}
