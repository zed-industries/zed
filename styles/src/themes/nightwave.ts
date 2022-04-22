import { colors, fontWeights, NumberToken } from "../tokens";
import { withOpacity } from "../utils/color";
import Theme, { buildPlayer, Syntax } from "./theme";

const backgroundColor = {
  100: {
    base: colors.purple[900],
    hovered: withOpacity(colors.neutral[900], 0.12),
    active: withOpacity(colors.neutral[900], 0.24),
    focused: withOpacity(colors.neutral[900], 0.12)
  },
  300: {
    base: colors.purple[900],
    hovered: withOpacity(colors.neutral[900], 0.12),
    active: withOpacity(colors.neutral[900], 0.24),
    focused: withOpacity(colors.neutral[900], 0.12)
  },
  500: {
    base: colors.purple[900],
    hovered: withOpacity(colors.neutral[900], 0.12),
    active: withOpacity(colors.neutral[900], 0.24),
    focused: withOpacity(colors.neutral[900], 0.12)
  },
  on300: {
    base: withOpacity(colors.neutral[0], 0.12),
    hovered: withOpacity(colors.neutral[0], 0.24),
    active: withOpacity(colors.neutral[0], 0.32),
    focused: withOpacity(colors.neutral[0], 0.24),
  },
  on500: {
    base: withOpacity(colors.neutral[900], 0.12),
    hovered: withOpacity(colors.neutral[900], 0.24),
    active: withOpacity(colors.neutral[900], 0.32),
    focused: withOpacity(colors.neutral[900], 0.24),
  },
  ok: {
    base: colors.green[600],
    hovered: colors.green[600],
    active: colors.green[600],
    focused: colors.green[600],
  },
  error: {
    base: colors.red[400],
    hovered: colors.red[400],
    active: colors.red[400],
    focused: colors.red[400],
  },
  warning: {
    base: colors.amber[300],
    hovered: colors.amber[300],
    active: colors.amber[300],
    focused: colors.amber[300],
  },
  info: {
    base: colors.blue[500],
    hovered: colors.blue[500],
    active: colors.blue[500],
    focused: colors.blue[500],
  },
};

const borderColor = {
  primary: withOpacity(colors.neutral[0], 0.08),
  secondary: withOpacity(colors.neutral[0], 0.12),
  muted: withOpacity(colors.neutral[0], 0.16),
  focused: colors.indigo[500],
  active: colors.neutral[900],
  ok: colors.green[500],
  error: colors.red[500],
  warning: colors.amber[500],
  info: colors.blue[500],
};

const textColor = {
  primary: withOpacity(colors.neutral[0], 0.95),
  secondary: withOpacity(colors.neutral[0], 0.65),
  muted: withOpacity(colors.neutral[0], 0.45),
  placeholder: withOpacity(colors.neutral[0], 0.25),
  active: colors.neutral[0],
  //TODO: (design) define feature and it's correct value
  feature: colors.pink[500],
  ok: colors.lime[500],
  error: colors.rose[500],
  warning: colors.yellow[400],
  info: colors.blue[500],
};

const iconColor = {
  primary: colors.neutral[0],
  secondary: withOpacity(colors.neutral[0], 0.70),
  muted: withOpacity(colors.neutral[0], 0.55),
  placeholder: withOpacity(colors.neutral[0], 0.35),
  active: colors.pink[500],
  //TODO: (design) define feature and it's correct value
  feature: colors.teal[500],
  ok: colors.green[600],
  error: colors.red[500],
  warning: colors.amber[400],
  info: colors.blue[600],
};

const player = {
  1: buildPlayer(colors.pink[500]),
  2: buildPlayer(colors.teal[500]),
  3: buildPlayer(colors.indigo[500]),
  4: buildPlayer(colors.yellow[500]),
  5: buildPlayer(colors.lime[500]),
  6: buildPlayer(colors.purple[400]),
  7: buildPlayer(colors.pink[700]),
  8: buildPlayer(colors.teal[700]),
};

const editor = {
  background: backgroundColor[500].base,
  indent_guide: borderColor.muted,
  indent_guide_active: borderColor.secondary,
  line: {
    active: withOpacity(colors.neutral[0], 0.07),
    highlighted: withOpacity(colors.neutral[0], 0.12),
    inserted: backgroundColor.ok.active,
    deleted: backgroundColor.error.active,
    modified: backgroundColor.info.active,
  },
  highlight: {
    selection: player[1].selectionColor,
    occurrence: withOpacity(colors.neutral[0], 0.12),
    activeOccurrence: withOpacity(colors.neutral[0], 0.16), // TODO: This is not correctly hooked up to occurences on the rust side
    matchingBracket: backgroundColor[500].active,
    match: withOpacity(colors.violet[700], 0.5),
    activeMatch: withOpacity(colors.violet[600], 0.7),
    related: backgroundColor[500].focused,
  },
  gutter: {
    primary: textColor.placeholder,
    active: textColor.active,
  },
};

const syntax: Syntax = {
  primary: {
    color: colors.pink[400],
    weight: fontWeights.normal,
  },
  comment: {
    color: colors.neutral[300],
    weight: fontWeights.normal,
  },
  punctuation: {
    color: colors.pink[400],
    weight: fontWeights.normal,
  },
  constant: {
    color: colors.yellow[300],
    weight: fontWeights.normal,
  },
  keyword: {
    color: colors.yellow[300],
    weight: fontWeights.normal,
  },
  function: {
    color: colors.teal[300],
    weight: fontWeights.normal,
  },
  type: {
    color: colors.red[500],
    weight: fontWeights.normal,
  },
  variant: {
    color: colors.teal[300],
    weight: fontWeights.normal,
  },
  property: {
    color: colors.pink[400],
    weight: fontWeights.normal,
  },
  enum: {
    color: colors.orange[500],
    weight: fontWeights.normal,
  },
  operator: {
    color: colors.orange[500],
    weight: fontWeights.normal,
  },
  string: {
    color: colors.emerald[400],
    weight: fontWeights.normal,
  },
  number: {
    color: colors.yellow[300],
    weight: fontWeights.normal,
  },
  boolean: {
    color: colors.yellow[300],
    weight: fontWeights.normal,
  },
  predictive: {
    color: textColor.muted,
    weight: fontWeights.normal,
  },
  title: {
    color: colors.red[500],
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
    color: colors.yellow[300],
    weight: fontWeights.normal,
    // TODO: add underline
  },
  linkText: {
    color: colors.neutral[200],
    weight: fontWeights.normal,
    // TODO: add italic
  },
};

const shadowAlpha: NumberToken = {
  value: 0.16,
  type: "number",
};

const theme: Theme = {
  name: "nightwave",
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
