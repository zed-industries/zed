import { buttonWithIconStyle } from "@/components";
import { buildTheme } from "@/theme/buildTheme";
import zedDark from "@/themes/zed/zedDark";

const theme = buildTheme(zedDark);

const iconButton = buttonWithIconStyle(theme);

console.log(JSON.stringify(iconButton))
