import { colors, fontWeights, NumberToken } from "../tokens";
import { withOpacity } from "../utils/color";
import Theme, { buildPlayer, Syntax } from "./theme";

const backgroundColor = {
  100: {
    base: colors.neutral[75],
    hovered: colors.neutral[100],
    active: colors.neutral[150],
    focused: colors.neutral[100],
  },
  300: {
    base: colors.neutral[25],
    hovered: colors.neutral[75],
    active: colors.neutral[100],
    focused: colors.neutral[75],
  },
  500: {
    base: colors.neutral[0],
    hovered: withOpacity(colors.neutral[900], 0.03),
    active: withOpacity(colors.neutral[900], 0.06),
    focused: colors.neutral[50],
  },
  on300: {
    base: colors.neutral[50],
    hovered: colors.neutral[100],
    active: colors.neutral[150],
    focused: colors.neutral[100],
  },
  on500: {
    base: colors.neutral[50],
    hovered: colors.neutral[25],
    active: colors.neutral[0],
    focused: colors.neutral[25],
  },
  ok: {
    base: colors.green[100],
    hovered: colors.green[100],
    active: colors.green[100],
    focused: colors.green[100],
  },
  error: {
    base: colors.red[100],
    hovered: colors.red[100],
    active: colors.red[100],
    focused: colors.red[100],
  },
  warning: {
    base: colors.yellow[100],
    hovered: colors.yellow[100],
    active: colors.yellow[100],
    focused: colors.yellow[100],
  },
  info: {
    base: colors.blue[100],
    hovered: colors.blue[100],
    active: colors.blue[100],
    focused: colors.blue[100],
  },
};

const borderColor = {
  primary: colors.neutral[150],
  secondary: colors.neutral[150],
  muted: colors.neutral[100],
  focused: colors.indigo[500],
  active: colors.neutral[250],
  ok: colors.green[200],
  error: colors.red[200],
  warning: colors.yellow[200],
  info: colors.blue[200],
};

const textColor = {
  primary: colors.neutral[750],
  secondary: colors.neutral[650],
  muted: colors.neutral[550],
  placeholder: colors.neutral[450],
  active: colors.neutral[900],
  feature: colors.indigo[500],
  ok: colors.green[500],
  error: colors.red[500],
  warning: colors.yellow[500],
  info: colors.blue[500],
};

const iconColor = {
  primary: colors.neutral[700],
  secondary: colors.neutral[500],
  muted: colors.neutral[350],
  placeholder: colors.neutral[300],
  active: colors.neutral[900],
  feature: colors.indigo[500],
  ok: colors.green[600],
  error: colors.red[600],
  warning: colors.yellow[400],
  info: colors.blue[600],
};

const player = {
  1: buildPlayer(colors.blue[500]),
  2: buildPlayer(colors.emerald[400]),
  3: buildPlayer(colors.fuschia[400]),
  4: buildPlayer(colors.orange[400]),
  5: buildPlayer(colors.purple[400]),
  6: buildPlayer(colors.teal[400]),
  7: buildPlayer(colors.pink[400]),
  8: buildPlayer(colors.yellow[400]),
};

const editor = {
  background: backgroundColor[500].base,
  indent_guide: borderColor.muted,
  indent_guide_active: borderColor.secondary,
  line: {
    active: withOpacity(colors.neutral[900], 0.06),
    highlighted: withOpacity(colors.neutral[900], 0.12),
    inserted: backgroundColor.ok.active,
    deleted: backgroundColor.error.active,
    modified: backgroundColor.info.active,
  },
  highlight: {
    selection: player[1].selectionColor,
    occurrence: withOpacity(colors.neutral[900], 0.06),
    activeOccurrence: withOpacity(colors.neutral[900], 0.16), // TODO: This is not hooked up to occurences on the rust side
    matchingBracket: colors.neutral[0],
    match: colors.yellow[100],
    activeMatch: colors.yellow[200], // TODO: This is not hooked up to occurences on the rust side
    related: colors.neutral[0],
  },
  gutter: {
    primary: colors.neutral[300],
    active: textColor.active,
  },
};

const syntax: Syntax = {
  primary: {
    color: colors.neutral[800],
    weight: fontWeights.normal,
  },
  comment: {
    color: colors.neutral[500],
    weight: fontWeights.normal,
  },
  punctuation: {
    color: colors.neutral[600],
    weight: fontWeights.normal,
  },
  constant: {
    color: colors.neutral[800],
    weight: fontWeights.normal,
  },
  keyword: {
    color: colors.indigo[700],
    weight: fontWeights.normal,
  },
  function: {
    color: colors.orange[600],
    weight: fontWeights.normal,
  },
  type: {
    color: colors.yellow[600],
    weight: fontWeights.normal,
  },
  variant: {
    color: colors.rose[700],
    weight: fontWeights.normal,
  },
  property: {
    color: colors.emerald[700],
    weight: fontWeights.normal,
  },
  enum: {
    color: colors.red[500],
    weight: fontWeights.normal,
  },
  operator: {
    color: colors.red[500],
    weight: fontWeights.normal,
  },
  string: {
    color: colors.red[500],
    weight: fontWeights.normal,
  },
  number: {
    color: colors.indigo[500],
    weight: fontWeights.normal,
  },
  boolean: {
    color: colors.red[500],
    weight: fontWeights.normal,
  },
  predictive: {
    color: textColor.placeholder,
    weight: fontWeights.normal,
  },
  title: {
    color: colors.sky[500],
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
    color: colors.lime[500],
    weight: fontWeights.normal,
    underline: true
  },
  linkText: {
    color: colors.red[500],
    weight: fontWeights.normal,
    italic: true
  },
};

const shadowAlpha: NumberToken = {
  value: 0.12,
  type: "number",
};

const theme: Theme = {
  name: "light",
  backgroundColor,
  borderColor,
  textColor,
  iconColor,
  editor,
  syntax,
  player,
  shadowAlpha,
};

export default theme;
