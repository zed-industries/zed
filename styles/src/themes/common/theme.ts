import { Scale } from "chroma-js";
import { FontWeight } from "../../common";
import { withOpacity } from "../../utils/color";

export interface SyntaxHighlightStyle {
  color: string;
  weight?: FontWeight;
  underline?: boolean;
  italic?: boolean;
}

export interface Player {
  baseColor: string;
  cursorColor: string;
  selectionColor: string;
  borderColor: string;
}
export function buildPlayer(
  color: string,
  cursorOpacity?: number,
  selectionOpacity?: number,
  borderOpacity?: number
) {
  return {
    baseColor: color,
    cursorColor: withOpacity(color, cursorOpacity || 1.0),
    selectionColor: withOpacity(color, selectionOpacity || 0.24),
    borderColor: withOpacity(color, borderOpacity || 0.8),
  };
}

export interface BackgroundColorSet {
  base: string;
  hovered: string;
  active: string;
}

export interface Syntax {
  primary: SyntaxHighlightStyle;
  comment: SyntaxHighlightStyle;
  punctuation: SyntaxHighlightStyle;
  constant: SyntaxHighlightStyle;
  keyword: SyntaxHighlightStyle;
  function: SyntaxHighlightStyle;
  type: SyntaxHighlightStyle;
  variant: SyntaxHighlightStyle;
  property: SyntaxHighlightStyle;
  enum: SyntaxHighlightStyle;
  operator: SyntaxHighlightStyle;
  string: SyntaxHighlightStyle;
  number: SyntaxHighlightStyle;
  boolean: SyntaxHighlightStyle;
  predictive: SyntaxHighlightStyle;
  title: SyntaxHighlightStyle;
  emphasis: SyntaxHighlightStyle;
  linkUri: SyntaxHighlightStyle;
  linkText: SyntaxHighlightStyle;

  [key: string]: SyntaxHighlightStyle;
}

export default interface Theme {
  name: string;
  isLight: boolean,
  backgroundColor: {
    // Basically just Title Bar
    // Lowest background level
    100: BackgroundColorSet;
    // Tab bars, panels, popovers
    // Mid-ground
    300: BackgroundColorSet;
    // The editor
    // Foreground
    500: BackgroundColorSet;
    // Hacks for elements on top of the midground
    // Buttons in a panel, tab bar, or panel
    on300: BackgroundColorSet;
    // Hacks for elements on top of the editor
    on500: BackgroundColorSet;
    ok: BackgroundColorSet;
    error: BackgroundColorSet;
    warning: BackgroundColorSet;
    info: BackgroundColorSet;
  };
  borderColor: {
    primary: string;
    secondary: string;
    muted: string;
    active: string;
    /**
     * Used for rendering borders on top of media like avatars, images, video, etc.
     */
    onMedia: string;
    ok: string;
    error: string;
    warning: string;
    info: string;
  };
  textColor: {
    primary: string;
    secondary: string;
    muted: string;
    placeholder: string;
    active: string;
    feature: string;
    ok: string;
    error: string;
    warning: string;
    info: string;
    onMedia: string;
  };
  iconColor: {
    primary: string;
    secondary: string;
    muted: string;
    placeholder: string;
    active: string;
    feature: string;
    ok: string;
    error: string;
    warning: string;
    info: string;
  };
  editor: {
    background: string;
    indent_guide: string;
    indent_guide_active: string;
    line: {
      active: string;
      highlighted: string;
    };
    highlight: {
      selection: string;
      occurrence: string;
      activeOccurrence: string;
      matchingBracket: string;
      match: string;
      activeMatch: string;
      related: string;
    };
    gutter: {
      primary: string;
      active: string;
    };
  };

  syntax: Syntax;

  player: {
    1: Player;
    2: Player;
    3: Player;
    4: Player;
    5: Player;
    6: Player;
    7: Player;
    8: Player;
  },
  shadow: string;
  ramps: { [rampName: string]: Scale };
}
