import * as fs from "fs";
import * as path from "path";
import { tmpdir } from 'os';
import app from "./styleTree/app";
import themes from "./themes";
import snakeCase from "./utils/snakeCase";

const themeDirectory = `${__dirname}/../../assets/themes/`;
const tempDirectory = fs.mkdtempSync(path.join(tmpdir(), 'build-themes'));

// Clear existing themes
for (const file of fs.readdirSync(themeDirectory)) {
  if (file.endsWith('.json')) {
    const name = file.replace(/\.json$/, '');
    if (!themes.find(theme => theme.name === name)) {
      fs.unlinkSync(path.join(themeDirectory, file));
    }
  }
}

// Write new themes to theme directory
for (let theme of themes) {
  let styleTree = snakeCase(app(theme));
  let styleTreeJSON = JSON.stringify(styleTree, null, 2);
  let tempPath = path.join(tempDirectory, `${theme.name}.json`);
  let outPath = path.join(themeDirectory, `${theme.name}.json`);
  fs.writeFileSync(tempPath, styleTreeJSON);
  fs.renameSync(tempPath, outPath);
  console.log(`- ${outPath} created`);
}
