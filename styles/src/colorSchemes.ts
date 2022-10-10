import fs from "fs";
import path from "path";
import { ColorScheme } from "./themes/common/colorScheme";

const colorSchemes: ColorScheme[] = [];
export default colorSchemes;

const internalColorSchemes: ColorScheme[] = [];
export { internalColorSchemes };

const experimentalColorSchemes: ColorScheme[] = [];
export { experimentalColorSchemes };

function fillColorSchemes(themesPath: string, colorSchemes: ColorScheme[]) {
  for (const fileName of fs.readdirSync(themesPath)) {
    if (fileName == "template.ts") continue;
    const filePath = path.join(themesPath, fileName);

    if (fs.statSync(filePath).isFile()) {
      const colorScheme = require(filePath);
      if (colorScheme.dark) colorSchemes.push(colorScheme.dark);
      if (colorScheme.light) colorSchemes.push(colorScheme.light);
    }
  }
}

fillColorSchemes(path.resolve(`${__dirname}/themes`), colorSchemes);
fillColorSchemes(
  path.resolve(`${__dirname}/themes/internal`),
  internalColorSchemes
);
fillColorSchemes(
  path.resolve(`${__dirname}/themes/experiments`),
  experimentalColorSchemes
);
