import picker from "./picker"
import { ColorScheme } from "../theme/color_scheme"
import { background, border, foreground, text } from "./components"

export default function contact_finder(theme: ColorScheme): any {
    const side_margin = 6
    const contact_button = {
        background: background(theme.middle, "variant"),
        color: foreground(theme.middle, "variant"),
        icon_width: 8,
        button_width: 16,
        corner_radius: 8,
    }

    const picker_style = picker(theme)
    const picker_input = {
        background: background(theme.middle, "on"),
        corner_radius: 6,
        text: text(theme.middle, "mono"),
        placeholder_text: text(theme.middle, "mono", "on", "disabled", {
            size: "xs",
        }),
        selection: theme.players[0],
        border: border(theme.middle),
        padding: {
            bottom: 4,
            left: 8,
            right: 8,
            top: 4,
        },
        margin: {
            left: side_margin,
            right: side_margin,
        },
    }

    return {
        picker: {
            empty_container: {},
            item: {
                ...picker_style.item,
                margin: { left: side_margin, right: side_margin },
            },
            no_matches: picker_style.no_matches,
            input_editor: picker_input,
            empty_input_editor: picker_input,
            header: picker_style.header,
            footer: picker_style.footer,
        },
        row_height: 28,
        contact_avatar: {
            corner_radius: 10,
            width: 18,
        },
        contact_username: {
            padding: {
                left: 8,
            },
        },
        contact_button: {
            ...contact_button,
            hover: {
                background: background(theme.middle, "variant", "hovered"),
            },
        },
        disabled_contact_button: {
            ...contact_button,
            background: background(theme.middle, "disabled"),
            color: foreground(theme.middle, "disabled"),
        },
    }
}
