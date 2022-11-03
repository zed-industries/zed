import { Color, Scale } from "chroma-js";
import { Syntax, SyntaxHighlightStyle } from "./theme";

interface Meta {
  /**
   * The theme's name, with no variants or modifiers
   *
   * For example:
   *
   * - Gruvbox, not Gruvbox Dark Hard
   */
  name: string;

  /**
   * Name of the theme's author, or the author who ported the theme
   */
  author: string;
  url: string;
  license: License;
}

interface License {
  type: string;
  url: string;
}

interface Colors {
  neutral: Scale<Color>;
  red: Scale<Color>;
  orange: Scale<Color>;
  yellow: Scale<Color>;
  green: Scale<Color>;
  cyan: Scale<Color>;
  blue: Scale<Color>;
  violet: Scale<Color>;
  magenta: Scale<Color>;
}

export interface SyntaxOverrides {
  [key: string]: SyntaxHighlightStyle
}

export interface ThemeConfig {
  meta: Meta;
  color: Colors;
  syntax: SyntaxOverrides;
}
