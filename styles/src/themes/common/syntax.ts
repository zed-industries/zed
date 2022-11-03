import { fontWeights } from "../../common";
import { RampSet } from "./colorScheme";
import { Syntax } from "./theme";
import { ThemeConfig } from "./themeConfig";

export default function buildSyntax(ramps: RampSet, theme: ThemeConfig) {
  const syntax: Syntax = {
    primary: theme.syntax.primary
      ? theme.syntax.primary
      : {
        color: ramps.neutral(1).hex(),
        weight: fontWeights.normal,
      },
    comment: theme.syntax.comment
      ? theme.syntax.comment
      : {
        color: ramps.neutral(0.71).hex(),
        weight: fontWeights.normal,
      },
    punctuation: theme.syntax.punctuation
      ? theme.syntax.punctuation
      : {
        color: ramps.neutral(0.86).hex(),
        weight: fontWeights.normal,
      },
    constant: theme.syntax.constant
      ? theme.syntax.constant
      : {
        color: ramps.green(0.5).hex(),
        weight: fontWeights.normal,
      },
    keyword: theme.syntax.keyword
      ? theme.syntax.keyword
      : {
        color: ramps.blue(0.5).hex(),
        weight: fontWeights.normal,
      },
    function: theme.syntax.function
      ? theme.syntax.function
      : {
        color: ramps.yellow(0.5).hex(),
        weight: fontWeights.normal,
      },
    type: theme.syntax.type
      ? theme.syntax.type
      : {
        color: ramps.cyan(0.5).hex(),
        weight: fontWeights.normal,
      },
    constructor:
    {
      color: ramps.blue(0.5).hex(),
      weight: fontWeights.normal,
    },
    variant: theme.syntax.variant
      ? theme.syntax.variant
      : {
        color: ramps.blue(0.5).hex(),
        weight: fontWeights.normal,
      },
    property: theme.syntax.property
      ? theme.syntax.property
      : {
        color: ramps.blue(0.5).hex(),
        weight: fontWeights.normal,
      },
    enum: theme.syntax.enum
      ? theme.syntax.enum
      : {
        color: ramps.orange(0.5).hex(),
        weight: fontWeights.normal,
      },
    operator: theme.syntax.operator
      ? theme.syntax.operator
      : {
        color: ramps.orange(0.5).hex(),
        weight: fontWeights.normal,
      },
    string: theme.syntax.string
      ? theme.syntax.string
      : {
        color: ramps.orange(0.5).hex(),
        weight: fontWeights.normal,
      },
    number: theme.syntax.number
      ? theme.syntax.number
      : {
        color: ramps.green(0.5).hex(),
        weight: fontWeights.normal,
      },
    boolean: theme.syntax.boolean
      ? theme.syntax.boolean
      : {
        color: ramps.green(0.5).hex(),
        weight: fontWeights.normal,
      },
    predictive: theme.syntax.predictive
      ? theme.syntax.predictive
      : {
        color: ramps.neutral(0.57).hex(),
        weight: fontWeights.normal,
      },
    title: theme.syntax.title
      ? theme.syntax.title
      : {
        color: ramps.yellow(0.5).hex(),
        weight: fontWeights.bold,
      },
    emphasis: theme.syntax.emphasis
      ? theme.syntax.emphasis
      : {
        color: ramps.blue(0.5).hex(),
        weight: fontWeights.normal,
      },
    "emphasis.strong": {
      color: ramps.blue(0.5).hex(),
      weight: fontWeights.bold,
    },
    linkUri: theme.syntax.linkUri
      ? theme.syntax.linkUri
      : {
        color: ramps.green(0.5).hex(),
        weight: fontWeights.normal,
        underline: true,
      },
    linkText: theme.syntax.linkText
      ? theme.syntax.linkText
      : {
        color: ramps.orange(0.5).hex(),
        weight: fontWeights.normal,
        italic: true,
      },
    method: theme.syntax.method
      ? theme.syntax.method
      : {
        color: ramps.blue(0.5).hex(),
        weight: fontWeights.normal,
      },
  };

  return syntax;
}
