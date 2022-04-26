import * as fs from "fs";
import * as path from "path";
import app from "./styleTree/app";
import { dark as caveDark, light as caveLight } from "./themes/cave";
import dark from "./themes/dark";
import light from "./themes/light";
import { dark as solarizedDark, light as solarizedLight } from "./themes/solarized";
import { dark as sulphurpoolDark, light as sulphurpoolLight } from "./themes/sulphurpool";
import snakeCase from "./utils/snakeCase";

const themes = [
  dark, light,
  caveDark, caveLight,
  solarizedDark, solarizedLight,
  sulphurpoolDark, sulphurpoolLight
];

for (let theme of themes) {
  let styleTree = snakeCase(app(theme));
  let styleTreeJSON = JSON.stringify(styleTree, null, 2);
  let outPath = path.resolve(
    `${__dirname}/../../assets/themes/${theme.name}.json`
  );
  fs.writeFileSync(outPath, styleTreeJSON);
  console.log(`- ${outPath} created`);
}
