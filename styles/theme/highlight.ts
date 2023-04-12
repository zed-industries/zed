import { Border } from "@/theme/border"

interface BackgroundHighlight {
    color: string
    borderRadius: string
}

type OutlineHighlight = Border

type HighlightStyle = BackgroundHighlight | OutlineHighlight

export interface Highlight {
    background: string
    outline: HighlightStyle
}
