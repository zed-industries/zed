import * as fs from "fs";
import * as path from "path";
import themes from "./themes";
import Theme from "./themes/common/theme";
import { colors, fontFamilies, fontSizes, fontWeights, sizes } from "./tokens";

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
      primary: theme.syntax.primary.color,
      comment: theme.syntax.comment.color,
      keyword: theme.syntax.keyword.color,
      function: theme.syntax.function.color,
      type: theme.syntax.type.color,
      variant: theme.syntax.variant.color,
      property: theme.syntax.property.color,
      enum: theme.syntax.enum.color,
      operator: theme.syntax.operator.color,
      string: theme.syntax.string.color,
      number: theme.syntax.number.color,
      boolean: theme.syntax.boolean.color,
    },
    player: theme.player,
    shadowAlpha: theme.shadowAlpha,
  };
}

// Organize core tokens
const coreTokens = {
  color: colors,
  text: {
    family: fontFamilies,
    weight: fontWeights,
  },
  size: sizes,
  fontSize: fontSizes,
};

const combinedTokens: any = {};

const distPath = path.resolve(`${__dirname}/../dist`);
for (const file of fs.readdirSync(distPath)) {
  fs.unlinkSync(path.join(distPath, file));
}

// Add core tokens to the combined tokens and write `core.json`.
// We write `core.json` as a separate file for the design team's convenience, but it isn't consumed by Figma Tokens directly.
const corePath = path.join(distPath, "core.json");
fs.writeFileSync(corePath, JSON.stringify(coreTokens, null, 2));
console.log(`- ${corePath} created`);
combinedTokens.core = coreTokens;

// Add each theme to the combined tokens and write ${theme}.json.
// We write `${theme}.json` as a separate file for the design team's convenience, but it isn't consumed by Figma Tokens directly.
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
