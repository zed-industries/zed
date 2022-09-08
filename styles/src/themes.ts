import fs from "fs";
import path from "path";
import Theme from "./themes/common/theme";

const themes: Theme[] = [];
export default themes;

const internalThemes: Theme[] = [];
export { internalThemes }

function fillThemes(themesPath: string, themes: Theme[]) {
  for (const fileName of fs.readdirSync(themesPath)) {
    if (fileName == "template.ts") continue;
    const filePath = path.join(themesPath, fileName);

    if (fs.statSync(filePath).isFile()) {
      const theme = require(filePath);
      if (theme.dark) themes.push(theme.dark);
      if (theme.light) themes.push(theme.light);
    }
  }
}

fillThemes(path.resolve(`${__dirname}/themes`), themes)
fillThemes(path.resolve(`${__dirname}/themes/internal`), internalThemes)
