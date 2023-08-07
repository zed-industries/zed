import { dark, light, mirage } from "ayu"
import {
    chroma,
    color_ramp,
    ThemeLicenseType,
    ThemeSyntax,
    ThemeFamilyMeta,
} from "../../common"

export const ayu = {
    dark,
    light,
    mirage,
}

export const build_theme = (t: typeof dark, light: boolean) => {
    const color = {
        light_blue: t.syntax.tag.hex(),
        yellow: t.syntax.func.hex(),
        blue: t.syntax.entity.hex(),
        green: t.syntax.string.hex(),
        teal: t.syntax.regexp.hex(),
        red: t.syntax.markup.hex(),
        orange: t.syntax.keyword.hex(),
        light_yellow: t.syntax.special.hex(),
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
            red: color_ramp(chroma(color.red)),
            orange: color_ramp(chroma(color.orange)),
            yellow: color_ramp(chroma(color.yellow)),
            green: color_ramp(chroma(color.green)),
            cyan: color_ramp(chroma(color.teal)),
            blue: color_ramp(chroma(color.blue)),
            violet: color_ramp(chroma(color.purple)),
            magenta: color_ramp(chroma(color.light_blue)),
        },
        syntax,
    }
}

export const build_syntax = (t: typeof dark): ThemeSyntax => {
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

export const meta: ThemeFamilyMeta = {
    name: "Ayu",
    author: "dempfi",
    license_type: ThemeLicenseType.MIT,
    license_url: "https://github.com/dempfi/ayu",
}
