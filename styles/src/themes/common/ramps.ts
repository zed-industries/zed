import chroma, { Color, Scale } from "chroma-js";
import {
  ColorScheme,
  Elevation,
  Layer,
  Player,
  RampSet,
  Shadow,
  Style,
  Styles,
  StyleSet,
} from "./colorScheme";

export function colorRamp(color: Color): Scale {
  let endColor = color.desaturate(1).brighten(5);
  let startColor = color.desaturate(1).darken(5);
  return chroma.scale([startColor, color, endColor]).mode("hsl");
}

export function createColorScheme(
  name: string,
  isLight: boolean,
  colorRamps: { [rampName: string]: Scale }
): ColorScheme {
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
      baseRamps[rampName] = chroma.scale(
        colorRamps[rampName].colors(100).reverse()
      );
    }
    baseRamps.neutral = chroma.scale(colorRamps.neutral.colors(100).reverse());
  } else {
    for (var rampName in colorRamps) {
      baseRamps[rampName] = chroma.scale(colorRamps[rampName].colors(100));
    }
    baseRamps.neutral = chroma.scale(colorRamps.neutral.colors(100));
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

  let lowest = elevation(resampleSet(baseSet, evenSamples(0, 1)));

  let middle = elevation(resampleSet(baseSet, evenSamples(0.08, 1)), {
    blur: 4,
    color: baseSet
      .neutral(isLight ? 7 : 0)
      .darken()
      .alpha(0.2)
      .hex(), // TODO used blend previously. Replace with something else
    offset: [1, 2],
  });
  lowest.above = middle;

  let highest = elevation(resampleSet(baseSet, evenSamples(0.16, 1)), {
    blur: 16,
    color: baseSet
      .neutral(isLight ? 7 : 0)
      .darken()
      .alpha(0.2)
      .hex(), // TODO used blend previously. Replace with something else
    offset: [0, 2],
  });
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
  };

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
  };
}

function evenSamples(min: number, max: number): number[] {
  return Array.from(Array(101).keys()).map(
    (i) => (i / 100) * (max - min) + min
  );
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
  };
}

function resample(scale: Scale, samples: number[]): Scale {
  let newColors = samples.map((sample) => scale(sample));
  return chroma.scale(newColors);
}

function elevation(
  ramps: RampSet,
  shadow?: Shadow
): Elevation {
  return {
    ramps,

    bottom: bottomLayer(ramps),
    middle: middleLayer(ramps),
    top: topLayer(ramps),

    shadow,
  };
}

function bottomLayer(ramps: RampSet): Layer {
  return {
    base: buildStyleSet(ramps.neutral, 0.2, 1),
    variant: buildStyleSet(ramps.neutral, 0.2, 0.7),
    on: buildStyleSet(ramps.neutral, 0.1, 1),
    accent: buildStyleSet(ramps.blue, 0.1, 0.5),
    positive: buildStyleSet(ramps.green, 0.1, 0.5),
    warning: buildStyleSet(ramps.yellow, 0.1, 0.5),
    negative: buildStyleSet(ramps.red, 0.1, 0.5),
  };
}

function middleLayer(ramps: RampSet): Layer {
  return {
    base: buildStyleSet(ramps.neutral, 0.1, 1),
    variant: buildStyleSet(ramps.neutral, 0.1, 0.7),
    on: buildStyleSet(ramps.neutral, 0, 1),
    accent: buildStyleSet(ramps.blue, 0.1, 0.5),
    positive: buildStyleSet(ramps.green, 0.1, 0.5),
    warning: buildStyleSet(ramps.yellow, 0.1, 0.5),
    negative: buildStyleSet(ramps.red, 0.1, 0.5),
  };
}

function topLayer(ramps: RampSet): Layer {
  return {
    base: buildStyleSet(ramps.neutral, 0, 1),
    variant: buildStyleSet(ramps.neutral, 0, 0.7),
    on: buildStyleSet(ramps.neutral, 0.1, 1),
    accent: buildStyleSet(ramps.blue, 0.1, 0.5),
    positive: buildStyleSet(ramps.green, 0.1, 0.5),
    warning: buildStyleSet(ramps.yellow, 0.1, 0.5),
    negative: buildStyleSet(ramps.red, 0.1, 0.5),
  };
}

function buildStyleSet(
  ramp: Scale,
  backgroundBase: number,
  foregroundBase: number,
  step: number = 0.08,
): StyleSet {
  let styleDefinitions = buildStyleDefinition(backgroundBase, foregroundBase, step);

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
    };
  }

  return {
    default: buildStyle("default"),
    hovered: buildStyle("hovered"),
    pressed: buildStyle("pressed"),
    active: buildStyle("active"),
    disabled: buildStyle("disabled"),
    inverted: buildStyle("inverted"),
  };
}

function buildStyleDefinition(bgBase: number, fgBase: number, step: number = 0.08) {
  return {
    background: {
      default: bgBase,
      hovered: bgBase + step,
      pressed: bgBase + step * 1.5,
      active: bgBase + step * 2.2,
      disabled: bgBase,
      inverted: fgBase + step * 6,
    },
    border: {
      default: bgBase + step * 1,
      hovered: bgBase + step,
      pressed: bgBase + step,
      active: bgBase + step * 3,
      disabled: bgBase + step * 0.5,
      inverted: bgBase - step * 3,
    },
    foreground: {
      default: fgBase,
      hovered: fgBase,
      pressed: fgBase,
      active: fgBase + step * 6,
      disabled: bgBase + step * 4,
      inverted: bgBase + step * 2,
    },
  };
}