type PaddingOptions = {
    all?: number
    left?: number
    right?: number
    top?: number
    bottom?: number
}

export type PaddingStyle = {
    top: number
    bottom: number
    left: number
    right: number
}

export const padding_style = (options: PaddingOptions): PaddingStyle => {
    const { all, top, bottom, left, right } = options

    if (all !== undefined)
        return {
            top: all,
            bottom: all,
            left: all,
            right: all,
        }

    if (
        top === undefined &&
        bottom === undefined &&
        left === undefined &&
        right === undefined
    )
        throw new Error("Padding must have at least one value")

    return {
        top: top || 0,
        bottom: bottom || 0,
        left: left || 0,
        right: right || 0,
    }
}
