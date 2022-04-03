import * as fs from "fs";
import * as path from "path";
import dark from "./themes/dark";
import light from "./themes/light";
import Theme from "./themes/theme";
import { colors, fontFamilies, fontSizes, fontWeights } from "./tokens";

// Organize theme tokens
function themeTokens(theme: Theme): Object {
  return {
    meta: {
      themeName: theme.name,
    },
    text: theme.textColor,
    icon: theme.iconColor,
    background: theme.backgroundColor,
    border: theme.borderColor,
    editor: theme.editor,
    syntax: {
      primary: {
        value: theme.syntax.primary.color.value,
        type: "color",
      },
      comment: {
        value: theme.syntax.comment.color.value,
        type: "color",
      },
      keyword: {
        value: theme.syntax.keyword.color.value,
        type: "color",
      },
      function: {
        value: theme.syntax.function.color.value,
        type: "color",
      },
      type: {
        value: theme.syntax.type.color.value,
        type: "color",
      },
      variant: {
        value: theme.syntax.variant.color.value,
        type: "color",
      },
      property: {
        value: theme.syntax.property.color.value,
        type: "color",
      },
      enum: {
        value: theme.syntax.enum.color.value,
        type: "color",
      },
      operator: {
        value: theme.syntax.operator.color.value,
        type: "color",
      },
      string: {
        value: theme.syntax.string.color.value,
        type: "color",
      },
      number: {
        value: theme.syntax.number.color.value,
        type: "color",
      },
      boolean: {
        value: theme.syntax.boolean.color.value,
        type: "color",
      },
    },
    player: theme.player,
    shadowAlpha: theme.shadowAlpha,
  };
}

let themes = [themeTokens(dark), themeTokens(light)];

// Create {theme}.json
const themePath = path.resolve(`${__dirname}/figma`);
themes.forEach((theme) => {
  const tokenJSON = JSON.stringify(theme, null, 2);
  //@ts-ignore //TODO: IDK what the hell TS wants me to do here
  fs.writeFileSync(`${themePath}/${theme.meta.themeName}.json`, tokenJSON);
});

// Organize core tokens
const coreTokens = {
  color: {
    ...colors,
  },
  text: {
    family: fontFamilies,
    weight: fontWeights,
  },
  size: fontSizes,
};

// Create core.json
const corePath = path.resolve(`${__dirname}/figma/core.json`);
const coreTokenJSON = JSON.stringify(coreTokens, null, 2);
fs.writeFileSync(corePath, coreTokenJSON);
