import { ColorScheme } from "../theme/colorScheme"
import { foreground, text } from "./components"
import { interactive } from "../element/interactive"

const headerPadding = 8

export default function updateNotification(colorScheme: ColorScheme): Object {
    let layer = colorScheme.middle
    return {
        message: {
            ...text(layer, "sans", { size: "xs" }),
            margin: { left: headerPadding, right: headerPadding },
        },
        actionMessage: interactive({
            base: {
                ...text(layer, "sans", { size: "xs" }),
                margin: { left: headerPadding, top: 6, bottom: 6 }
            }, state: {
                hovered: {
                    color: foreground(layer, "hovered"),
                }
            }
        }),
        dismissButton: interactive({
            base: {
                color: foreground(layer),
                iconWidth: 8,
                iconHeight: 8,
                buttonWidth: 8,
                buttonHeight: 8
            }, state: {
                hovered: {
                    color: foreground(layer, "hovered"),
                },
            },
        })

    }
}
