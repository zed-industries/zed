import * as fs from "fs";
import * as path from "path";
import { default as dark } from "./themes/dark";
import light from "./themes/light";
import Theme from "./themes/theme";
import { colors, fontFamilies, fontSizes, fontWeights } from "./tokens";

const corePath = path.resolve(`${__dirname}/figma/core.json`);
const themePath = path.resolve(`${__dirname}/figma`);

function coreTokens(): Object {
  return {
    color: {
      ...colors,
    },
    text: {
      family: fontFamilies,
      weight: fontWeights,
    },
    size: fontSizes,
  };
}

function themeTokens(theme: Theme): Object {
  return {
    text: {
      primary: {
        value: theme.textColor.primary.value,
        type: "color",
      },
    },
  };
}

let themes = [
  { dark: themeTokens(dark) },
  { light: themeTokens(light) },
];

themes.forEach((theme) => {
  const name = Object.getOwnPropertyNames(theme);
  const tokenJSON = JSON.stringify(theme, null, 2);
  fs.writeFileSync(`${themePath}/${name}.json`, tokenJSON);
});


// Create core.json
const coreTokenJSON = JSON.stringify(coreTokens(), null, 2);
fs.writeFileSync(corePath, coreTokenJSON);

// const tokenJSON = JSON.stringify(tokens, null, 2);
// fs.writeFileSync(outPath, tokenJSON);
