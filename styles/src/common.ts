export const fontFamilies = {
  sans: "Zed Sans",
  mono: "Zed Mono",
}

export const fontSizes = {
  "3xs": 8,
  "2xs": 10,
  xs: 12,
  sm: 14,
  md: 16,
  lg: 18,
  xl: 20,
};

export type FontWeight = "thin"
  | "extra_light"
  | "light"
  | "normal"
  | "medium"
  | "semibold"
  | "bold"
  | "extra_bold"
  | "black";
export const fontWeights: { [key: string]: FontWeight } = {
  thin: "thin",
  extra_light: "extra_light",
  light: "light",
  normal: "normal",
  medium: "medium",
  semibold: "semibold",
  bold: "bold",
  extra_bold: "extra_bold",
  black: "black"
};

export const sizes = {
  px: 1,
  xs: 2,
  sm: 4,
  md: 6,
  lg: 8,
  xl: 12,
};

// export const colors = {
//   neutral: colorRamp(["white", "black"], { steps: 37, increment: 25 }), // (900/25) + 1
//   rose: colorRamp("#F43F5EFF"),
//   red: colorRamp("#EF4444FF"),
//   orange: colorRamp("#F97316FF"),
//   amber: colorRamp("#F59E0BFF"),
//   yellow: colorRamp("#EAB308FF"),
//   lime: colorRamp("#84CC16FF"),
//   green: colorRamp("#22C55EFF"),
//   emerald: colorRamp("#10B981FF"),
//   teal: colorRamp("#14B8A6FF"),
//   cyan: colorRamp("#06BBD4FF"),
//   sky: colorRamp("#0EA5E9FF"),
//   blue: colorRamp("#3B82F6FF"),
//   indigo: colorRamp("#6366F1FF"),
//   violet: colorRamp("#8B5CF6FF"),
//   purple: colorRamp("#A855F7FF"),
//   fuschia: colorRamp("#D946E4FF"),
//   pink: colorRamp("#EC4899FF"),
// }