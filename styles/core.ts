import color from "./core.color";

export type Color = string;

export default {
  color: color,

  fontFamily: {
    sans: "Zed Sans",
    mono: "Zed Mono",
  },
  fontSize: {
    "3xs": {
      value: "8",
      type: "fontSizes",
    },
    "2xs": {
      value: "10",
      type: "fontSizes",
    },
    xs: {
      value: "12",
      type: "fontSizes",
    },
    sm: {
      value: "14",
      type: "fontSizes",
    },
    md: {
      value: "16",
      type: "fontSizes",
    },
    lg: {
      value: "18",
      type: "fontSizes",
    },
    xl: {
      value: "20",
      type: "fontSizes",
    },
  },
};
