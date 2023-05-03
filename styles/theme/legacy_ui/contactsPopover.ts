import { useSurfaceStyle } from "@components/surface"
import { Theme } from "@theme*"
import { ContainerStyle, containerStyle } from "@theme/container"
import { padding } from "@theme/padding"
import { shadow } from "@theme/shadow"

export default function contactsPopover(theme: Theme) {
    const surfaceStyle = useSurfaceStyle(theme, 'popover')

    const container: ContainerStyle = containerStyle({
        background: surfaceStyle.background,
        border: surfaceStyle.border,
        borderRadius: 6,
        width: 300,
        height: 400,
        padding: padding(0, 6),
        shadow: shadow(theme, "popover"),
    })

    const legacy_properties = {
        sidePadding: 12,
    }

    return {
        ...legacy_properties,
        ...container,
    }
}
