import { ColorScheme } from "../common";
import { interactive } from "../element";
import { background, foreground } from "../styleTree/components";
import { Margin } from "../types/zed";

interface IconButtonOptions {
    color?: keyof ColorScheme['lowest'];
    margin?: Partial<Margin>;
}

export function icon_button(theme: ColorScheme, { color, margin }: IconButtonOptions) {
    if (!color)
        color = "base";

    const m = {
        top: margin?.top ?? 0,
        bottom: margin?.bottom ?? 0,
        left: margin?.left ?? 0,
        right: margin?.right ?? 0,
    }

    return interactive({
        base: {
            corner_radius: 4,
            padding: {
                top: 2,
                bottom: 2,
                left: 4,
                right: 4,
            },
            margin: m,
            icon_width: 15,
            icon_height: 15,
            button_width: 23,
            button_height: 19,
        },
        state: {
            default: {
                background: background(theme.lowest, color),
                color: foreground(theme.lowest, color),
            },
            hovered: {
                background: background(theme.lowest, color, "hovered"),
                color: foreground(theme.lowest, color, "hovered"),

            },
            clicked: {
                background: background(theme.lowest, color, "pressed"),
                color: foreground(theme.lowest, color, "pressed"),

            },
        },
    });
}
