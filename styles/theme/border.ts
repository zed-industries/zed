type BorderStyle = "solid" | "dashed" | "dotted" | "double" | "wavy"

export interface Border {
    color: string
    style: BorderStyle
    inset: boolean
}
