import { generateColorFamily } from "../lib/generate";
import { curve } from "./curves";

// These are the source colors for the color scales in the system.
// These should never directly be used directly in components or themes as they generate thousands of lines of code.
// Instead, use the outputs from the reference palette which exports a smaller subset of colors.

// Token or user-facing colors should use short, clear names and a 100-900 scale to match the font weight scale.

// Red ======================================== //

export const red = generateColorFamily({
  name: "red",
  color: {
    hue: {
      start: 0,
      end: 0,
      curve: curve.linear,
    },
    saturation: {
      start: 95,
      end: 75,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 25,
      curve: curve.lightness,
    },
  },
});

// Sunset ======================================== //

export const sunset = generateColorFamily({
  name: "sunset",
  color: {
    hue: {
      start: 12,
      end: 12,
      curve: curve.linear,
    },
    saturation: {
      start: 100,
      end: 80,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 25,
      curve: curve.lightness,
    },
  },
});

// Orange ======================================== //

export const orange = generateColorFamily({
  name: "orange",
  color: {
    hue: {
      start: 25,
      end: 25,
      curve: curve.linear,
    },
    saturation: {
      start: 100,
      end: 100,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 25,
      curve: curve.lightness,
    },
  },
});

// Amber ======================================== //

export const amber = generateColorFamily({
  name: "amber",
  color: {
    hue: {
      start: 34,
      end: 34,
      curve: curve.linear,
    },
    saturation: {
      start: 100,
      end: 100,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 25,
      curve: curve.lightness,
    },
  },
});

// Yellow ======================================== //

export const yellow = generateColorFamily({
  name: "yellow",
  color: {
    hue: {
      start: 48,
      end: 48,
      curve: curve.linear,
    },
    saturation: {
      start: 90,
      end: 100,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 28,
      curve: curve.lightness,
    },
  },
});

// Citron ======================================== //

export const citron = generateColorFamily({
  name: "citron",
  color: {
    hue: {
      start: 65,
      end: 65,
      curve: curve.linear,
    },
    saturation: {
      start: 85,
      end: 70,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 25,
      curve: curve.lightness,
    },
  },
});

// Lime ======================================== //

export const lime = generateColorFamily({
  name: "lime",
  color: {
    hue: {
      start: 85,
      end: 85,
      curve: curve.linear,
    },
    saturation: {
      start: 85,
      end: 70,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 25,
      curve: curve.lightness,
    },
  },
});

// Green ======================================== //

export const green = generateColorFamily({
  name: "green",
  color: {
    hue: {
      start: 108,
      end: 108,
      curve: curve.linear,
    },
    saturation: {
      start: 60,
      end: 50,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 25,
      curve: curve.lightness,
    },
  },
});

// Mint ======================================== //

export const mint = generateColorFamily({
  name: "mint",
  color: {
    hue: {
      start: 142,
      end: 142,
      curve: curve.linear,
    },
    saturation: {
      start: 60,
      end: 50,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 20,
      curve: curve.lightness,
    },
  },
});

// Cyan ======================================== //

export const cyan = generateColorFamily({
  name: "cyan",
  color: {
    hue: {
      start: 179,
      end: 179,
      curve: curve.linear,
    },
    saturation: {
      start: 70,
      end: 60,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 20,
      curve: curve.lightness,
    },
  },
});

// Sky ======================================== //

export const sky = generateColorFamily({
  name: "sky",
  color: {
    hue: {
      start: 195,
      end: 195,
      curve: curve.linear,
    },
    saturation: {
      start: 85,
      end: 75,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 20,
      curve: curve.lightness,
    },
  },
});

// Blue ======================================== //

export const blue = generateColorFamily({
  name: "blue",
  color: {
    hue: {
      start: 210,
      end: 210,
      curve: curve.linear,
    },
    saturation: {
      start: 90,
      end: 75,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 20,
      curve: curve.lightness,
    },
  },
});

// Indigo ======================================== //

export const indigo = generateColorFamily({
  name: "indigo",
  color: {
    hue: {
      start: 230,
      end: 230,
      curve: curve.linear,
    },
    saturation: {
      start: 80,
      end: 50,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 20,
      curve: curve.lightness,
    },
  },
});

// Purple ======================================== //

export const purple = generateColorFamily({
  name: "purple",
  color: {
    hue: {
      start: 260,
      end: 265,
      curve: curve.linear,
    },
    saturation: {
      start: 80,
      end: 50,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 20,
      curve: curve.lightness,
    },
  },
});

// Pink ======================================== //

export const pink = generateColorFamily({
  name: "pink",
  color: {
    hue: {
      start: 310,
      end: 310,
      curve: curve.linear,
    },
    saturation: {
      start: 80,
      end: 75,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 20,
      curve: curve.lightness,
    },
  },
});

// Rose ======================================== //

export const rose = generateColorFamily({
  name: "rose",
  color: {
    hue: {
      start: 345,
      end: 345,
      curve: curve.linear,
    },
    saturation: {
      start: 90,
      end: 65,
      curve: curve.saturation,
    },
    lightness: {
      start: 97,
      end: 20,
      curve: curve.lightness,
    },
  },
});
