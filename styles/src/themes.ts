import fs from "fs";
import path from "path";
import Theme from "./themes/common/theme";

const themes: Theme[] = [];
export default themes;

const themesPath = path.resolve(`${__dirname}/themes`);
for (const fileName of fs.readdirSync(themesPath)) {
  const filePath = path.join(themesPath, fileName);
  if (fs.statSync(filePath).isFile()) {
    const theme = require(filePath);
    themes.push(theme.dark);
    themes.push(theme.light);
  }
}
