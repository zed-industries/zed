import chroma from "chroma-js";

export type Color = string;

function getColorRamp(colorName, baseColor, steps = 10) {
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
    .colors(steps);

  let tokens = {};
  let token = {};
  let colorNumber = 0;

  for (let i = 0; i < steps; i++) {
    if (i !== 0) {
      colorNumber = i * 100;
    }

    token = {
      [`${colorName}_${colorNumber}`]: {
        value: ramp[i].value,
        rootValue: baseColor,
        step: i,
        type: "color",
      },
    };

    Object.assign(token, tokens);
  }

  return tokens;
}

export default {
  color: {
    rose: getColorRamp("rose", "#F43F5E", 10),
  },

  fontFamily: {
    sans: "Zed Sans",
    mono: "Zed Mono",
  },
  fontSize: {
    "3xs": {
      value: "8",
      type: "fontSizes",
    },
    "2xs": {
      value: "10",
      type: "fontSizes",
    },
    xs: {
      value: "12",
      type: "fontSizes",
    },
    sm: {
      value: "14",
      type: "fontSizes",
    },
    md: {
      value: "16",
      type: "fontSizes",
    },
    lg: {
      value: "18",
      type: "fontSizes",
    },
    xl: {
      value: "20",
      type: "fontSizes",
    },
  },
};
