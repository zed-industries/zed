import chroma from "chroma-js";
import { generateColorSet } from "../algorithm";
import { ColorFamily } from "../types";

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

// Yellow ======================================== //

const yellow: ColorFamily = generateColorSet({
  name: "yellow",
  color: {
    start: "#FEFCE8",
    middle: "#FADB15",
    end: "#715E12",
  },
});

export const yellowLight = chroma.scale(yellow.colors).mode("lch");
export const yellowDark = chroma.scale(yellow.invertedColors).mode("lch");

// Lime ======================================== //

const lime: ColorFamily = generateColorSet({
  name: "lime",
  color: {
    start: "#F7FEE7",
    middle: "#32CD32",
    end: "#295214",
  },
});

export const limeLight = chroma.scale(lime.colors).mode("lch");
export const limeDark = chroma.scale(lime.invertedColors).mode("lch");

// Green ======================================== //

const green: ColorFamily = generateColorSet({
  name: "green",
  color: {
    start: "#F1FDF0",
    middle: "#43c84c",
    end: "#155117",
  },
});

export const greenLight = chroma.scale(green.colors).mode("lch");
export const greenDark = chroma.scale(green.invertedColors).mode("lch");

// Emerald ======================================== //

const emerald: ColorFamily = generateColorSet({
  name: "emerald",
  color: {
    start: "#F0FDF4",
    middle: "#51C878",
    end: "#134E29",
  },
});

export const emeraldLight = chroma.scale(emerald.colors).mode("lch");
export const emeraldDark = chroma.scale(emerald.invertedColors).mode("lch");

// Jade ======================================== //

const jade: ColorFamily = generateColorSet({
  name: "jade",
  color: {
    start: "#ECFDF5",
    middle: "#1ABD82",
    end: "#064E3B",
  },
});

export const jadeLight = chroma.scale(jade.colors).mode("lch");
export const jadeDark = chroma.scale(jade.invertedColors).mode("lch");

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

// Cyan ======================================== //

const cyan = generateColorSet({
  name: "cyan",
  color: {
    start: "#E0F7FA",
    middle: "#00BCD4",
    end: "#006064",
  },
});

export const cyanLight = chroma.scale(cyan.colors).mode("lch");
export const cyanDark = chroma.scale(cyan.invertedColors).mode("lch");

// Light Blue ======================================== //

const lightBlue = generateColorSet({
  name: "lightBlue",
  color: {
    start: "#E1F5FE",
    middle: "#03A9F4",
    end: "#01579B",
  },
});

export const lightBlueLight = chroma.scale(lightBlue.colors).mode("lch");
export const lightBlueDark = chroma.scale(lightBlue.invertedColors).mode("lch");

// Blue ======================================== //

const blue = generateColorSet({
  name: "blue",
  color: {
    start: "#E3F2FD",
    middle: "#3B82F6",
    end: "#0D47A1",
  },
});

export const blueLight = chroma.scale(blue.colors).mode("lch");
export const blueDark = chroma.scale(blue.colors).mode("lch");

// Indigo ======================================== //

const indigo = generateColorSet({
  name: "indigo",
  color: {
    start: "#e8eaf7",
    middle: "#586cc6",
    end: "#182383",
  },
});

export const indigoLight = chroma.scale(indigo.colors).mode("lch");
export const indigoDark = chroma.scale(indigo.colors).mode("lch");

// Violet ======================================== //

const violet = generateColorSet({
  name: "violet",
  color: {
    start: "#f6e4f6",
    middle: "#b93ec2",
    end: "#490d85",
  },
});

export const violetLight = chroma.scale(violet.colors).mode("lch");
export const violetDark = chroma.scale(violet.colors).mode("lch");

// Pink ======================================== //

const pink = generateColorSet({
  name: "pink",
  color: {
    start: "#ffe3ec",
    middle: "#ff257a",
    end: "#950050",
  },
});

export const pinkLight = chroma.scale(pink.colors).mode("lch");
export const pinkDark = chroma.scale(pink.colors).mode("lch");

// Brown ======================================== //

const brown = generateColorSet({
  name: "brown",
  color: {
    start: "#f0ebe9",
    middle: "#936c61",
    end: "#422622",
  },
});

export const brownLight = chroma.scale(brown.colors).mode("lch");
export const brownDark = chroma.scale(brown.colors).mode("lch");
