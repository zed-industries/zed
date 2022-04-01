import dark from "./themes/dark";
import light from "./themes/light";
import app from "./styleTree/app";

for (let theme of [dark, light]) {
    let styleTree = app(theme);

    let styleTreeJson = JSON.stringify(styleTree);
    console.log(styleTreeJson);
    // TODO: Write style tree json to zed crate assets folder
}