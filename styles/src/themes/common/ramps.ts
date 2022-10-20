import chroma, { Color, Scale } from "chroma-js";
import {
  ColorScheme,
  Layer,
  Player,
  RampSet,
  Style,
  Styles,
  StyleSet,
} from "./colorScheme";

export function colorRamp(color: Color): Scale {
  let endColor = color.desaturate(1).brighten(5);
  let startColor = color.desaturate(1).darken(4);
  return chroma.scale([startColor, color, endColor]).mode("lab");
}

export function createColorScheme(
  name: string,
  isLight: boolean,
  colorRamps: { [rampName: string]: Scale }
): ColorScheme {
  // Chromajs scales from 0 to 1 flipped if isLight is true
  let ramps: RampSet = {} as any;

  // Chromajs mutates the underlying ramp when you call domain. This causes problems because
  // we now store the ramps object in the theme so that we can pull colors out of them.
  // So instead of calling domain and storing the result, we have to construct new ramps for each
  // theme so that we don't modify the passed in ramps.
  // This combined with an error in the type definitions for chroma js means we have to cast the colors
  // function to any in order to get the colors back out from the original ramps.
  if (isLight) {
    for (var rampName in colorRamps) {
      (ramps as any)[rampName] = chroma.scale(
        colorRamps[rampName].colors(100).reverse()
      );
    }
    ramps.neutral = chroma.scale(colorRamps.neutral.colors(100).reverse());
  } else {
    for (var rampName in colorRamps) {
      (ramps as any)[rampName] = chroma.scale(colorRamps[rampName].colors(100));
    }
    ramps.neutral = chroma.scale(colorRamps.neutral.colors(100));
  }

  let lowest = lowestLayer(ramps);
  let middle = middleLayer(ramps);
  let highest = highestLayer(ramps);

  let popoverShadow = {
    blur: 4,
    color: ramps
      .neutral(isLight ? 7 : 0)
      .darken()
      .alpha(0.2)
      .hex(), // TODO used blend previously. Replace with something else
    offset: [1, 2],
  };

  let modalShadow = {
    blur: 16,
    color: ramps
      .neutral(isLight ? 7 : 0)
      .darken()
      .alpha(0.2)
      .hex(), // TODO used blend previously. Replace with something else
    offset: [0, 2],
  };

  let players = {
    "0": player(ramps.blue),
    "1": player(ramps.green),
    "2": player(ramps.magenta),
    "3": player(ramps.orange),
    "4": player(ramps.violet),
    "5": player(ramps.cyan),
    "6": player(ramps.red),
    "7": player(ramps.yellow),
  };

  return {
    name,
    isLight,

    ramps,

    lowest,
    middle,
    highest,

    popoverShadow,
    modalShadow,

    players,
  };
}

function player(ramp: Scale): Player {
  return {
    selection: ramp(0.5).alpha(0.24).hex(),
    cursor: ramp(0.5).hex(),
  };
}

function lowestLayer(ramps: RampSet): Layer {
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

function highestLayer(ramps: RampSet): Layer {
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