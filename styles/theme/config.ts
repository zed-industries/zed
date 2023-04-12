import { OptionalSyntaxStyles } from "@/theme/syntax";

interface Author {
    name: string;
    email: string;
    handle: string;
}

type License = "MIT" | "Apache-2.0" | "GPL-3.0" | "Unlicense";

export type InputColor = string | string[];

export interface ThemeConfig {
    name: string;
    appearance: "light" | "dark";
    author: string | Author;
    url?: string;
    license: License;
    colors: {
        neutral: InputColor;
        accent: InputColor;
        error: InputColor;
        info: InputColor;
        warning: InputColor;
        success: InputColor;
    };
    syntax?: OptionalSyntaxStyles;
}

export interface CalculatedThemeProperties {
    intensity: {
        min: number;
        max: number;
    };
}

export type Theme = ThemeConfig & CalculatedThemeProperties;
