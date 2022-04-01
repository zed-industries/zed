import core from "./core";
import Theme from "./theme";

const backgroundColor = {
    100: {
        base: {
            value: core.color.neutral[999].value,
        },
        hovered: {
            value: core.color.neutral[999].value,
        },
        active: {
            value: core.color.neutral[999].value,
        },
        focused: {
            value: core.color.neutral[999].value,
        },
    },
    300: {
        base: {
            value: core.color.neutral[999].value,
        },
        hovered: {
            value: core.color.neutral[999].value,
        },
        active: {
            value: core.color.neutral[999].value,
        },
        focused: {
            value: core.color.neutral[999].value,
        },
    },
    500: {
        base: {
            value: core.color.neutral[999].value,
        },
        hovered: {
            value: "#000000",
        },
        active: {
            value: "#000000",
        },
        focused: {
            value: "#000000",
        },
    },
    ok: {
        base: {
            value: "#000000",
        },
        hovered: {
            value: "#000000",
        },
        active: {
            value: "#000000",
        },
        focused: {
            value: "#000000",
        },
    },
    error: {
        base: {
            value: "#000000",
        },
        hovered: {
            value: "#000000",
        },
        active: {
            value: "#000000",
        },
        focused: {
            value: "#000000",
        },
    },
    warning: {
        base: {
            value: "#000000",
        },
        hovered: {
            value: "#000000",
        },
        active: {
            value: "#000000",
        },
        focused: {
            value: "#000000",
        },
    },
    info: {
        base: {
            value: "#000000",
        },
        hovered: {
            value: "#000000",
        },
        active: {
            value: "#000000",
        },
        focused: {
            value: "#000000",
        },
    },
};

const borderColor = {
    primary: {
        value: "#000000",
    },
    secondary: {
        value: "#000000",
    },
    muted: {
        value: "#000000",
    },
    focused: {
        value: "#000000",
    },
    active: {
        value: "#000000",
    },
    ok: {
        value: "#000000",
    },
    error: {
        value: "#000000",
    },
    warning: {
        value: "#000000",
    },
    info: {
        value: "#000000",
    },
};

const textColor = {
    primary: {
        value: core.color.neutral[150].value,
    },
    secondary: {
        value: core.color.neutral[350].value,
    },
    muted: {
        value: core.color.neutral[550].value,
    },
    placeholder: {
        value: core.color.neutral[750].value,
    },
    active: {
        value: core.color.neutral[0].value,
    },
    feature: {
        //TODO: (design) define feature and it's correct value
        value: core.color.sky[500].value,
    },
    ok: {
        value: core.color.green[600].value,
    },
    error: {
        value: core.color.red[400].value,
    },
    warning: {
        value: core.color.amber[300].value,
    },
    info: {
        value: core.color.blue[500].value,
    },
};

const iconColor = {
    primary: {
        value: core.color.neutral[300].value,
    },
    secondary: {
        value: core.color.neutral[500].value,
    },
    muted: {
        value: core.color.neutral[600].value,
    },
    placeholder: {
        value: core.color.neutral[700].value,
    },
    active: {
        value: core.color.neutral[50].value,
    },
    feature: {
        //TODO: (design) define feature and it's correct value
        value: core.color.sky[500].value,
    },
    ok: {
        value: core.color.green[600].value,
    },
    error: {
        value: core.color.red[400].value,
    },
    warning: {
        value: core.color.amber[300].value,
    },
    info: {
        value: core.color.blue[500].value,
    },
};

const editor = {
    background: {
        value: backgroundColor[500].base.value,
    },
    indent_guide: {
        value: core.color.neutral[999].value,
    },
    indent_guide_active: {
        value: core.color.neutral[999].value,
    },
    line: {
        active: {
            value: core.color.neutral[999].value,
        },
        highlighted: {
            value: core.color.neutral[999].value,
        },
        inserted: {
            value: core.color.neutral[999].value,
        },
        deleted: {
            value: core.color.neutral[999].value,
        },
        modified: {
            value: core.color.neutral[999].value,
        },
    },
    highlight: {
        selection: {
            value: core.color.neutral[999].value,
        },
        occurrence: {
            value: core.color.neutral[999].value,
        },
        activeOccurrence: {
            value: core.color.neutral[999].value,
        },
        matchingBracket: {
            value: core.color.neutral[999].value,
        },
        match: {
            value: core.color.neutral[999].value,
        },
        activeMatch: {
            value: core.color.neutral[999].value,
        },
        related: {
            value: core.color.neutral[999].value,
        },
    },
    gutter: {
        primary: {
            value: core.color.neutral[999].value,
        },
        active: {
            value: core.color.neutral[999].value,
        },
    },
};

const syntax = {
    primary: {
        color: {
            value: core.color.neutral[150]
        },
        weight: { value: "normal" },
    },
    comment: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    punctuation: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    constant: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    keyword: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    function: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    type: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    variant: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    property: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    enum: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    operator: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    string: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    number: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    boolean: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
    predictive: {
        color: { value: "000000" },
        weight: { value: "normal" },
    },
};

const player = {
    1: {
        baseColor: {
            value: core.color.neutral[999].value,
        },
        cursorColor: {
            value: core.color.neutral[999].value,
        },
        selectionColor: {
            value: core.color.neutral[999].value,
        },
        borderColor: {
            value: core.color.neutral[999].value,
        },
    },
    2: {
        baseColor: {
            value: core.color.neutral[999].value,
        },
        cursorColor: {
            value: core.color.neutral[999].value,
        },
        selectionColor: {
            value: core.color.neutral[999].value,
        },
        borderColor: {
            value: core.color.neutral[999].value,
        },
    },
    3: {
        baseColor: {
            value: core.color.neutral[999].value,
        },
        cursorColor: {
            value: core.color.neutral[999].value,
        },
        selectionColor: {
            value: core.color.neutral[999].value,
        },
        borderColor: {
            value: core.color.neutral[999].value,
        },
    },
    4: {
        baseColor: {
            value: core.color.neutral[999].value,
        },
        cursorColor: {
            value: core.color.neutral[999].value,
        },
        selectionColor: {
            value: core.color.neutral[999].value,
        },
        borderColor: {
            value: core.color.neutral[999].value,
        },
    },
    5: {
        baseColor: {
            value: core.color.neutral[999].value,
        },
        cursorColor: {
            value: core.color.neutral[999].value,
        },
        selectionColor: {
            value: core.color.neutral[999].value,
        },
        borderColor: {
            value: core.color.neutral[999].value,
        },
    },
    6: {
        baseColor: {
            value: core.color.neutral[999].value,
        },
        cursorColor: {
            value: core.color.neutral[999].value,
        },
        selectionColor: {
            value: core.color.neutral[999].value,
        },
        borderColor: {
            value: core.color.neutral[999].value,
        },
    },
    7: {
        baseColor: {
            value: core.color.neutral[999].value,
        },
        cursorColor: {
            value: core.color.neutral[999].value,
        },
        selectionColor: {
            value: core.color.neutral[999].value,
        },
        borderColor: {
            value: core.color.neutral[999].value,
        },
    },
    8: {
        baseColor: {
            value: core.color.neutral[999].value,
        },
        cursorColor: {
            value: core.color.neutral[999].value,
        },
        selectionColor: {
            value: core.color.neutral[999].value,
        },
        borderColor: {
            value: core.color.neutral[999].value,
        },
    },
};

const shadowAlpha = {
    value: 0.32,
};

export default function dark(): Theme {
    return {
        backgroundColor,
        borderColor,
        textColor,
        iconColor,
        editor,
        syntax,
        player,
        shadowAlpha,
    };
}
