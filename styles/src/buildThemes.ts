import * as fs from "fs";
import * as path from "path";
import app from "./styleTree/app";
import dark from "./themes/dark";
import light from "./themes/light";
import solarizedDark from "./themes/solarized-dark";
import solarizedLight from "./themes/solarized-light";
import sulphurpoolDark from "./themes/sulphurpool-dark";
import sulphurpoolLight from "./themes/sulphurpool-light";
import snakeCase from "./utils/snakeCase";

const themes = [
  dark, light,
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
