import * as fs from "fs";
import * as path from "path";
import { tmpdir } from "os";
import app from "./styleTree/app";
import colorSchemes, { internalColorSchemes, experimentalColorSchemes } from "./colorSchemes";
import snakeCase from "./utils/snakeCase";
import { ColorScheme } from "./themes/common/colorScheme";

const themeDirectory = `${__dirname}/../../assets/themes`;
const internalDirectory = `${themeDirectory}/internal`;
const experimentsDirectory = `${themeDirectory}/experiments`;
const tempDirectory = fs.mkdtempSync(path.join(tmpdir(), "build-themes"));

// Clear existing themes
function clearThemes(themeDirectory: string) {
  for (const file of fs.readdirSync(themeDirectory)) {
    if (file.endsWith(".json")) {
      const name = file.replace(/\.json$/, "");
      if (!colorSchemes.find((colorScheme) => colorScheme.name === name)) {
        fs.unlinkSync(path.join(themeDirectory, file));
      }
    }
  }
}

clearThemes(themeDirectory);
clearThemes(internalDirectory);
clearThemes(experimentsDirectory);

function writeThemes(colorSchemes: ColorScheme[], outputDirectory: string) {
  for (let colorScheme of colorSchemes) {
    let styleTree = snakeCase(app(colorScheme));
    let styleTreeJSON = JSON.stringify(styleTree, null, 2);
    let tempPath = path.join(tempDirectory, `${colorScheme.name}.json`);
    let outPath = path.join(outputDirectory, `${colorScheme.name}.json`);
    fs.writeFileSync(tempPath, styleTreeJSON);
    fs.renameSync(tempPath, outPath);
    console.log(`- ${outPath} created`);
  }
}

// Write new themes to theme directory
writeThemes(colorSchemes, themeDirectory);
writeThemes(internalColorSchemes, internalDirectory);
writeThemes(experimentalColorSchemes, experimentsDirectory);
