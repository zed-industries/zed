import { Intensity } from "./color";
import { color as buildColor } from "./color";
import { Theme } from "./config";

function getColor(theme: Theme, colorKey: keyof Theme["colors"], intensity: Intensity) {
    return buildColor(theme, colorKey, intensity);
}

interface ColorFunctions {
    [colorKey: string]: (intensity: Intensity) => string;
}

export function useColors(theme: Theme): ColorFunctions {
    const functions: ColorFunctions = {};
    for (const colorKey in theme.colors) {
        const key = colorKey as keyof Theme["colors"];
        functions[key] = (intensity: Intensity) => getColor(theme, key, intensity);
    }
    return functions;
}
