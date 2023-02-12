import { Color as ChromaColor } from "chroma-js";

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
    start: string | ChromaColor;
    middle: string | ChromaColor;
    end: string | ChromaColor;
  };
}
