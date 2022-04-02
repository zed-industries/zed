import { colors, fontWeights, NumberToken } from "../tokens";
import Theme, { Syntax } from "./theme";

// TODO: Replace with light values

const backgroundColor = {
  100: {
    base: colors.neutral[100],
    hovered: colors.neutral[150],
    active: colors.neutral[200],
    focused: colors.neutral[150],
  },
  300: {
    base: colors.neutral[50],
    hovered: colors.neutral[100],
    active: colors.neutral[150],
    focused: colors.neutral[100],
  },
  500: {
    base: colors.neutral[0],
    hovered: colors.neutral[50],
    active: colors.neutral[100],
    focused: colors.neutral[50],
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
  primary: colors.neutral[200],
  secondary: colors.neutral[100],
  muted: colors.neutral[50],
  focused: colors.neutral[100],
  active: colors.neutral[250],
  ok: colors.green[200],
  error: colors.red[200],
  warning: colors.yellow[200],
  info: colors.blue[200],
};

const textColor = {
  primary: colors.neutral[750],
  secondary: colors.neutral[600],
  muted: colors.neutral[450],
  placeholder: colors.neutral[300],
  active: colors.neutral[900],
  feature: colors.blue[500],
  ok: colors.green[500],
  error: colors.red[500],
  warning: colors.yellow[500],
  info: colors.blue[500],
};

const iconColor = {
  primary: colors.neutral[300],
  secondary: colors.neutral[500],
  muted: colors.neutral[600],
  placeholder: colors.neutral[700],
  active: colors.neutral[900],
  feature: colors.sky[600],
  ok: colors.green[600],
  error: colors.red[600],
  warning: colors.yellow[400],
  info: colors.blue[600],
};

const player = {
  1: {
    baseColor: colors.blue[600],
    cursorColor: colors.blue[500],
    selectionColor: colors.blue[100],
    borderColor: colors.blue[500],
  },
  2: {
    baseColor: colors.lime[500],
    cursorColor: colors.lime[500],
    selectionColor: colors.lime[100],
    borderColor: colors.lime[500],
  },
  3: {
    baseColor: colors.indigo[500],
    cursorColor: colors.indigo[500],
    selectionColor: colors.indigo[100],
    borderColor: colors.indigo[500],
  },
  4: {
    baseColor: colors.orange[500],
    cursorColor: colors.orange[500],
    selectionColor: colors.orange[100],
    borderColor: colors.orange[500],
  },
  5: {
    baseColor: colors.purple[500],
    cursorColor: colors.purple[500],
    selectionColor: colors.purple[100],
    borderColor: colors.purple[500],
  },
  6: {
    baseColor: colors.teal[400],
    cursorColor: colors.teal[400],
    selectionColor: colors.teal[100],
    borderColor: colors.teal[400],
  },
  7: {
    baseColor: colors.pink[400],
    cursorColor: colors.pink[400],
    selectionColor: colors.pink[100],
    borderColor: colors.pink[400],
  },
  8: {
    baseColor: colors.yellow[400],
    cursorColor: colors.yellow[400],
    selectionColor: colors.yellow[100],
    borderColor: colors.yellow[400],
  },
};

// TODO: Fixup
const editor = {
  background: backgroundColor[500].base,
  indent_guide: borderColor.muted,
  indent_guide_active: borderColor.secondary,
  line: {
    active: backgroundColor[500].active,
    highlighted: backgroundColor[500].active,
    inserted: backgroundColor.ok.active,
    deleted: backgroundColor.error.active,
    modified: backgroundColor.info.active,
  },
  highlight: {
    selection: player[1].selectionColor,
    occurrence: backgroundColor[500].active,
    activeOccurrence: colors.neutral[0],
    matchingBracket: colors.neutral[0],
    match: colors.neutral[0],
    activeMatch: colors.neutral[0],
    related: colors.neutral[0],
  },
  gutter: {
    primary: textColor.muted,
    active: textColor.active,
  },
};

const syntax: Syntax = {
  primary: {
    color: textColor.primary,
    weight: fontWeights.normal,
  },
  comment: {
    color: colors.lime[200],
    weight: fontWeights.normal,
  },
  punctuation: {
    color: textColor.primary,
    weight: fontWeights.normal,
  },
  constant: {
    color: colors.neutral[150],
    weight: fontWeights.normal,
  },
  keyword: {
    color: colors.sky[400],
    weight: fontWeights.normal,
  },
  function: {
    color: colors.yellow[200],
    weight: fontWeights.normal,
  },
  type: {
    color: colors.teal[300],
    weight: fontWeights.normal,
  },
  variant: {
    color: colors.teal[300],
    weight: fontWeights.normal,
  },
  property: {
    color: colors.sky[300],
    weight: fontWeights.normal,
  },
  enum: {
    color: colors.sky[400],
    weight: fontWeights.normal,
  },
  operator: {
    color: colors.sky[400],
    weight: fontWeights.normal,
  },
  string: {
    color: colors.orange[300],
    weight: fontWeights.normal,
  },
  number: {
    color: colors.neutral[150],
    weight: fontWeights.normal,
  },
  boolean: {
    color: colors.neutral[150],
    weight: fontWeights.normal,
  },
  predictive: {
    color: textColor.muted,
    weight: fontWeights.normal,
  },
  title: {
    color: colors.sky[500],
    weight: fontWeights.bold,
  },
  emphasis: {
    color: textColor.active,
    weight: fontWeights.normal,
  },
  emphasisStrong: {
    color: textColor.active,
    weight: fontWeights.bold,
  },
  linkUrl: {
    color: colors.lime[500],
    weight: fontWeights.normal,
    // TODO: add underline
  },
  linkText: {
    color: colors.orange[500],
    weight: fontWeights.normal,
    // TODO: add italic
  },
  listMarker: {
    color: colors.sky[400],
    weight: fontWeights.normal,
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
