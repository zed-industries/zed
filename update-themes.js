const fs = require("fs");
const path = require("path");

const THEMES_ROOT = "assets/themes";

for (const themeDir of fs.readdirSync(THEMES_ROOT)) {
  const stat = fs.statSync(path.join(THEMES_ROOT, themeDir));
  if (!stat.isDirectory()) {
    continue;
  }

  for (const themeFile of fs.readdirSync(path.join(THEMES_ROOT, themeDir))) {
    if (!themeFile.endsWith(".json")) {
      continue;
    }

    const themeFilepath = path.join(THEMES_ROOT, themeDir, themeFile);
    console.log(themeFilepath);

    const themeFamilyJson = fs.readFileSync(themeFilepath, "utf8");
    const themeFamily = JSON.parse(themeFamilyJson);

    for (const theme of themeFamily.themes) {
      theme.style["ignored"] = theme.style["text.disabled"];
    }

    fs.writeFileSync(
      themeFilepath,
      JSON.stringify(themeFamily, null, 2) + "\n",
      "utf8",
    );
  }
}
