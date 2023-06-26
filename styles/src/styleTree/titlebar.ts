import { ColorScheme } from "../common";
import { interactive, toggleable } from "../element"
import { background, foreground, text } from "./components";

const titlebarButton = (theme: ColorScheme) => toggleable({
    base: interactive({
        base: {
            cornerRadius: 6,
            height: 24,
            width: 24,
            padding: {
                top: 4,
                bottom: 4,
                left: 4,
                right: 4,
            },
            ...text(theme.lowest, "sans", { size: "xs" }),
            background: background(theme.lowest),
        },
        state: {
            hovered: {
                ...text(theme.lowest, "sans", "hovered", {
                    size: "xs",
                }),
                background: background(theme.lowest, "hovered"),
            },
            clicked: {
                ...text(theme.lowest, "sans", "pressed", {
                    size: "xs",
                }),
                background: background(theme.lowest, "pressed"),
            },
        },
    }),
    state: {
        active: {
            default: {
                ...text(theme.lowest, "sans", "active", { size: "xs" }),
                background: background(theme.middle),
            },
            hovered: {
                ...text(theme.lowest, "sans", "active", { size: "xs" }),
                background: background(theme.middle, "hovered"),
            },
            clicked: {
                ...text(theme.lowest, "sans", "active", { size: "xs" }),
                background: background(theme.middle, "pressed"),
            },
        },
    }
});

/**
* Opens the User Menu when toggled
*
* When logged in shows the user's avatar and a chevron,
* When logged out only shows a chevron.
*/
function userMenuButton(theme: ColorScheme) {
    return {
        userMenu: titlebarButton(theme),
        avatar: {
            icon_width: 16,
            icon_height: 16,
            cornerRadius: 4,
            outerWidth: 10,
            outerCornerRadius: 10
        },
        icon: {
            width: 11,
            height: 11,
            color: foreground(theme.lowest)
        }
    }
}

export function titlebar(theme: ColorScheme) {
    return {
        userMenuButton: userMenuButton(theme)
    }
}
