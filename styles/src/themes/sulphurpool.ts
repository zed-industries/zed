import { color, fontWeights, NumberToken } from "../tokens";
import { withOpacity } from "../utils/color";
import Theme, { buildPlayer, Syntax } from "./theme";

// Dark: 0 == Darkest, 3 == Lightest
const dark = {
  0: color("#202746"),
  1: color("#293256"),
  2: color("#5e6687"),
  3: color("#6b7394"),
};
// Light: 0 == Lightest, 3 == Darkest
const light = {
  0: color("#f5f7ff"),
  1: color("#dfe2f1"),
  2: color("#979db4"),
  3: color("#898ea4"),
};

const colors = {
  "red": color("#c94922"),
  "orange": color("#c76b29"),
  "yellow": color("#c08b30"),
  "green": color("#ac9739"),
  "cyan": color("#22a2c9"),
  "blue": color("#3d8fd1"),
  "violet": color("#6679cc"),
  "magenta": color("#9c637a"),
};

export function sulphurpool(darkTheme: boolean): Theme {
  let fg = darkTheme ? light : dark;
  let bg = darkTheme ? dark : light;
  let name = darkTheme ? "sulphurpool-dark" : "sulphurpool-light";

  const backgroundColor = {
    100: {
      base: bg[1],
      hovered: bg[3],
      active: bg[3],
      focused: bg[3],
    },
    300: {
      base: bg[1],
      hovered: bg[3],
      active: bg[3],
      focused: bg[3],
    },
    500: {
      base: bg[0],
      hovered: bg[1],
      active: bg[1],
      focused: bg[1],
    },
    on300: {
      base: bg[0],
      hovered: bg[1],
      active: bg[1],
      focused: bg[1],
    },
    on500: {
      base: bg[1],
      hovered: bg[3],
      active: bg[3],
      focused: bg[3],
    },
    ok: {
      base: colors.green,
      hovered: colors.green,
      active: colors.green,
      focused: colors.green,
    },
    error: {
      base: colors.red,
      hovered: colors.red,
      active: colors.red,
      focused: colors.red,
    },
    warning: {
      base: colors.yellow,
      hovered: colors.yellow,
      active: colors.yellow,
      focused: colors.yellow,
    },
    info: {
      base: colors.blue,
      hovered: colors.blue,
      active: colors.blue,
      focused: colors.blue,
    },
  };

  const borderColor = {
    primary: bg[0],
    secondary: bg[1],
    muted: bg[3],
    focused: bg[3],
    active: bg[3],
    ok: colors.green,
    error: colors.red,
    warning: colors.yellow,
    info: colors.blue,
  };

  const textColor = {
    primary: fg[1],
    secondary: fg[2],
    muted: fg[2],
    placeholder: fg[3],
    active: fg[0],
    //TODO: (design) define feature and it's correct value
    feature: colors.blue,
    ok: colors.green,
    error: colors.red,
    warning: colors.yellow,
    info: colors.blue,
  };

  const player = {
    1: buildPlayer(colors.blue),
    2: buildPlayer(colors.green),
    3: buildPlayer(colors.magenta),
    4: buildPlayer(colors.orange),
    5: buildPlayer(colors.violet),
    6: buildPlayer(colors.cyan),
    7: buildPlayer(colors.red),
    8: buildPlayer(colors.yellow),
  };

  const editor = {
    background: backgroundColor[500].base,
    indent_guide: borderColor.muted,
    indent_guide_active: borderColor.secondary,
    line: {
      active: withOpacity(fg[0], 0.07),
      highlighted: withOpacity(fg[0], 0.12),
      inserted: backgroundColor.ok.active,
      deleted: backgroundColor.error.active,
      modified: backgroundColor.info.active,
    },
    highlight: {
      selection: player[1].selectionColor,
      occurrence: withOpacity(bg[0], 0.12),
      activeOccurrence: withOpacity(bg[0], 0.16), // TODO: This is not correctly hooked up to occurences on the rust side
      matchingBracket: backgroundColor[500].active,
      match: withOpacity(colors.violet, 0.5),
      activeMatch: withOpacity(colors.violet, 0.7),
      related: backgroundColor[500].focused,
    },
    gutter: {
      primary: textColor.placeholder,
      active: textColor.active,
    },
  };

  const syntax: Syntax = {
    primary: {
      color: fg[0],
      weight: fontWeights.normal,
    },
    comment: {
      color: fg[2],
      weight: fontWeights.normal,
    },
    punctuation: {
      color: fg[2],
      weight: fontWeights.normal,
    },
    constant: {
      color: fg[3],
      weight: fontWeights.normal,
    },
    keyword: {
      color: colors.blue,
      weight: fontWeights.normal,
    },
    function: {
      color: colors.yellow,
      weight: fontWeights.normal,
    },
    type: {
      color: colors.cyan,
      weight: fontWeights.normal,
    },
    variant: {
      color: colors.blue,
      weight: fontWeights.normal,
    },
    property: {
      color: colors.blue,
      weight: fontWeights.normal,
    },
    enum: {
      color: colors.orange,
      weight: fontWeights.normal,
    },
    operator: {
      color: colors.orange,
      weight: fontWeights.normal,
    },
    string: {
      color: colors.orange,
      weight: fontWeights.normal,
    },
    number: {
      color: colors.green,
      weight: fontWeights.normal,
    },
    boolean: {
      color: colors.green,
      weight: fontWeights.normal,
    },
    predictive: {
      color: textColor.muted,
      weight: fontWeights.normal,
    },
    title: {
      color: colors.yellow,
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
      color: colors.green,
      weight: fontWeights.normal,
      underline: true,
    },
    linkText: {
      color: colors.orange,
      weight: fontWeights.normal,
      italic: true,
    },
  };

  const shadowAlpha: NumberToken = {
    value: 0.32,
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