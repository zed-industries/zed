import { labelButton } from "@components/button"
import { Theme } from "@theme/config"

export default function feedback(theme: Theme) {
    const legacy_properties = {
        button_margin: 8,
    }

    return {
        ...legacy_properties,
        submit_button: labelButton(theme),
        info_text_default: text(layer, "sans", "default", { size: "xs" }),
        link_text_default: text(layer, "sans", "default", {
            size: "xs",
            underline: true,
        }),
        link_text_hover: text(layer, "sans", "hovered", {
            size: "xs",
            underline: true,
        }),
    }
}
