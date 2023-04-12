import { dark, light, mirage } from "ayu"
import { ThemeSyntax } from "./syntax"
import chroma from "chroma-js"
import { colorRamp } from "./ramps"
import { Meta } from "./colorScheme"

export const ayu = {
    dark,
    light,
    mirage,
}

export const buildTheme = (t: typeof dark, light: boolean) => {
    const color = {
        lightBlue: t.syntax.tag.hex(),
        yellow: t.syntax.func.hex(),
        blue: t.syntax.entity.hex(),
        green: t.syntax.string.hex(),
        teal: t.syntax.regexp.hex(),
        red: t.syntax.markup.hex(),
        orange: t.syntax.keyword.hex(),
        lightYellow: t.syntax.special.hex(),
        gray: t.syntax.comment.hex(),
        purple: t.syntax.constant.hex(),
    }

    const syntax: ThemeSyntax = {
        constant: { color: t.syntax.constant.hex() },
        "string.regex": { color: t.syntax.regexp.hex() },
        string: { color: t.syntax.string.hex() },
        comment: { color: t.syntax.comment.hex() },
        keyword: { color: t.syntax.keyword.hex() },
        operator: { color: t.syntax.operator.hex() },
        number: { color: t.syntax.constant.hex() },
        type: { color: color.blue },
        boolean: { color: color.purple },
        "punctuation.special": { color: color.purple },
        "string.special": { color: t.syntax.special.hex() },
        function: { color: t.syntax.func.hex() },
    }

    return {
        ramps: {
            neutral: chroma.scale([
                light ? t.editor.fg.hex() : t.editor.bg.hex(),
                light ? t.editor.bg.hex() : t.editor.fg.hex(),
            ]),
            red: colorRamp(chroma(color.red)),
            orange: colorRamp(chroma(color.orange)),
            yellow: colorRamp(chroma(color.yellow)),
            green: colorRamp(chroma(color.green)),
            cyan: colorRamp(chroma(color.teal)),
            blue: colorRamp(chroma(color.blue)),
            violet: colorRamp(chroma(color.purple)),
            magenta: colorRamp(chroma(color.lightBlue)),
        },
        syntax,
    }
}

export const buildSyntax = (t: typeof dark): ThemeSyntax => {
    return {
        constant: { color: t.syntax.constant.hex() },
        "string.regex": { color: t.syntax.regexp.hex() },
        string: { color: t.syntax.string.hex() },
        comment: { color: t.syntax.comment.hex() },
        keyword: { color: t.syntax.keyword.hex() },
        operator: { color: t.syntax.operator.hex() },
        number: { color: t.syntax.constant.hex() },
        type: { color: t.syntax.regexp.hex() },
        "punctuation.special": { color: t.syntax.special.hex() },
        "string.special": { color: t.syntax.special.hex() },
        function: { color: t.syntax.func.hex() },
    }
}

export const meta: Meta = {
    name: "Ayu",
    author: "dempfi",
    license: {
        SPDX: "MIT",
        license_text: {
            https_url:
                "https://raw.githubusercontent.com/dempfi/ayu/master/LICENSE",
            license_checksum:
                "e0af0e0d1754c18ca075649d42f5c6d9a60f8bdc03c20dfd97105f2253a94173",
        },
    },
    url: "https://github.com/dempfi/ayu",
}
