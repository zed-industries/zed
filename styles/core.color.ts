import chroma from "chroma-js";

export type Color = string;

function returnTokens(
  colorName: string,
  ramp: Array<object>, // help, have no clue on type here
) {
  let tokens = {};
  let token = {};
  let colorNumber = 0;
  let increment = 0;

  for (let i = 0; i < ramp.len; i++) {
    if (i > 11 ) {
      increment = 50;
    } else {
      increment = 100;
    }

    if (i !== 0) {
      colorNumber = i * increment;
    }

    token = {
      [`${colorName}_${colorNumber}`]: {
        value: ramp[i].value,
        step: i,
        type: "color",
      },
    };

    Object.assign(token, tokens);
  }
  return tokens;
}

function oneColorRamp(
  colorName: string, 
  baseColor: string, 
  steps: number = 10
) {
  let hsl = chroma(baseColor).hsl();
  let h = Math.round(hsl[0]);
  let lightColor = chroma.hsl(h, 0.88, 0.96).hex();
  let darkColor = chroma.hsl(h, 0.68, 0.32).hex();

  let ramp = chroma
    .scale([lightColor, baseColor, darkColor])
    .domain([0, 0.5, 1])
    .mode("hsl")
    .gamma(1)
    .correctLightness(true)
    .padding([0, 0.15])
    .colors(steps)
    .hex();

  return returnTokens(colorName, ramp);
}

function colorRamp(
  colorName: string, 
  startColor: string, 
  endColor: string, 
  steps: number
) {
  let ramp = chroma.scale([startColor, endColor]).mode("hsl").colors(steps).hex();

  return returnTokens(colorName, ramp);
}

export default {
  color: {
    neutral: colorRamp("neutral", "black", "white", 21), // colorName, startColor, endColor, steps
    rose: oneColorRamp("rose", "#F43F5EFF"), // colorName, baseColor, steps(optional)
    red: oneColorRamp("red", "#EF4444FF"),
    orange: oneColorRamp("orange", "#F97316FF"),
    amber: oneColorRamp("amber", "#F59E0BFF"),
    yellow: oneColorRamp("yellow", "#EAB308FF"),
    lime: oneColorRamp("lime", "#84CC16FF"),
    green: oneColorRamp("green", "#22C55EFF"),
    emerald: oneColorRamp("emerald", "#10B981FF"),
    teal: oneColorRamp("teal", "#14B8A6FF"),
    cyan: oneColorRamp("cyan", "#06BBD4FF"),
    sky: oneColorRamp("sky", "#0EA5E9FF"),
    blue: oneColorRamp("blue", "#3B82F6FF"),
    indigo: oneColorRamp("indigo", "#6366F1FF"),
    violet: oneColorRamp("violet", "#8B5CF6FF"),
    purple: oneColorRamp("purple", "#A855F7FF"),
    fuschia: oneColorRamp("fuschia", "#D946E4FF"),
    pink: oneColorRamp("pink", "#EC4899FF"),
  },
};
