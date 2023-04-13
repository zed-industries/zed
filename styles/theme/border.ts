type BorderStyle = "solid" | "dashed" | "dotted" | "double" | "wavy"

export interface Border {
    width: number
    color: string
    style: BorderStyle
    inset: boolean
}
