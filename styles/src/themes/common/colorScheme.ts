import { Scale } from "chroma-js";

export interface ColorScheme {
  name: string,
  isLight: boolean,

  lowest: Elevation,
  middle: Elevation,
  highest: Elevation,

  players: Players,
}

export interface Player {
  cursor: string,
  selection: string,
}

export interface Players {
  "0": Player,
  "1": Player,
  "2": Player,
  "3": Player,
  "4": Player,
  "5": Player,
  "6": Player,
  "7": Player,
}

export interface Elevation {
  ramps: RampSet,

  bottom: Layer,
  middle: Layer,
  top: Layer,

  above?: Elevation,
  shadow?: Shadow
}

export interface Shadow {
  blur: number,
  color: string,
  offset: number[],
}

export type StyleSets = keyof Layer;
export interface Layer {
  base: StyleSet,
  variant: StyleSet,
  on: StyleSet,
  info: StyleSet,
  positive: StyleSet,
  warning: StyleSet,
  negative: StyleSet,
}

export interface RampSet {
  neutral: Scale,
  red: Scale,
  orange: Scale,
  yellow: Scale,
  green: Scale,
  cyan: Scale,
  blue: Scale,
  violet: Scale,
  magenta: Scale,
}

export type Styles = keyof StyleSet;
export interface StyleSet {
  default: Style,
  active: Style,
  disabled: Style,
  hovered: Style,
  pressed: Style,
}

export interface Style {
  background: string,
  border: string,
  foreground: string,
}