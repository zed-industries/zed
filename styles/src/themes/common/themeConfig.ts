import { Color, Scale } from "chroma-js";
import { SyntaxHighlightStyle } from "./theme";

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
   *
   * Github handle > full name, if both are available format like "@handle (Full Name)"
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

interface ThemeOverrides {
  accent: string;
  ui: {
    scrollbar: string,
  }
  status: {
    positive: string,
    negative: string,
    warning: string,
    info: string,
  }
  versionControl: {
    added: string,
    removed: string,
    modified: string
  }
}

export interface SyntaxOverrides {
  [key: string]: SyntaxHighlightStyle
}

export interface ThemeConfig {
  meta: Meta;
  color: Colors;
  // Syntax probably moves inside of override
  syntax: SyntaxOverrides;
  override: Partial<ThemeOverrides>
}
