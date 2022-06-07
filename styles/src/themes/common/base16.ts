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
): Theme {
  if (isLight) {
    for (var rampName in ramps) {
      ramps[rampName] = ramps[rampName].domain([1, 0]);
    }
    ramps.neutral = ramps.neutral.domain([7, 0]);
  } else {
    ramps.neutral = ramps.neutral.domain([0, 7]);
  }

  let blend = isLight ? 0.12 : 0.24;

  function sample(ramp: Scale, index: number): ColorToken {
    return color(ramp(index).hex());
  }
  const darkest = color(ramps.neutral(isLight ? 7 : 0).hex());

  const backgroundColor = {
    // Title bar
    100: {
      base: sample(ramps.neutral, 1.25),
      hovered: sample(ramps.neutral, 1.5),
      active: sample(ramps.neutral, 1.75),
    },
    // Midground (panels, etc)
    300: {
      base: sample(ramps.neutral, 1),
      hovered: sample(ramps.neutral, 1.25),
      active: sample(ramps.neutral, 1.5),
    },
    // Editor
    500: {
      base: sample(ramps.neutral, 0),
      hovered: sample(ramps.neutral, 0.25),
      active: sample(ramps.neutral, 0.5),
    },
    on300: {
      base: sample(ramps.neutral, 0),
      hovered: sample(ramps.neutral, 0.25),
      active: sample(ramps.neutral, 0.5),
    },
    on500: {
      base: sample(ramps.neutral, 1.25),
      hovered: sample(ramps.neutral, 1.5),
      active: sample(ramps.neutral, 1.75),
    },
    ok: {
      base: withOpacity(sample(ramps.green, 0.5), 0.15),
      hovered: withOpacity(sample(ramps.green, 0.5), 0.2),
      active: withOpacity(sample(ramps.green, 0.5), 0.25),
    },
    error: {
      base: withOpacity(sample(ramps.red, 0.5), 0.15),
      hovered: withOpacity(sample(ramps.red, 0.5), 0.2),
      active: withOpacity(sample(ramps.red, 0.5), 0.25),
    },
    warning: {
      base: withOpacity(sample(ramps.yellow, 0.5), 0.15),
      hovered: withOpacity(sample(ramps.yellow, 0.5), 0.2),
      active: withOpacity(sample(ramps.yellow, 0.5), 0.25),
    },
    info: {
      base: withOpacity(sample(ramps.blue, 0.5), 0.15),
      hovered: withOpacity(sample(ramps.blue, 0.5), 0.2),
      active: withOpacity(sample(ramps.blue, 0.5), 0.25),
    },
  };

  const borderColor = {
    primary: sample(ramps.neutral, isLight ? 1.5 : 0),
    secondary: sample(ramps.neutral, isLight ? 1.25 : 1),
    muted: sample(ramps.neutral, isLight ? 1 : 3),
    active: sample(ramps.neutral, isLight ? 4 : 3),
    onMedia: withOpacity(darkest, 0.1),
    ok: withOpacity(sample(ramps.green, 0.5), 0.15),
    error: withOpacity(sample(ramps.red, 0.5), 0.15),
    warning: withOpacity(sample(ramps.yellow, 0.5), 0.15),
    info: withOpacity(sample(ramps.blue, 0.5), 0.15),
  };

  const textColor = {
    primary: sample(ramps.neutral, 6),
    secondary: sample(ramps.neutral, 5),
    muted: sample(ramps.neutral, 5),
    placeholder: sample(ramps.neutral, 4),
    active: sample(ramps.neutral, 7),
    feature: sample(ramps.blue, 0.5),
    ok: sample(ramps.green, 0.5),
    error: sample(ramps.red, 0.5),
    warning: sample(ramps.yellow, 0.5),
    info: sample(ramps.blue, 0.5),
    onMedia: darkest,
  };

  const player = {
    1: buildPlayer(sample(ramps.blue, 0.5)),
    2: buildPlayer(sample(ramps.green, 0.5)),
    3: buildPlayer(sample(ramps.magenta, 0.5)),
    4: buildPlayer(sample(ramps.orange, 0.5)),
    5: buildPlayer(sample(ramps.violet, 0.5)),
    6: buildPlayer(sample(ramps.cyan, 0.5)),
    7: buildPlayer(sample(ramps.red, 0.5)),
    8: buildPlayer(sample(ramps.yellow, 0.5)),
  };

  const editor = {
    background: backgroundColor[500].base,
    indent_guide: borderColor.muted,
    indent_guide_active: borderColor.secondary,
    line: {
      active: sample(ramps.neutral, 1),
      highlighted: sample(ramps.neutral, 1.25), // TODO: Where is this used?
    },
    highlight: {
      selection: player[1].selectionColor,
      occurrence: withOpacity(sample(ramps.neutral, 3.5), blend),
      activeOccurrence: withOpacity(sample(ramps.neutral, 3.5), blend * 2), // TODO: Not hooked up - https://github.com/zed-industries/zed/issues/751
      matchingBracket: backgroundColor[500].active, // TODO: Not hooked up
      match: sample(ramps.violet, 0.15),
      activeMatch: withOpacity(sample(ramps.violet, 0.4), blend * 2), // TODO: Not hooked up - https://github.com/zed-industries/zed/issues/751
      related: backgroundColor[500].hovered,
    },
    gutter: {
      primary: textColor.placeholder,
      active: textColor.active,
    },
  };

  const syntax: Syntax = {
    primary: {
      color: sample(ramps.neutral, 7),
      weight: fontWeights.normal,
    },
    comment: {
      color: sample(ramps.neutral, 5),
      weight: fontWeights.normal,
    },
    punctuation: {
      color: sample(ramps.neutral, 6),
      weight: fontWeights.normal,
    },
    constant: {
      color: sample(ramps.neutral, 4),
      weight: fontWeights.normal,
    },
    keyword: {
      color: sample(ramps.blue, 0.5),
      weight: fontWeights.normal,
    },
    function: {
      color: sample(ramps.yellow, 0.5),
      weight: fontWeights.normal,
    },
    type: {
      color: sample(ramps.cyan, 0.5),
      weight: fontWeights.normal,
    },
    variant: {
      color: sample(ramps.blue, 0.5),
      weight: fontWeights.normal,
    },
    property: {
      color: sample(ramps.blue, 0.5),
      weight: fontWeights.normal,
    },
    enum: {
      color: sample(ramps.orange, 0.5),
      weight: fontWeights.normal,
    },
    operator: {
      color: sample(ramps.orange, 0.5),
      weight: fontWeights.normal,
    },
    string: {
      color: sample(ramps.orange, 0.5),
      weight: fontWeights.normal,
    },
    number: {
      color: sample(ramps.green, 0.5),
      weight: fontWeights.normal,
    },
    boolean: {
      color: sample(ramps.green, 0.5),
      weight: fontWeights.normal,
    },
    predictive: {
      color: textColor.muted,
      weight: fontWeights.normal,
    },
    title: {
      color: sample(ramps.yellow, 0.5),
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
      color: sample(ramps.green, 0.5),
      weight: fontWeights.normal,
      underline: true,
    },
    linkText: {
      color: sample(ramps.orange, 0.5),
      weight: fontWeights.normal,
      italic: true,
    },
  };

  const shadow = withOpacity(
    color(ramps.neutral(isLight ? 7 : 0).darken().hex()),
    blend);

  return {
    name,
    backgroundColor,
    borderColor,
    textColor,
    iconColor: textColor,
    editor,
    syntax,
    player,
    shadow,
  };
}
