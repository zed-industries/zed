import chroma, { Scale } from "chroma-js";

export type Color = string;
export type ColorRampStep = { value: Color; type: "color"; step: number };
export type ColorRamp = {
  [index: number]: ColorRampStep;
};

export function colorRamp(
  color: Color | [Color, Color],
  options?: { steps?: number; increment?: number; }
): ColorRamp {
  let scale: Scale;
  if (Array.isArray(color)) {
    const [startColor, endColor] = color;
    scale = chroma.scale([startColor, endColor]);
  } else {
    let hue = Math.round(chroma(color).hsl()[0]);
    let startColor = chroma.hsl(hue, 0.88, 0.96);
    let endColor = chroma.hsl(hue, 0.68, 0.32);
    scale = chroma
      .scale([startColor, color, endColor])
      .domain([0, 0.5, 1])
      .mode("hsl")
      .gamma(0.2)
      // .correctLightness(true)
      .padding([0, 0.15]);
  }

  const ramp: ColorRamp = {};
  const steps = options?.steps || 10;
  const increment = options?.increment || 100;

  scale.colors(steps, "hex").forEach((color, ix) => {
    const step = ix * increment;
    ramp[step] = {
      value: color,
      step,
      type: "color",
    };
  });

  return ramp;
}
