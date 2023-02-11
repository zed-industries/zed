export type Color = {
  step: number;
  hex: string;
  lch: number[];
  rgbaArray: number[];
};

export type ColorSet = Color[];

export type ColorFamily = {
  name: string;
  colors: string[];
  invertedColors: string[];
  colorsMeta: ColorSet;
  invertedMeta: ColorSet;
};

export interface ColorProps {
  name: string;
  color: {
    start: string;
    middle: string;
    end: string;
  };
}
