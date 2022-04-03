import * as fs from "fs";
import * as path from "path";
import app from "./styleTree/app";
import dark from "./themes/dark";
import light from "./themes/light";
import { colors } from "./tokens";
import decamelizeTree from "./utils/decamelizeTree";

const themes = [dark, light];
for (let theme of themes) {
    let styleTree = decamelizeTree(app(theme));
    let styleTreeJSON = JSON.stringify(styleTree, null, 2);
    let outPath = path.resolve(
        `${__dirname}/../crates/zed/assets/themes/${theme.name}.json`
    );
    fs.writeFileSync(outPath, styleTreeJSON);
    console.log(`Generated ${outPath}`);

    console.log(JSON.stringify(colors.indigo, null, 2));
}
