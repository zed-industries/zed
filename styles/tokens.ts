import { colorRamp } from "./utils/color";

interface Token<V, T> {
  value: V,
  type: T
}

export type FontFamily = string;
export type FontFamilyToken = Token<FontFamily, "fontFamily">;
function fontFamily(value: FontFamily): FontFamilyToken {
  return {
    value,
    type: "fontFamily"
  }
}
export const fontFamilies = {
  sans: fontFamily("Zed Sans"),
  mono: fontFamily("Zed Mono"),
}

export type FontSize = number;
export type FontSizeToken = Token<FontSize, "fontSize">;
function fontSize(value: FontSize) {
  return {
    value,
    type: "fontSize"
  };
}
export const fontSizes = {
  "3xs": fontSize(8),
  "2xs": fontSize(10),
  xs: fontSize(12),
  sm: fontSize(14),
  md: fontSize(16),
  lg: fontSize(18),
  xl: fontSize(20),
};

export type FontWeight = 
  | "thin"
  | "extra_light"
  | "light"
  | "normal"
  | "medium"
  | "semibold"
  | "bold"
  | "extra_bold"
  | "black";
export type FontWeightToken = Token<FontWeight, "fontWeight">;
function fontWeight(value: FontWeight): FontWeightToken {
  return {
    value,
    type: "fontWeight"
  };
}
export const fontWeights = {
  "thin": fontWeight("thin"),
  "extra_light": fontWeight("extra_light"),
  "light": fontWeight("light"),
  "normal": fontWeight("normal"),
  "medium": fontWeight("medium"),
  "semibold": fontWeight("semibold"),
  "bold": fontWeight("bold"),
  "extra_bold": fontWeight("extra_bold"),
  "black": fontWeight("black"),
}

export type Color = string;
export interface ColorToken {
  value: Color,
  type: "color",
  step?: number,
}
export const colors = {
  neutral: colorRamp(["white", "black"], { steps: 37, increment: 25 }), // (900/25) + 1
  rose: colorRamp("#F43F5EFF"),
  red: colorRamp("#EF4444FF"),
  orange: colorRamp("#F97316FF"),
  amber: colorRamp("#F59E0BFF"),
  yellow: colorRamp("#EAB308FF"),
  lime: colorRamp("#84CC16FF"),
  green: colorRamp("#22C55EFF"),
  emerald: colorRamp("#10B981FF"),
  teal: colorRamp("#14B8A6FF"),
  cyan: colorRamp("#06BBD4FF"),
  sky: colorRamp("#0EA5E9FF"),
  blue: colorRamp("#3B82F6FF"),
  indigo: colorRamp("#6366F1FF"),
  violet: colorRamp("#8B5CF6FF"),
  purple: colorRamp("#A855F7FF"),
  fuschia: colorRamp("#D946E4FF"),
  pink: colorRamp("#EC4899FF"),
}

export type NumberToken = Token<number, "number">;

export default {
  fontFamilies,
  fontSizes,
  fontWeights,
  colors,
};
