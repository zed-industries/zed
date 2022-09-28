import chroma, { Color, Scale } from "chroma-js";
import { ColorScheme, Elevation, Layer, Player, RampSet, Shadow, Style, Styles, StyleSet } from "./colorScheme";

export function colorRamp(color: Color): Scale {
  let hue = color.hsl()[0];
  let endColor = chroma.hsl(hue, 0.88, 0.96);
  let startColor = chroma.hsl(hue, 0.68, 0.12);
  return chroma.scale([startColor, color, endColor]).mode("hsl");
}

export function createColorScheme(name: string, isLight: boolean, colorRamps: { [rampName: string]: Scale }): ColorScheme {
  // Chromajs scales from 0 to 1 flipped if isLight is true
  let baseRamps: typeof colorRamps = {};

  // Chromajs mutates the underlying ramp when you call domain. This causes problems because
  // we now store the ramps object in the theme so that we can pull colors out of them.
  // So instead of calling domain and storing the result, we have to construct new ramps for each
  // theme so that we don't modify the passed in ramps.
  // This combined with an error in the type definitions for chroma js means we have to cast the colors
  // function to any in order to get the colors back out from the original ramps.
  if (isLight) {
    for (var rampName in colorRamps) {
      baseRamps[rampName] = chroma
        .scale(colorRamps[rampName].colors(100).reverse());
    }
    baseRamps.neutral = chroma
      .scale(colorRamps.neutral.colors(100).reverse());
  } else {
    for (var rampName in colorRamps) {
      baseRamps[rampName] = chroma
        .scale(colorRamps[rampName].colors(100));
    }
    baseRamps.neutral = chroma
      .scale(colorRamps.neutral.colors(100));
  }

  let baseSet = {
    neutral: baseRamps.neutral,
    red: baseRamps.red,
    orange: baseRamps.orange,
    yellow: baseRamps.yellow,
    green: baseRamps.green,
    cyan: baseRamps.cyan,
    blue: baseRamps.blue,
    violet: baseRamps.violet,
    magenta: baseRamps.magenta,
  };

  let lowest = elevation(
    resampleSet(
      baseSet,
      evenSamples(0, 1)
    ),
    isLight,
  );

  let middle = elevation(
    resampleSet(
      baseSet,
      evenSamples(0.125, 1)
    ),
    isLight,
    {
      blur: 4,
      color: baseSet.neutral(isLight ? 7 : 0).darken().alpha(0.2).hex(), // TODO used blend previously. Replace with something else
      offset: [1, 2],
    }
  );
  lowest.above = middle;

  let highest = elevation(
    resampleSet(
      baseSet,
      evenSamples(0.25, 1)
    ),
    isLight,
    {
      blur: 16,
      color: baseSet.neutral(isLight ? 7 : 0).darken().alpha(0.2).hex(), // TODO used blend previously. Replace with something else
      offset: [0, 2],
    }
  );
  middle.above = highest;

  let players = {
    "0": player(baseSet.blue),
    "1": player(baseSet.green),
    "2": player(baseSet.magenta),
    "3": player(baseSet.orange),
    "4": player(baseSet.violet),
    "5": player(baseSet.cyan),
    "6": player(baseSet.red),
    "7": player(baseSet.yellow),
  }

  return {
    name,
    isLight,

    lowest,
    middle,
    highest,

    players,
  };
}

function player(ramp: Scale): Player {
  return {
    selection: ramp(0.5).alpha(0.24).hex(),
    cursor: ramp(0.5).hex(),
  }
}

function evenSamples(min: number, max: number): number[] {
  return Array.from(Array(101).keys()).map((i) => (i / 100) * (max - min) + min);
}

function resampleSet(ramps: RampSet, samples: number[]): RampSet {
  return {
    neutral: resample(ramps.neutral, samples),
    red: resample(ramps.red, samples),
    orange: resample(ramps.orange, samples),
    yellow: resample(ramps.yellow, samples),
    green: resample(ramps.green, samples),
    cyan: resample(ramps.cyan, samples),
    blue: resample(ramps.blue, samples),
    violet: resample(ramps.violet, samples),
    magenta: resample(ramps.magenta, samples),
  }
}

function resample(scale: Scale, samples: number[]): Scale {
  let newColors = samples.map((sample) => scale(sample));
  return chroma.scale(newColors);
}

function elevation(ramps: RampSet, isLight: boolean, shadow?: Shadow): Elevation {
  return {
    ramps,

    bottom: bottomLayer(ramps, isLight),
    middle: middleLayer(ramps, isLight),
    top: topLayer(ramps, isLight),

    shadow,
  };
}

