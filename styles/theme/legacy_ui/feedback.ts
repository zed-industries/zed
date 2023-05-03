import { labelButton } from "@components/button"
import { Theme } from "@theme/config"
import { useInteractiveText, textStyle } from "@theme/text"

export default function feedback(theme: Theme) {
    const link_text = useInteractiveText(theme)
    const info_text = textStyle(theme)

    const legacy_properties = {
        button_margin: 8,
        // Should be info_text
        info_text_default: info_text,
        // Should be link_text

        link_text_default: link_text.default,
        link_text_hover: link_text.hovered,
    }

    return {
        ...legacy_properties,
        submit_button: labelButton(theme),
    }
}
