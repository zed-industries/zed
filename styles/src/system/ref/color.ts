import * as chroma from "chroma-js";
import { ColorFamily, generateColorSet } from "../algorithm";

// Colors should use the LCH color space.
// https://www.w3.org/TR/css-color-4/#lch-colors

export const black = chroma.lch(0, 0, 0);

export const white = chroma.lch(150, 0, 0);

// Gray ======================================== //

const gray: ColorFamily = generateColorSet({
  name: "gray",
  color: {
    start: "#F0F0F0",
    middle: "#787878",
    end: "#0F0F0F",
  },
});

export const grayLight = chroma.scale(gray.colors).mode("lch");
export const grayDark = chroma.scale(gray.invertedColors).mode("lch");

// Rose ======================================== //

const rose: ColorFamily = generateColorSet({
  name: "rose",
  color: {
    start: "#FFF1F2",
    middle: "#F43F5E",
    end: "#881337",
  },
});

export const roseLight = chroma.scale(rose.colors).mode("lch");
export const roseDark = chroma.scale(rose.invertedColors).mode("lch");

// Red ======================================== //

const red: ColorFamily = generateColorSet({
  name: "red",
  color: {
    start: "#FEF2F2",
    middle: "#EF4444",
    end: "#7F1D1D",
  },
});

export const redLight = chroma.scale(red.colors).mode("lch");
export const redDark = chroma.scale(red.invertedColors).mode("lch");

// Orange ======================================== //

const orange: ColorFamily = generateColorSet({
  name: "orange",
  color: {
    start: "#FFF7ED",
    middle: "#F97316",
    end: "#7C2D12",
  },
});

export const orangeLight = chroma.scale(orange.colors).mode("lch");
export const orangeDark = chroma.scale(orange.invertedColors).mode("lch");

// Amber ======================================== //

const amber: ColorFamily = generateColorSet({
  name: "amber",
  color: {
    start: "#FFFBEB",
    middle: "#F59E0B",
    end: "#78350F",
  },
});

export const amberLight = chroma.scale(amber.colors).mode("lch");
export const amberDark = chroma.scale(amber.invertedColors).mode("lch");

// TODO: Add the rest of the colors.
// Source: https://www.figma.com/file/YEZ9jsC1uc9o6hgbv4kfxq/Core-color-library?node-id=48%3A816&t=Ae6tY1cVb2fm5xaM-1

// Teal ======================================== //

const teal: ColorFamily = generateColorSet({
  name: "teal",
  color: {
    start: "#E6FFFA",
    middle: "#14B8A6",
    end: "#134E4A",
  },
});

export const tealLight = chroma.scale(teal.colors).mode("lch");
export const tealDark = chroma.scale(teal.invertedColors).mode("lch");

const cyan = generateColorSet({
  name: "cyan",
  color: {
    start: "#F0FDFA",
    middle: "#06BBD4",
    end: "#164E63",
  },
});

export const cyanLight = chroma.scale(cyan.colors).mode("lch");
export const cyanDark = chroma.scale(cyan.colors).mode("lch");

console.log(JSON.stringify(teal, null, 2));
