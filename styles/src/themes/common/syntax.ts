import { fontWeights } from "../../common";
import { RampSet } from "./colorScheme";
import { ThemeConfig } from "./themeConfig";

export default function buildSyntax(
  ramps: RampSet,
  theme: ThemeConfig
) {
  console.log(theme)

  let primary
  let comment
  let punctuation
  let constant
  let keyword

  if (theme?.syntax) {
    primary = theme.syntax.primary
    comment = theme.syntax.comment
    punctuation = theme.syntax.punctuation
    constant = theme.syntax.constant
    keyword = theme.syntax.keyword
  } else {
    primary = ramps.neutral(1).hex()
    comment = ramps.neutral(0.71).hex()
    punctuation = ramps.neutral(0.86).hex()
    constant = ramps.green(0.5).hex()
    keyword = ramps.blue(0.5).hex()
  }

  return {
    primary: {
      color: primary,
      weight: fontWeights.normal,
    },
    comment: {
      color: comment,
      weight: fontWeights.normal,
    },
    punctuation: {
      color: punctuation,
      weight: fontWeights.normal,
    },
    constant: {
      color: constant,
      weight: fontWeights.normal,
    },
    keyword: {
      color: keyword,
      weight: fontWeights.normal,
    },
    function: {
      color: ramps.yellow(0.5).hex(),
      weight: fontWeights.normal,
    },
    type: {
      color: ramps.cyan(0.5).hex(),
      weight: fontWeights.normal,
    },
    constructor: {
      color: ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    variant: {
      color: ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    property: {
      color: ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    enum: {
      color: ramps.orange(0.5).hex(),
      weight: fontWeights.normal,
    },
    operator: {
      color: ramps.orange(0.5).hex(),
      weight: fontWeights.normal,
    },
    string: {
      color: ramps.orange(0.5).hex(),
      weight: fontWeights.normal,
    },
    number: {
      color: ramps.green(0.5).hex(),
      weight: fontWeights.normal,
    },
    boolean: {
      color: ramps.green(0.5).hex(),
      weight: fontWeights.normal,
    },
    predictive: {
      color: ramps.neutral(0.57).hex(),
      weight: fontWeights.normal,
    },
    title: {
      color: ramps.yellow(0.5).hex(),
      weight: fontWeights.bold,
    },
    emphasis: {
      color: ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    "emphasis.strong": {
      color: ramps.blue(0.5).hex(),
      weight: fontWeights.bold,
    },
    linkUri: {
      color: ramps.green(0.5).hex(),
      weight: fontWeights.normal,
      underline: true,
    },
    linkText: {
      color: ramps.orange(0.5).hex(),
      weight: fontWeights.normal,
      italic: true,
    }
  }
};
