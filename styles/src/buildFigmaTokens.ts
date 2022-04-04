import * as fs from "fs";
import * as path from "path";
import dark from "./themes/dark";
import light from "./themes/light";
import Theme from "./themes/theme";
import { colors, fontFamilies, fontSizes, fontWeights } from "./tokens";

// Organize theme tokens
function themeTokens(theme: Theme) {
    return {
        meta: {
            themeName: theme.name,
        },
        text: theme.textColor,
        icon: theme.iconColor,
        background: theme.backgroundColor,
        border: theme.borderColor,
        editor: theme.editor,
        syntax: {
            primary: {
                value: theme.syntax.primary.color.value,
                type: "color",
            },
            comment: {
                value: theme.syntax.comment.color.value,
                type: "color",
            },
            keyword: {
                value: theme.syntax.keyword.color.value,
                type: "color",
            },
            function: {
                value: theme.syntax.function.color.value,
                type: "color",
            },
            type: {
                value: theme.syntax.type.color.value,
                type: "color",
            },
            variant: {
                value: theme.syntax.variant.color.value,
                type: "color",
            },
            property: {
                value: theme.syntax.property.color.value,
                type: "color",
            },
            enum: {
                value: theme.syntax.enum.color.value,
                type: "color",
            },
            operator: {
                value: theme.syntax.operator.color.value,
                type: "color",
            },
            string: {
                value: theme.syntax.string.color.value,
                type: "color",
            },
            number: {
                value: theme.syntax.number.color.value,
                type: "color",
            },
            boolean: {
                value: theme.syntax.boolean.color.value,
                type: "color",
            },
        },
        player: theme.player,
        shadowAlpha: theme.shadowAlpha,
    };
}

// Organize core tokens
const coreTokens = {
    color: {
        ...colors,
    },
    text: {
        family: fontFamilies,
        weight: fontWeights,
    },
    size: fontSizes,
};

const combinedTokens: any = {
    core: coreTokens,
}

// Create core.json
const corePath = path.resolve(`${__dirname}/../dist/figma/core.json`);
const coreJSON = JSON.stringify(coreTokens, null, 2);
fs.writeFileSync(corePath, coreJSON);
console.log(`- Core: core.json created`);

// Create {theme}.json
let themes = [dark, light];
const themePath = path.resolve(`${__dirname}/figma`);
themes.forEach((theme) => {
    const tokenJSON = JSON.stringify(themeTokens(theme), null, 2);
    fs.writeFileSync(`${themePath}/${theme.name}.json`, tokenJSON);
    console.log(`- Theme: ${theme.name}.json created`);
    combinedTokens[theme.name] = themeTokens(theme);
});

// Create combined tokens.json
const combinedPath = path.resolve(`${__dirname}/figma/tokens.json`);
const combinedJSON = JSON.stringify(combinedTokens, null, 2);
fs.writeFileSync(combinedPath, combinedJSON);
console.log(`- Combined: tokens.json created`);