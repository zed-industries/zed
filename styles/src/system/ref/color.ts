import chroma from "chroma-js";
import {
  generateColors2,
  generateColorSet,
  generateColorsUsingCurve,
} from "../algorithm";
import { curve } from "../curves";
import { ColorFamily } from "../types";

// These are the source colors for the color scales in the system.
// This should never directly be used in the system, or exported to be used in a component or theme
// As it will generate thousands of lines of code.
// Instead, use the outputs from the reference palette which exports a smaller subset of colors.

// Token or user-facing colors should use short, clear names
// and a 100-900 scale to match the font weight scale.

// Red ======================================== //

export const red = generateColors2(
  {
    start: 0,
    end: 0,
    curve: curve.linear,
  },
  {
    start: 95,
    end: 75,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 25,
    curve: curve.lightness,
  }
);

// Sunset ======================================== //

export const sunset = generateColors2(
  {
    start: 12,
    end: 12,
    curve: curve.linear,
  },
  {
    start: 100,
    end: 80,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 25,
    curve: curve.lightness,
  }
);

// Orange ======================================== //

export const orange = generateColors2(
  {
    start: 25,
    end: 25,
    curve: curve.linear,
  },
  {
    start: 100,
    end: 100,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 25,
    curve: curve.lightness,
  }
);

// Amber ======================================== //

export const amber = generateColors2(
  {
    start: 34,
    end: 34,
    curve: curve.linear,
  },
  {
    start: 100,
    end: 100,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 25,
    curve: curve.lightness,
  }
);

// Yellow ======================================== //

export const yellow = generateColors2(
  {
    start: 48,
    end: 48,
    curve: curve.linear,
  },
  {
    start: 90,
    end: 100,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 32,
    curve: curve.lightness,
  }
);

// Citron ======================================== //

export const citron = generateColors2(
  {
    start: 65,
    end: 65,
    curve: curve.linear,
  },
  {
    start: 85,
    end: 70,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 25,
    curve: curve.lightness,
  }
);

// Lime ======================================== //

export const lime = generateColors2(
  {
    start: 85,
    end: 85,
    curve: curve.linear,
  },
  {
    start: 85,
    end: 70,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 25,
    curve: curve.lightness,
  }
);

// Green ======================================== //

export const green = generateColors2(
  {
    start: 108,
    end: 108,
    curve: curve.linear,
  },
  {
    start: 60,
    end: 50,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 25,
    curve: curve.lightness,
  }
);

// Mint ======================================== //

export const mint = generateColors2(
  {
    start: 142,
    end: 142,
    curve: curve.linear,
  },
  {
    start: 60,
    end: 50,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 20,
    curve: curve.lightness,
  }
);

// Cyan ======================================== //

export const cyan = generateColors2(
  {
    start: 179,
    end: 179,
    curve: curve.linear,
  },
  {
    start: 70,
    end: 60,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 20,
    curve: curve.lightness,
  }
);

// Sky ======================================== //

export const sky = generateColors2(
  {
    start: 190,
    end: 190,
    curve: curve.linear,
  },
  {
    start: 85,
    end: 75,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 20,
    curve: curve.lightness,
  }
);

// Blue ======================================== //

export const blue = generateColors2(
  {
    start: 210,
    end: 210,
    curve: curve.linear,
  },
  {
    start: 90,
    end: 60,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 20,
    curve: curve.lightness,
  }
);

// Indigo ======================================== //

export const indigo = generateColors2(
  {
    start: 240,
    end: 240,
    curve: curve.linear,
  },
  {
    start: 80,
    end: 40,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 20,
    curve: curve.lightness,
  }
);

// Purple ======================================== //

export const purple = generateColors2(
  {
    start: 260,
    end: 265,
    curve: curve.linear,
  },
  {
    start: 80,
    end: 50,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 20,
    curve: curve.lightness,
  }
);

// Pink ======================================== //

export const pink = generateColors2(
  {
    start: 310,
    end: 310,
    curve: curve.linear,
  },
  {
    start: 80,
    end: 70,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 20,
    curve: curve.lightness,
  }
);

// Rose ======================================== //

export const rose = generateColors2(
  {
    start: 345,
    end: 345,
    curve: curve.linear,
  },
  {
    start: 90,
    end: 65,
    curve: curve.saturation,
  },
  {
    start: 97,
    end: 20,
    curve: curve.lightness,
  }
);