interface StyleColors {
  default: number | Color,
  hovered: number | Color,
  pressed: number | Color,
  active: number | Color,
  disabled: number | Color,
}
function buildStyleSet(ramp: Scale, styleDefinitions: {
  background: StyleColors,
  border: StyleColors,
  foreground: StyleColors,
}): StyleSet {
  function colorString(indexOrColor: number | Color): string {
    if (typeof indexOrColor === "number") {
      return ramp(indexOrColor).hex();
    } else {
      return indexOrColor.hex();
    }
  }

  function buildStyle(style: Styles): Style {
    return {
      background: colorString(styleDefinitions.background[style]),
      border: colorString(styleDefinitions.border[style]),
      foreground: colorString(styleDefinitions.foreground[style]),
    }
  }

  return {
    default: buildStyle("default"),
    hovered: buildStyle("hovered"),
    pressed: buildStyle("pressed"),
    active: buildStyle("active"),
    disabled: buildStyle("disabled"),
  }
}

function bottomLayer(ramps: RampSet, isLight: boolean): Layer {
  let baseSet = buildStyleSet(ramps.neutral, {
    background: {
      default: 0.16,
      hovered: 0.31,
      pressed: 0.41,
      active: 1,
      disabled: 0.16,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.5,
      disabled: 0.2,
    },
    foreground: {
      default: 1,
      hovered: 1,
      pressed: 1,
      active: 0,
      disabled: 0.4,
    },
  });

  let variantSet = buildStyleSet(ramps.neutral, {
    background: {
      default: 0.16,
      hovered: 0.31,
      pressed: 0.41,
      active: 1,
      disabled: 0.16,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.5,
      disabled: 0.2,
    },
    foreground: {
      default: 0.7,
      hovered: 1,
      pressed: 1,
      active: 0,
      disabled: 0.4,
    },
  });

  let onSet = buildStyleSet(ramps.neutral, {
    background: {
      default: 0.08,
      hovered: 0.23,
      pressed: 0.33,
      active: 1,
      disabled: 0.08,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.5,
      disabled: 0.2,
    },
    foreground: {
      default: 1,
      hovered: 1,
      pressed: 1,
      active: 0,
      disabled: 0.4,
    },
  });

  let infoSet = buildStyleSet(ramps.blue, {
    background: {
      default: 0,
      hovered: 0.1,
      pressed: 0.2,
      active: 0.4,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.6,
      disabled: 0.1,
    },
    foreground: {
      default: 0.9,
      hovered: 0.9,
      pressed: 0.9,
      active: 0.9,
      disabled: 0.2,
    },
  });

  let positiveSet = buildStyleSet(ramps.green, {
    background: {
      default: 0,
      hovered: 0.1,
      pressed: 0.2,
      active: 0.4,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.6,
      disabled: 0.1,
    },
    foreground: {
      default: 0.9,
      hovered: 0.9,
      pressed: 0.9,
      active: 0.9,
      disabled: 0.2,
    },
  });

  let warningSet = buildStyleSet(ramps.yellow, {
    background: {
      default: 0.1,
      hovered: 0.1,
      pressed: 0.2,
      active: 0.4,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.6,
      disabled: 0.1,
    },
    foreground: {
      default: 0.6,
      hovered: 0.9,
      pressed: 0.9,
      active: 0.9,
      disabled: 0.2,
    },
  });

  let negativeSet = buildStyleSet(ramps.red, {
    background: {
      default: 0,
      hovered: 0.1,
      pressed: 0.2,
      active: 0.4,
      disabled: 0,
    },
    border: {
      default: 0.1,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.6,
      disabled: 0.1,
    },
    foreground: {
      default: 0.9,
      hovered: 0.9,
      pressed: 0.9,
      active: 0.9,
      disabled: 0.2,
    },
  });

  return {
    base: baseSet,
    variant: variantSet,
    on: onSet,
    info: infoSet,
    positive: positiveSet,
    warning: warningSet,
    negative: negativeSet
  };
}

