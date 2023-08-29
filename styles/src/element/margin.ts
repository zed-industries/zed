type MarginOptions = {
    all?: number
    left?: number
    right?: number
    top?: number
    bottom?: number
}

export type MarginStyle = {
    top: number
    bottom: number
    left: number
    right: number
}

export const margin_style = (options: MarginOptions): MarginStyle => {
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
        throw new Error("Margin must have at least one value")

    return {
        top: top || 0,
        bottom: bottom || 0,
        left: left || 0,
        right: right || 0,
    }
}
