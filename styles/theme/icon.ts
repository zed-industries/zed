import { useIntensityColor } from "./color"
import { Theme, ThemeColor } from "./config"
import { Intensity } from "./intensity"

type Size = "sm" | "md" | "lg";

type Sizes = Record<Size, number>;

const iconSize: Sizes = {
    sm: 7,
    md: 11,
    lg: 15,
} as const;

interface IconProps {
    theme: Theme,
    size: Size
    intensity?: Intensity
    color?: ThemeColor
}

const DEFAULT_ICON_INTENSITY: Intensity = 100 as const;

/**
* Get an iconStyle from an icon size and intensity.
*
* Optionally, a color can be specified.
*
* If no color is specified, neutral is used.
*/
export const iconStyle = ({ theme, size, intensity = DEFAULT_ICON_INTENSITY, color = "neutral" }: IconProps): IconStyle => {
    const resolvedColor = useIntensityColor(theme, color, intensity);
    const sizeValue = iconSize[size];
    return { color: resolvedColor, size: sizeValue };
};

export interface IconStyle {
    color: string
    size: number
}
