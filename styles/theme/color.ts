import chroma from "chroma-js"
import { Theme, ThemeConfig } from "./config"

export type Color = chroma.Color
export type Scale = chroma.Scale
type UIScales = Record<keyof ThemeConfig["colors"], Scale>

export { chroma }

function buildScaleFromSingleColor(color: Color): Scale {
    // TODO: Do something to generate a ramp from a single color
    const scale = chroma.scale([
        color.darken(1),
        color.darken(0.5),
        color,
        color.brighten(0.5),
        color.brighten(1),
    ])
    return scale
}

export function buildThemeScales(themeConfig: ThemeConfig): UIScales {
    const scales: UIScales = {} as UIScales
    for (const [colorName, colorValue] of Object.entries(themeConfig.colors)) {
        const name = colorName as keyof ThemeConfig["colors"]

        scales[name] = Array.isArray(colorValue)
            ? chroma.scale(colorValue)
            : buildScaleFromSingleColor(chroma(colorValue))
    }

    return scales
}

// Dumb but it works
export type Intensity =
    | 1
    | 2
    | 3
    | 4
    | 5
    | 6
    | 7
    | 8
    | 9
    | 10
    | 11
    | 12
    | 13
    | 14
    | 15
    | 16
    | 17
    | 18
    | 19
    | 20
    | 21
    | 22
    | 23
    | 24
    | 25
    | 26
    | 27
    | 28
    | 29
    | 30
    | 31
    | 32
    | 33
    | 34
    | 35
    | 36
    | 37
    | 38
    | 39
    | 40
    | 41
    | 42
    | 43
    | 44
    | 45
    | 46
    | 47
    | 48
    | 49
    | 50
    | 51
    | 52
    | 53
    | 54
    | 55
    | 56
    | 57
    | 58
    | 59
    | 60
    | 61
    | 62
    | 63
    | 64
    | 65
    | 66
    | 67
    | 68
    | 69
    | 70
    | 71
    | 72
    | 73
    | 74
    | 75
    | 76
    | 77
    | 78
    | 79
    | 80
    | 81
    | 82
    | 83
    | 84
    | 85
    | 86
    | 87
    | 88
    | 89
    | 90
    | 91
    | 92
    | 93
    | 94
    | 95
    | 96
    | 97
    | 98
    | 99
    | 100

export type UIColors = Record<keyof Theme["color"], Intensity>
export type UIColor = keyof UIColors

export function color(
    theme: Theme,
    color: UIColor,
    intensity: Intensity
): string {
    const scale = theme.color[color]
    const c = scale(intensity).hex()
    return c
}
