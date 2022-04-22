import * as fs from "fs";
import * as path from "path";
import dark from "./themes/dark";
import light from "./themes/light";
import solarizedDark from "./themes/solarized-dark";
import solarizedLight from "./themes/solarized-light";
import Theme from "./themes/theme";
import { colors, fontFamilies, fontSizes, fontWeights } from "./tokens";

// Organize theme tokens
function themeTokens(theme: Theme) {
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

const combinedTokens: any = {};

const distPath = path.resolve(`${__dirname}/../dist`);

// Add core tokens to the combined tokens and write `core.json`.
// We write `core.json` as a separate file for the design team's convenience, but it isn't consumed by Figma Tokens directly.
const corePath = path.join(distPath, "core.json");
fs.writeFileSync(corePath, JSON.stringify(coreTokens, null, 2));
console.log(`- ${corePath} created`);
combinedTokens.core = coreTokens;

// Add each theme to the combined tokens and write ${theme}.json.
// We write `${theme}.json` as a separate file for the design team's convenience, but it isn't consumed by Figma Tokens directly.
let themes = [dark, light, solarizedDark, solarizedLight];
themes.forEach((theme) => {
  const themePath = `${distPath}/${theme.name}.json`
  fs.writeFileSync(themePath, JSON.stringify(themeTokens(theme), null, 2));
  console.log(`- ${themePath} created`);
  combinedTokens[theme.name] = themeTokens(theme);
});

// Write combined tokens to `tokens.json`. This file is consumed by the Figma Tokens plugin to keep our designs consistent with the app.
const combinedPath = path.resolve(`${distPath}/tokens.json`);
fs.writeFileSync(combinedPath, JSON.stringify(combinedTokens, null, 2));
console.log(`- ${combinedPath} created`);
