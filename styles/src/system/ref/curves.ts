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
