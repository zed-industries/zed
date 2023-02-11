// Adapted from @k-vyn/coloralgorithm

export interface Curve {
  name: string;
  formatted_name: string;
  value: number[];
}

export interface Curves {
  linear: Curve;
  easeInCubic: Curve;
  easeOutCubic: Curve;
  easeInOutCubic: Curve;
  easeInSine: Curve;
  easeOutSine: Curve;
  easeInOutSine: Curve;
  easeInQuad: Curve;
  easeOutQuad: Curve;
  easeInOutQuad: Curve;
  easeInQuart: Curve;
  easeOutQuart: Curve;
  easeInOutQuart: Curve;
  easeInQuint: Curve;
  easeOutQuint: Curve;
  easeInOutQuint: Curve;
  easeInExpo: Curve;
  easeOutExpo: Curve;
  easeInOutExpo: Curve;
  easeInCirc: Curve;
  easeOutCirc: Curve;
  easeInOutCirc: Curve;
  easeInBack: Curve;
  easeOutBack: Curve;
  easeInOutBack: Curve;
}

export const curve: Curves = {
  linear: {
    name: "linear",
    formatted_name: "Linear",
    value: [0.5, 0.5, 0.5, 0.5],
  },
  easeInCubic: {
    name: "easeInCubic",
    formatted_name: "Cubic - EaseIn",
    value: [0.55, 0.055, 0.675, 0.19],
  },
  easeOutCubic: {
    name: "easeOutCubic",
    formatted_name: "Cubic - EaseOut",
    value: [0.215, 0.61, 0.355, 1],
  },
  easeInOutCubic: {
    name: "easeInOutCubic",
    formatted_name: "Cubic - EaseInOut",
    value: [0.645, 0.045, 0.355, 1],
  },
  easeInSine: {
    name: "easeInSine",
    formatted_name: "Sine - EaseIn",
    value: [0.47, 0, 0.745, 0.715],
  },
  easeOutSine: {
    name: "easeOutSine",
    formatted_name: "Sine - EaseOut",
    value: [0.39, 0.575, 0.565, 1],
  },
  easeInOutSine: {
    name: "easeInOutSine",
    formatted_name: "Sine - EaseInOut",
    value: [0.445, 0.05, 0.55, 0.95],
  },
  easeInQuad: {
    name: "easeInQuad",
    formatted_name: "Quad - EaseIn",
    value: [0.55, 0.085, 0.68, 0.53],
  },
  easeOutQuad: {
    name: "easeOutQuad",
    formatted_name: "Quad - EaseOut",
    value: [0.25, 0.46, 0.45, 0.94],
  },
  easeInOutQuad: {
    name: "easeInOutQuad",
    formatted_name: "Quad - EaseInOut",
    value: [0.455, 0.03, 0.515, 0.955],
  },
  easeInQuart: {
    name: "easeInQuart",
    formatted_name: "Quart - EaseIn",
    value: [0.895, 0.03, 0.685, 0.22],
  },
  easeOutQuart: {
    name: "easeOutQuart",
    formatted_name: "Quart - EaseOut",
    value: [0.165, 0.84, 0.44, 1],
  },
  easeInOutQuart: {
    name: "easeInOutQuart",
    formatted_name: "Quart - EaseInOut",
    value: [0.77, 0, 0.175, 1],
  },
  easeInQuint: {
    name: "easeInQuint",
    formatted_name: "Quint - EaseIn",
    value: [0.755, 0.05, 0.855, 0.06],
  },
  easeOutQuint: {
    name: "easeOutQuint",
    formatted_name: "Quint - EaseOut",
    value: [0.23, 1, 0.32, 1],
  },
  easeInOutQuint: {
    name: "easeInOutQuint",
    formatted_name: "Quint - EaseInOut",
    value: [0.86, 0, 0.07, 1],
  },
  easeInCirc: {
    name: "easeInCirc",
    formatted_name: "Circ - EaseIn",
    value: [0.6, 0.04, 0.98, 0.335],
  },
  easeOutCirc: {
    name: "easeOutCirc",
    formatted_name: "Circ - EaseOut",
    value: [0.075, 0.82, 0.165, 1],
  },
  easeInOutCirc: {
    name: "easeInOutCirc",
    formatted_name: "Circ - EaseInOut",
    value: [0.785, 0.135, 0.15, 0.86],
  },
  easeInExpo: {
    name: "easeInExpo",
    formatted_name: "Expo - EaseIn",
    value: [0.95, 0.05, 0.795, 0.035],
  },
  easeOutExpo: {
    name: "easeOutExpo",
    formatted_name: "Expo - EaseOut",
    value: [0.19, 1, 0.22, 1],
  },
  easeInOutExpo: {
    name: "easeInOutExpo",
    formatted_name: "Expo - EaseInOut",
    value: [1, 0, 0, 1],
  },
  easeInBack: {
    name: "easeInBack",
    formatted_name: "Back - EaseIn",
    value: [0.6, -0.28, 0.735, 0.045],
  },
  easeOutBack: {
    name: "easeOutBack",
    formatted_name: "Back - EaseOut",
    value: [0.175, 0.885, 0.32, 1.275],
  },
  easeInOutBack: {
    name: "easeInOutBack",
    formatted_name: "Back - EaseInOut",
    value: [0.68, -0.55, 0.265, 1.55],
  },
};
