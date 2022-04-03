import * as fs from "fs";
import * as path from "path";
import { default as dark } from "./themes/dark";
import light from "./themes/light";
import Theme from "./themes/theme";
import { colors, fontFamilies, fontSizes, fontWeights } from "./tokens";

const outPath = path.resolve(
  `${__dirname}/figma/tokens.json`
);

function coreTokens(): Object {
  return {
    color: colors,
    text: {
      family: fontFamilies,
      weight: fontWeights,
    },
    size: fontSizes,
  }
}

function themeTokens(theme: Theme): Object {
  return {
    text: {
      primary: {
        value: theme.textColor.primary.value,
        type: "color",
      }
    }
  }
}

let tokens = {
  core: coreTokens(),
  dark: themeTokens(dark),
  light: themeTokens(light),
};

const tokenJSON = JSON.stringify(tokens, null, 2);

fs.writeFileSync(outPath, tokenJSON);