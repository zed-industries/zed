import fs from "fs";
import path from "path";
import { Meta } from "./themes/common/themeConfig";
import { ColorScheme } from "./themes/common/colorScheme";

const colorSchemes: ColorScheme[] = [];
export default colorSchemes;

const schemeMeta: Meta[] = [];
export { schemeMeta };

const staffColorSchemes: ColorScheme[] = [];
export { staffColorSchemes };

const themes_directory = path.resolve(`${__dirname}/themes`);

function for_all_color_schemes_in(themesPath: string, callback: (module: any, path: string) => void) {
  for (const fileName of fs.readdirSync(themesPath)) {
    if (fileName == "template.ts") continue;
    const filePath = path.join(themesPath, fileName);

    if (fs.statSync(filePath).isFile()) {
      const colorScheme = require(filePath);
      callback(colorScheme, fileName);
    }
  }
}

function fillColorSchemes(themesPath: string, colorSchemes: ColorScheme[]) {
  for_all_color_schemes_in(themesPath, (colorScheme, _path) => {
    if (colorScheme.dark) colorSchemes.push(colorScheme.dark);
    if (colorScheme.light) colorSchemes.push(colorScheme.light);
    if (colorScheme.variants) {
      colorScheme.variants.map((variant: ColorScheme) => {
        colorSchemes.push(variant)
      })
    };

  })
}

fillColorSchemes(themes_directory, colorSchemes);
fillColorSchemes(
  path.resolve(`${themes_directory}/staff`),
  staffColorSchemes
);

function fillMeta(themesPath: string, meta: Meta[]) {
  for_all_color_schemes_in(themesPath, (colorScheme, name) => {
    if (colorScheme.meta) {
      meta.push(colorScheme.meta)
    } else {
      throw Error(`Public theme ${name} must have a meta field`)
    }
  })
}

fillMeta(themes_directory, schemeMeta);
