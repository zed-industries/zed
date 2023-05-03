import { Theme } from "@theme*"
import { popoverContainerStyle } from "@theme/container/popover"
import { margin, padding } from "@theme/properties"
import { shadow } from "@theme/shadow"
import { textStyle } from "@theme/text"

export default function HoverPopover(theme: Theme) {
    const popverOptions = {
        borderRadius: 8,
        padding: padding(8, 4),
        margin: margin(0, 0, 0, -8),
        shadow: shadow(theme, "popover"),
    }

    const diagnosticSourceHighlight = textStyle(theme, {
        underline: true,
        color: "accent"
    })

    const legacy_properties = {
        blockStyle: {
            padding: { top: 4 },
        },
        // Should be a full text style
        diagnosticSourceHighlight: {
            underline: diagnosticSourceHighlight.underline,
            color: diagnosticSourceHighlight.color,
        },
        // TODO: I don't know what this is
        // Make it a bright solid color for now
        highlight: '#0000FF',
    }

    return {
        ...legacy_properties,
        container: popoverContainerStyle({
            theme,
            options: popverOptions
        }),
        infoContainer: popoverContainerStyle({
            theme,
            color: "accent",
            options: popverOptions,
        }),
        warningContainer: popoverContainerStyle({
            theme,
            color: "warning",
            options: popverOptions,
        }),
        errorContainer: popoverContainerStyle({
            theme,
            color: "error",
            options: popverOptions,
        }),
        prose: textStyle(theme),
    }
}
