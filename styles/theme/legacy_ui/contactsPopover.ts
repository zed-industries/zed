import { Theme } from "@theme*"
import { popoverContainerStyle } from "@theme/container/popover"
import { padding } from "@theme/padding"

export default function contactsPopover(theme: Theme) {
    const container = popoverContainerStyle({
        theme,
        options: {
            borderRadius: 6,
            width: 300,
            height: 400,
            padding: padding(0, 6),
        },
    })

    const legacy_properties = {
        sidePadding: 12,
    }

    return {
        ...legacy_properties,
        ...container,
    }
}
