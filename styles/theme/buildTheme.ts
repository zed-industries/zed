import { Theme, ThemeConfig } from "@/theme/config";
import { buildThemeIntensity } from "@/theme/intensity";

export function buildTheme(themeConfig: ThemeConfig): Theme {
    const intensity = buildThemeIntensity(themeConfig);

    const theme: Theme = {
        ...themeConfig,
        intensity,
    }

    console.log(JSON.stringify(theme, null, 2))

    return theme;
}
