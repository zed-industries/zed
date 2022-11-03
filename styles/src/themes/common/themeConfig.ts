import { Color, Scale } from "chroma-js"

interface License {
  type: string,
  url: string,
}

interface Meta {
  name: string,
  author: string,
  url: string,
  license: License
}

interface Colors {
  neutral: Scale<Color>
  red: Scale<Color>
  orange: Scale<Color>
  yellow: Scale<Color>
  green: Scale<Color>
  cyan: Scale<Color>
  blue: Scale<Color>
  violet: Scale<Color>
  magenta: Scale<Color>
}

export interface ThemeConfig {
  meta: Meta
  color: Colors,
  syntax: {
    primary?: string
    comment?: string
    punctuation?: string
    constant?: string
    keyword?: string
  }
}