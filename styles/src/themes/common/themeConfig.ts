import { Color, Scale } from "chroma-js";
import { Syntax, SyntaxHighlightStyle } from "./theme";

export interface Meta {
  name: string,
  author: string,
  url: string,
  license: License
}

export interface License {
  SPDX: SPDXExpression,
  /// A url where we can download the license's text
  https_url: string,
  license_checksum: string
}

// FIXME: Add support for the SPDX expression syntax
export type SPDXExpression = "MIT";


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
  // Syntax probably moves inside of override
  syntax?: Partial<Syntax>;
  override?: Partial<ThemeOverrides>
}
