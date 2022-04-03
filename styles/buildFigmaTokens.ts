import * as fs from "fs";
import * as path from "path";
import dark from "./themes/dark";
import light from "./themes/light";
import Theme from "./themes/theme";
import { colors, fontFamilies, fontSizes, fontWeights } from "./tokens";

// Organize theme tokens
function themeTokens(name: String, theme: Theme): Object {
  return {
    meta: {
      themeName: name,
    },
    text: {
      primary: {
        value: theme.textColor.primary.value,
        type: "color",
      },
    },
  };
}

let themes = [
  themeTokens("dark", dark), 
  themeTokens("light", light), 
];

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
