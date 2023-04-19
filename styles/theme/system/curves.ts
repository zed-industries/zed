import bezier from "bezier-easing"

export interface Curve {
    name: string
    value: number[]
}

export interface Curves {
    lightness: Curve
    saturation: Curve
    linear: Curve
}

export const curve: Curves = {
    lightness: {
        name: "lightnessCurve",
        value: [0.2, 0, 0.75, 1.0],
    },
    saturation: {
        name: "saturationCurve",
        value: [0.67, 0.6, 0.55, 1.0],
    },
    linear: {
        name: "linear",
        value: [0.5, 0.5, 0.5, 0.5],
    },
}

/**
 * Formats our Curve data structure into a bezier easing function.
 */
export function useCurve(curve: Curve, inverted?: Boolean) {
    if (inverted) {
        return bezier(
            curve.value[3],
            curve.value[2],
            curve.value[1],
            curve.value[0]
        )
    }

    return bezier(
        curve.value[0],
        curve.value[1],
        curve.value[2],
        curve.value[3]
    )
}
