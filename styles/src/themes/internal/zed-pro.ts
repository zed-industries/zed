import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "./common/ramps";

const name = "zed-pro";

const ramps = {
    neutral: chroma.scale([
        "#101010",
        "#1C1C1C",
        "#212121",
        "#2D2D2D",
        "#B9B9B9",
        "#DADADA",
        "#E6E6E6",
        "#FFFFFF",
    ]),
    red: colorRamp(chroma("#DC604F")),
    orange: colorRamp(chroma("#DE782F")),
    yellow: colorRamp(chroma("#E0B750")),
    green: colorRamp(chroma("#2A643D")),
    cyan: colorRamp(chroma("#215050")),
    blue: colorRamp(chroma("#2F6DB7")),
    violet: colorRamp(chroma("#5874C1")),
    magenta: colorRamp(chroma("#DE9AB8")),
};

export const dark = createColorScheme(`${name}-dark`, false, ramps);
export const light = createColorScheme(`${name}-light`, true, ramps);
