import { ColorToken, fontWeights, NumberToken } from "../tokens";
import { withOpacity } from "../utils/color";
import Theme, { buildPlayer, Syntax } from "./theme";

export interface Accents {
  "red": ColorToken,
  "orange": ColorToken,
  "yellow": ColorToken,
  "green": ColorToken,
  "cyan": ColorToken,
  "blue": ColorToken,
  "violet": ColorToken,
  "magenta": ColorToken,
}

export function createTheme(name: string, isLight: boolean, neutral: ColorToken[], accent: Accents): Theme {
  if (isLight) {
    neutral = [...neutral].reverse();
  }
  let blend = isLight ? 0.12 : 0.24;

  const backgroundColor = {
    100: {
      base: neutral[1],
      hovered: withOpacity(neutral[2], blend),
      active: withOpacity(neutral[2], blend * 1.5),
      focused: withOpacity(neutral[2], blend),
    },
    300: {
      base: neutral[1],
      hovered: withOpacity(neutral[2], blend),
      active: withOpacity(neutral[2], blend * 1.5),
      focused: withOpacity(neutral[2], blend),
    },
    500: {
      base: neutral[0],
      hovered: withOpacity(neutral[1], blend),
      active: withOpacity(neutral[1], blend * 1.5),
      focused: withOpacity(neutral[1], blend),
    },
    on300: {
      base: neutral[0],
      hovered: withOpacity(neutral[1], blend),
      active: withOpacity(neutral[1], blend * 2),
      focused: withOpacity(neutral[1], blend),
    },
    on500: {
      base: neutral[1],
      hovered: withOpacity(neutral[2], blend),
      active: withOpacity(neutral[2], blend * 2),
      focused: withOpacity(neutral[2], blend),
    },
    ok: {
      base: withOpacity(accent.green, 0.15),
      hovered: withOpacity(accent.green, 0.20),
      active: withOpacity(accent.green, 0.25),
      focused: withOpacity(accent.green, 0.20),
    },
    error: {
      base: withOpacity(accent.red, 0.15),
      hovered: withOpacity(accent.red, 0.20),
      active: withOpacity(accent.red, 0.25),
      focused: withOpacity(accent.red, 0.20),
    },
    warning: {
      base: withOpacity(accent.yellow, 0.15),
      hovered: withOpacity(accent.yellow, 0.20),
      active: withOpacity(accent.yellow, 0.25),
      focused: withOpacity(accent.yellow, 0.20),
    },
    info: {
      base: withOpacity(accent.blue, 0.15),
      hovered: withOpacity(accent.blue, 0.20),
      active: withOpacity(accent.blue, 0.25),
      focused: withOpacity(accent.blue, 0.20),
    },
  };

  const borderColor = {
    primary: neutral[0],
    secondary: neutral[1],
    muted: neutral[3],
    focused: neutral[3],
    active: neutral[3],
    onMedia: withOpacity(neutral[0], 0.1),
    ok: withOpacity(accent.green, 0.15),
    error: withOpacity(accent.red, 0.15),
    warning: withOpacity(accent.yellow, 0.15),
    info: withOpacity(accent.blue, 0.15),
  };

  const textColor = {
    primary: neutral[6],
    secondary: neutral[5],
    muted: neutral[5],
    placeholder: neutral[4],
    active: neutral[7],
    feature: accent.blue,
    ok: accent.green,
    error: accent.red,
    warning: accent.yellow,
    info: accent.blue,
  };

  const player = {
    1: buildPlayer(accent.blue),
    2: buildPlayer(accent.green),
    3: buildPlayer(accent.magenta),
    4: buildPlayer(accent.orange),
    5: buildPlayer(accent.violet),
    6: buildPlayer(accent.cyan),
    7: buildPlayer(accent.red),
    8: buildPlayer(accent.yellow),
  };

  const editor = {
    background: backgroundColor[500].base,
    indent_guide: borderColor.muted,
    indent_guide_active: borderColor.secondary,
    line: {
      active: withOpacity(neutral[7], 0.07),
      highlighted: withOpacity(neutral[7], 0.12),
      inserted: backgroundColor.ok.active,
      deleted: backgroundColor.error.active,
      modified: backgroundColor.info.active,
    },
    highlight: {
      selection: player[1].selectionColor,
      occurrence: withOpacity(neutral[7], blend / 2),
      activeOccurrence: withOpacity(neutral[7], blend),
      matchingBracket: backgroundColor[500].active,
      match: withOpacity(accent.violet, blend * 2),
      activeMatch: withOpacity(accent.violet, blend * 3),
      related: backgroundColor[500].focused,
    },
    gutter: {
      primary: textColor.placeholder,
      active: textColor.active,
    },
  };

  const syntax: Syntax = {
    primary: {
      color: neutral[7],
      weight: fontWeights.normal,
    },
    comment: {
      color: neutral[5],
      weight: fontWeights.normal,
    },
    punctuation: {
      color: neutral[5],
      weight: fontWeights.normal,
    },
    constant: {
      color: neutral[4],
      weight: fontWeights.normal,
    },
    keyword: {
      color: accent.blue,
      weight: fontWeights.normal,
    },
    function: {
      color: accent.yellow,
      weight: fontWeights.normal,
    },
    type: {
      color: accent.cyan,
      weight: fontWeights.normal,
    },
    variant: {
      color: accent.blue,
      weight: fontWeights.normal,
    },
    property: {
      color: accent.blue,
      weight: fontWeights.normal,
    },
    enum: {
      color: accent.orange,
      weight: fontWeights.normal,
    },
    operator: {
      color: accent.orange,
      weight: fontWeights.normal,
    },
    string: {
      color: accent.orange,
      weight: fontWeights.normal,
    },
    number: {
      color: accent.green,
      weight: fontWeights.normal,
    },
    boolean: {
      color: accent.green,
      weight: fontWeights.normal,
    },
    predictive: {
      color: textColor.muted,
      weight: fontWeights.normal,
    },
    title: {
      color: accent.yellow,
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
      color: accent.green,
      weight: fontWeights.normal,
      underline: true,
    },
    linkText: {
      color: accent.orange,
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