function middleLayer(ramps: RampSet, isLight: boolean): Layer {
  let baseSet = buildStyleSet(ramps.neutral, {
    background: {
      default: 0.08,
      hovered: 0.23,
      pressed: 0.33,
      active: 1,
      disabled: 0.08,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.5,
      disabled: 0.2,
    },
    foreground: {
      default: 1,
      hovered: 1,
      pressed: 1,
      active: 0,
      disabled: 0.4,
    },
  });

  let variantSet = buildStyleSet(ramps.neutral, {
    background: {
      default: 0.08,
      hovered: 0.23,
      pressed: 0.33,
      active: 1,
      disabled: 0.08,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.5,
      disabled: 0.2,
    },
    foreground: {
      default: 0.7,
      hovered: 0.7,
      pressed: 0.7,
      active: 0,
      disabled: 0.4,
    },
  });

  let onSet = buildStyleSet(ramps.neutral, {
    background: {
      default: 0,
      hovered: 0.15,
      pressed: 0.25,
      active: 1,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.5,
      disabled: 0.2,
    },
    foreground: {
      default: 1,
      hovered: 1,
      pressed: 1,
      active: 0,
      disabled: 0.4,
    },
  });

  let infoSet = buildStyleSet(ramps.blue, {
    background: {
      default: 0,
      hovered: 0.1,
      pressed: 0.2,
      active: 0.4,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.6,
      disabled: 0.1,
    },
    foreground: {
      default: 0.9,
      hovered: 0.9,
      pressed: 0.9,
      active: 0.9,
      disabled: 0.2,
    },
  });

  let positiveSet = buildStyleSet(ramps.green, {
    background: {
      default: 0,
      hovered: 0.1,
      pressed: 0.2,
      active: 0.4,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.6,
      disabled: 0.1,
    },
    foreground: {
      default: 0.9,
      hovered: 0.9,
      pressed: 0.9,
      active: 0.9,
      disabled: 0.2,
    },
  });

  let warningSet = buildStyleSet(ramps.yellow, {
    background: {
      default: 0,
      hovered: 0.1,
      pressed: 0.2,
      active: 0.4,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.6,
      disabled: 0.1,
    },
    foreground: {
      default: 0.9,
      hovered: 0.9,
      pressed: 0.9,
      active: 0.9,
      disabled: 0.2,
    },
  });

  let negativeSet = buildStyleSet(ramps.red, {
    background: {
      default: 0,
      hovered: 0.1,
      pressed: 0.2,
      active: 0.4,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.6,
      disabled: 0.1,
    },
    foreground: {
      default: 0.9,
      hovered: 0.9,
      pressed: 0.9,
      active: 0.9,
      disabled: 0.2,
    },
  });

  return {
    base: baseSet,
    variant: variantSet,
    on: onSet,
    info: infoSet,
    positive: positiveSet,
    warning: warningSet,
    negative: negativeSet
  };
}

function topLayer(ramps: RampSet, isLight: boolean): Layer {

  let baseSet = buildStyleSet(ramps.neutral, {
    background: {
      default: 0,
      hovered: 0.15,
      pressed: 0.25,
      active: 1,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.5,
      disabled: 0.2,
    },
    foreground: {
      default: 1,
      hovered: 1,
      pressed: 1,
      active: 0,
      disabled: 0.4,
    },
  });

  let variantSet = buildStyleSet(ramps.neutral, {
    background: {
      default: 0,
      hovered: 0.15,
      pressed: 0.25,
      active: 1,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.5,
      disabled: 0.2,
    },
    foreground: {
      default: 0.7,
      hovered: 0.7,
      pressed: 0.7,
      active: 0,
      disabled: 0.4,
    },
  });

  let onSet = buildStyleSet(ramps.neutral, {
    background: {
      default: 0.15,
      hovered: 0.3,
      pressed: 0.4,
      active: 1,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.5,
      disabled: 0.2,
    },
    foreground: {
      default: 1,
      hovered: 1,
      pressed: 1,
      active: 0,
      disabled: 0.4,
    },
  });

  let infoSet = buildStyleSet(ramps.blue, {
    background: {
      default: 0,
      hovered: 0.1,
      pressed: 0.2,
      active: 0.4,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.6,
      disabled: 0.1,
    },
    foreground: {
      default: 0.9,
      hovered: 0.9,
      pressed: 0.9,
      active: 0.9,
      disabled: 0.2,
    },
  });

  let positiveSet = buildStyleSet(ramps.green, {
    background: {
      default: 0,
      hovered: 0.1,
      pressed: 0.2,
      active: 0.4,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.6,
      disabled: 0.1,
    },
    foreground: {
      default: 0.9,
      hovered: 0.9,
      pressed: 0.9,
      active: 0.9,
      disabled: 0.2,
    },
  });

  let warningSet = buildStyleSet(ramps.yellow, {
    background: {
      default: 0,
      hovered: 0.1,
      pressed: 0.2,
      active: 0.4,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.6,
      disabled: 0.1,
    },
    foreground: {
      default: 0.9,
      hovered: 0.9,
      pressed: 0.9,
      active: 0.9,
      disabled: 0.2,
    },
  });

  let negativeSet = buildStyleSet(ramps.red, {
    background: {
      default: 0,
      hovered: 0.1,
      pressed: 0.2,
      active: 0.4,
      disabled: 0,
    },
    border: {
      default: 0.2,
      hovered: 0.2,
      pressed: 0.2,
      active: 0.6,
      disabled: 0.1,
    },
    foreground: {
      default: 0.9,
      hovered: 0.9,
      pressed: 0.9,
      active: 0.9,
      disabled: 0.2,
    },
  });

  return {
    base: baseSet,
    variant: variantSet,
    on: onSet,
    info: infoSet,
    positive: positiveSet,
    warning: warningSet,
    negative: negativeSet
  };
}