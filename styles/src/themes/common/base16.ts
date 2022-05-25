import chroma, { Color, Scale } from "chroma-js";
import { color, ColorToken, fontWeights, NumberToken } from "../../tokens";
import { withOpacity } from "../../utils/color";
import Theme, { buildPlayer, Syntax } from "./theme";

export function colorRamp(color: Color): Scale {
  let hue = color.hsl()[0];
  let endColor = chroma.hsl(hue, 0.88, 0.96);
  let startColor = chroma.hsl(hue, 0.68, 0.12);
  return chroma.scale([startColor, color, endColor]).mode("hsl");
}

export function createTheme(
  name: string,
  isLight: boolean,
  ramps: { [rampName: string]: Scale },
  blend?: number
): Theme {
  if (isLight) {
    for (var rampName in ramps) {
      ramps[rampName] = ramps[rampName].domain([1, 0]);
    }
    ramps.neutral = ramps.neutral.domain([7, 0]);
  } else {
    ramps.neutral = ramps.neutral.domain([0, 7]);
  }

  if (blend === undefined) {
    blend = isLight ? 0.12 : 0.24;
  }

  function rampColor(ramp: Scale, index: number): ColorToken {
    return color(ramp(index).hex());
  }

  const backgroundColor = {
    // Title bar
    100: {
      base: rampColor(ramps.neutral, 1.25),
      hovered: rampColor(ramps.neutral, 1.5),
      active: rampColor(ramps.neutral, 1.75),
    },
    // Midground (panels, etc)
    300: {
      base: rampColor(ramps.neutral, 1),
      hovered: rampColor(ramps.neutral, 1.25),
      active: rampColor(ramps.neutral, 1.5),
    },
    // Editor
    500: {
      base: rampColor(ramps.neutral, 0),
      hovered: rampColor(ramps.neutral, 0.25),
      active: rampColor(ramps.neutral, 0.5),
    },
    on300: {
      base: rampColor(ramps.neutral, 0),
      hovered: rampColor(ramps.neutral, 0.25),
      active: rampColor(ramps.neutral, 0.5),
    },
    on500: {
      base: rampColor(ramps.neutral, 1.25),
      hovered: rampColor(ramps.neutral, 1.5),
      active: rampColor(ramps.neutral, 1.75),
    },
    ok: {
      base: withOpacity(rampColor(ramps.green, 0.5), 0.15),
      hovered: withOpacity(rampColor(ramps.green, 0.5), 0.2),
      active: withOpacity(rampColor(ramps.green, 0.5), 0.25),
    },
    error: {
      base: withOpacity(rampColor(ramps.red, 0.5), 0.15),
      hovered: withOpacity(rampColor(ramps.red, 0.5), 0.2),
      active: withOpacity(rampColor(ramps.red, 0.5), 0.25),
    },
    warning: {
      base: withOpacity(rampColor(ramps.yellow, 0.5), 0.15),
      hovered: withOpacity(rampColor(ramps.yellow, 0.5), 0.2),
      active: withOpacity(rampColor(ramps.yellow, 0.5), 0.25),
    },
    info: {
      base: withOpacity(rampColor(ramps.blue, 0.5), 0.15),
      hovered: withOpacity(rampColor(ramps.blue, 0.5), 0.2),
      active: withOpacity(rampColor(ramps.blue, 0.5), 0.25),
    },
  };

  const borderColor = {
    primary: rampColor(ramps.neutral, isLight ? 1.5 : 0),
    secondary: rampColor(ramps.neutral, isLight ? 1.25 : 1),
    muted: rampColor(ramps.neutral, isLight ? 1 : 3),
    active: rampColor(ramps.neutral, isLight ? 4 : 3),
    onMedia: withOpacity(rampColor(ramps.neutral, 0), 0.1),
    ok: withOpacity(rampColor(ramps.green, 0.5), 0.15),
    error: withOpacity(rampColor(ramps.red, 0.5), 0.15),
    warning: withOpacity(rampColor(ramps.yellow, 0.5), 0.15),
    info: withOpacity(rampColor(ramps.blue, 0.5), 0.15),
  };

  const textColor = {
    primary: rampColor(ramps.neutral, 6),
    secondary: rampColor(ramps.neutral, 5),
    muted: rampColor(ramps.neutral, 5),
    placeholder: rampColor(ramps.neutral, 4),
    active: rampColor(ramps.neutral, 7),
    feature: rampColor(ramps.blue, 0.5),
    ok: rampColor(ramps.green, 0.5),
    error: rampColor(ramps.red, 0.5),
    warning: rampColor(ramps.yellow, 0.5),
    info: rampColor(ramps.blue, 0.5),
  };

  const player = {
    1: buildPlayer(rampColor(ramps.blue, 0.5)),
    2: buildPlayer(rampColor(ramps.green, 0.5)),
    3: buildPlayer(rampColor(ramps.magenta, 0.5)),
    4: buildPlayer(rampColor(ramps.orange, 0.5)),
    5: buildPlayer(rampColor(ramps.violet, 0.5)),
    6: buildPlayer(rampColor(ramps.cyan, 0.5)),
    7: buildPlayer(rampColor(ramps.red, 0.5)),
    8: buildPlayer(rampColor(ramps.yellow, 0.5)),
  };

  const editor = {
    background: backgroundColor[500].base,
    indent_guide: borderColor.muted,
    indent_guide_active: borderColor.secondary,
    line: {
      active: rampColor(ramps.neutral, 1),
      highlighted: rampColor(ramps.neutral, 1.25), // TODO: Where is this used?
    },
    highlight: {
      selection: player[1].selectionColor,
      occurrence: withOpacity(rampColor(ramps.neutral, 3.5), blend),
      activeOccurrence: withOpacity(rampColor(ramps.neutral, 3.5), blend * 2), // TODO: Not hooked up - https://github.com/zed-industries/zed/issues/751
      matchingBracket: backgroundColor[500].active, // TODO: Not hooked up
      match: rampColor(ramps.violet, 0.15),
      activeMatch: withOpacity(rampColor(ramps.violet, 0.4), blend * 2), // TODO: Not hooked up - https://github.com/zed-industries/zed/issues/751
      related: backgroundColor[500].hovered,
    },
    gutter: {
      primary: textColor.placeholder,
      active: textColor.active,
    },
  };

  const syntax: Syntax = {
    primary: {
      color: rampColor(ramps.neutral, 7),
      weight: fontWeights.normal,
    },
    comment: {
      color: rampColor(ramps.neutral, 5),
      weight: fontWeights.normal,
    },
    punctuation: {
      color: rampColor(ramps.neutral, 6),
      weight: fontWeights.normal,
    },
    constant: {
      color: rampColor(ramps.neutral, 4),
      weight: fontWeights.normal,
    },
    keyword: {
      color: rampColor(ramps.blue, 0.5),
      weight: fontWeights.normal,
    },
    function: {
      color: rampColor(ramps.yellow, 0.5),
      weight: fontWeights.normal,
    },
    type: {
      color: rampColor(ramps.cyan, 0.5),
      weight: fontWeights.normal,
    },
    variant: {
      color: rampColor(ramps.blue, 0.5),
      weight: fontWeights.normal,
    },
    property: {
      color: rampColor(ramps.blue, 0.5),
      weight: fontWeights.normal,
    },
    enum: {
      color: rampColor(ramps.orange, 0.5),
      weight: fontWeights.normal,
    },
    operator: {
      color: rampColor(ramps.orange, 0.5),
      weight: fontWeights.normal,
    },
    string: {
      color: rampColor(ramps.orange, 0.5),
      weight: fontWeights.normal,
    },
    number: {
      color: rampColor(ramps.green, 0.5),
      weight: fontWeights.normal,
    },
    boolean: {
      color: rampColor(ramps.green, 0.5),
      weight: fontWeights.normal,
    },
    predictive: {
      color: textColor.muted,
      weight: fontWeights.normal,
    },
    title: {
      color: rampColor(ramps.yellow, 0.5),
      weight: fontWeights.bold,
    },
    emphasis: {
      color: textColor.feature,
      weight: fontWeights.normal,
    },
    "emphasis.strong": {
      color: textColor.feature,
      weight: fontWeights.bold,
    },
    linkUri: {
      color: rampColor(ramps.green, 0.5),
      weight: fontWeights.normal,
      underline: true,
    },
    linkText: {
      color: rampColor(ramps.orange, 0.5),
      weight: fontWeights.normal,
      italic: true,
    },
  };

  const shadowAlpha: NumberToken = {
    value: blend,
    type: "number",
  };

  return {
    name,
    backgroundColor,
    borderColor,
    textColor,
    iconColor: textColor,
    editor,
    syntax,
    player,
    shadowAlpha,
  };
}
