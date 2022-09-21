import chroma, { Color, Scale } from "chroma-js";
import { ColorScheme, Elevation, Layer, Player, RampSet, Shadow, Style, StyleSet } from "./colorScheme";

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
        .scale((colorRamps[rampName].colors as any)())
        .domain([1, 0]);
    }
    baseRamps.neutral = chroma
      .scale((colorRamps.neutral.colors as any)())
      .domain([1, 0]);
  } else {
    for (var rampName in colorRamps) {
      baseRamps[rampName] = chroma
        .scale((colorRamps[rampName].colors as any)())
        .domain([0, 1]);
    }
    baseRamps.neutral = chroma
      .scale((colorRamps.neutral.colors as any)())
      .domain([0, 1]);
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

function bottomLayer(ramps: RampSet, isLight: boolean): Layer {
  let defaultStyle: Style = {
    background: ramps.neutral(0.25).hex(),
    border: ramps.neutral(0.6).hex(),
    foreground: ramps.neutral(1).hex(),
  };

  let variantStyle: Style = {
    background: ramps.neutral(0.3).hex(),
    border: ramps.neutral(0.6).hex(),
    foreground: ramps.neutral(0.8).hex(),
  };

  let hoveredStyle: Style = {
    background: ramps.neutral(0.4).hex(),
    border: ramps.neutral(1.0).hex(),
    foreground: ramps.neutral(0.9).hex(),
  };

  let pressedStyle: Style = {
    background: ramps.neutral(0.55).hex(),
    border: ramps.neutral(0.9).hex(),
    foreground: ramps.neutral(0.9).hex(),
  };

  let activeStyle: Style = {
    background: ramps.neutral(0.8).hex(),
    border: ramps.neutral(0.8).hex(),
    foreground: ramps.neutral(0.1).hex(),
  };

  let disabledStyle: Style = {
    background: ramps.neutral(0.25).hex(),
    border: ramps.neutral(1).hex(),
    foreground: ramps.neutral(0.9).hex(),
  };

  let styleSet: StyleSet = {
    default: defaultStyle,
    variant: variantStyle,
    hovered: hoveredStyle,
    pressed: pressedStyle,
    active: activeStyle,
    disabled: disabledStyle,
  };

  return {
    base: styleSet,
    on: styleSet,
    info: styleSet,
    positive: styleSet,
    warning: styleSet,
    negative: styleSet
  };
}

function middleLayer(ramps: RampSet, isLight: boolean): Layer {
  let defaultStyle: Style = {
    background: ramps.neutral(0.2).hex(),
    border: ramps.neutral(0.7).hex(),
    foreground: ramps.neutral(1).hex(),
  };

  let variantStyle: Style = {
    background: ramps.neutral(0.3).hex(),
    border: ramps.neutral(0.6).hex(),
    foreground: ramps.neutral(0.8).hex(),
  };

  let hoveredStyle: Style = {
    background: ramps.neutral(0.4).hex(),
    border: ramps.neutral(1.0).hex(),
    foreground: ramps.neutral(0.9).hex(),
  };

  let pressedStyle: Style = {
    background: ramps.neutral(0.55).hex(),
    border: ramps.neutral(0.9).hex(),
    foreground: ramps.neutral(0.9).hex(),
  };

  let activeStyle: Style = {
    background: ramps.neutral(0.8).hex(),
    border: ramps.neutral(0.8).hex(),
    foreground: ramps.neutral(0.1).hex(),
  };

  let disabledStyle: Style = {
    background: ramps.neutral(0.25).hex(),
    border: ramps.neutral(1).hex(),
    foreground: ramps.neutral(0.9).hex(),
  };

  let styleSet: StyleSet = {
    default: defaultStyle,
    variant: variantStyle,
    hovered: hoveredStyle,
    pressed: pressedStyle,
    active: activeStyle,
    disabled: disabledStyle,
  };

  return {
    base: styleSet,
    on: styleSet,
    info: styleSet,
    positive: styleSet,
    warning: styleSet,
    negative: styleSet
  };
}

function topLayer(ramps: RampSet, isLight: boolean): Layer {
  let defaultStyle: Style = {
    background: ramps.neutral(0).hex(),
    border: ramps.neutral(0.7).hex(),
    foreground: ramps.neutral(1).hex(),
  };

  let variantStyle: Style = {
    background: ramps.neutral(0.2).hex(),
    border: ramps.neutral(0.6).hex(),
    foreground: ramps.neutral(0.8).hex(),
  };

  let hoveredStyle: Style = {
    background: ramps.neutral(0.4).hex(),
    border: ramps.neutral(1.0).hex(),
    foreground: ramps.neutral(0.9).hex(),
  };

  let pressedStyle: Style = {
    background: ramps.neutral(0.55).hex(),
    border: ramps.neutral(0.9).hex(),
    foreground: ramps.neutral(0.9).hex(),
  };

  let activeStyle: Style = {
    background: ramps.neutral(0.8).hex(),
    border: ramps.neutral(0.8).hex(),
    foreground: ramps.neutral(0.1).hex(),
  };

  let disabledStyle: Style = {
    background: ramps.neutral(0.25).hex(),
    border: ramps.neutral(1).hex(),
    foreground: ramps.neutral(0.9).hex(),
  };

  let styleSet: StyleSet = {
    default: defaultStyle,
    variant: variantStyle,
    hovered: hoveredStyle,
    pressed: pressedStyle,
    active: activeStyle,
    disabled: disabledStyle,
  };

  return {
    base: styleSet,
    on: styleSet,
    info: styleSet,
    positive: styleSet,
    warning: styleSet,
    negative: styleSet
  };
}