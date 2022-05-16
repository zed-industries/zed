import * as fs from "fs";
import * as path from "path";
import app from "./styleTree/app";
import { dark as caveDark, light as caveLight } from "./themes/cave";
import { dark as solarizedDark, light as solarizedLight } from "./themes/solarized";
import { dark as sulphurpoolDark, light as sulphurpoolLight } from "./themes/sulphurpool";
import snakeCase from "./utils/snakeCase";

const themes = [
  caveDark, caveLight,
  solarizedDark, solarizedLight,
  sulphurpoolDark, sulphurpoolLight
];

const themeDirectory = `${__dirname}/../../assets/themes/`;

// Clear existing themes
for (const file of fs.readdirSync(themeDirectory)) {
  fs.unlinkSync(path.join(themeDirectory, file));
}

// Write new themes to theme directory
for (let theme of themes) {
  let styleTree = snakeCase(app(theme));
  let styleTreeJSON = JSON.stringify(styleTree, null, 2);
  let outPath = path.resolve(
    `${__dirname}/../../assets/themes/${theme.name}.json`
  );
  fs.writeFileSync(outPath, styleTreeJSON);
  console.log(`- ${outPath} created`);
}
