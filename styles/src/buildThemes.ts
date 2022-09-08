import * as fs from "fs";
import * as path from "path";
import { tmpdir } from "os";
import app from "./styleTree/app";
import themes, { internalThemes } from "./themes";
import snakeCase from "./utils/snakeCase";
import Theme from "./themes/common/theme";

const themeDirectory = `${__dirname}/../../assets/themes`;
const internalDirectory = `${themeDirectory}/internal`;
const tempDirectory = fs.mkdtempSync(path.join(tmpdir(), "build-themes"));

// Clear existing themes
function clearThemes(themeDirectory: string) {
  for (const file of fs.readdirSync(themeDirectory)) {
    if (file.endsWith(".json")) {
      const name = file.replace(/\.json$/, "");
      if (!themes.find((theme) => theme.name === name)) {
        fs.unlinkSync(path.join(themeDirectory, file));
      }
    }
  }
}

clearThemes(themeDirectory);
clearThemes(internalDirectory);

function writeThemes(themes: Theme[], outputDirectory: string) {
  for (let theme of themes) {
    let styleTree = snakeCase(app(theme));
    let styleTreeJSON = JSON.stringify(styleTree, null, 2);
    let tempPath = path.join(tempDirectory, `${theme.name}.json`);
    let outPath = path.join(outputDirectory, `${theme.name}.json`);
    fs.writeFileSync(tempPath, styleTreeJSON);
    fs.renameSync(tempPath, outPath);
    console.log(`- ${outPath} created`);
  }
}

// Write new themes to theme directory
writeThemes(themes, themeDirectory);
writeThemes(internalThemes, internalDirectory);
