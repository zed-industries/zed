import { foreground } from "../style_tree/components"
import { Layer, StyleSets } from "../theme"

export const indicator = ({
    layer,
    color,
}: {
    layer: Layer
    color: StyleSets
}) => ({
    corner_radius: 4,
    padding: 4,
    margin: { top: 12, left: 12 },
    background: foreground(layer, color),
})
