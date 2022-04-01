import core from "../tokens/core";
import Theme, { NumberToken, Syntax } from "./theme";

const { color } = core;

// TODO: Replace with light values

const backgroundColor = {
    100: {
        base: color.neutral[750],
        hovered: color.neutral[750],
        active: color.neutral[750],
        focused: color.neutral[750],
    },
    300: {
        base: color.neutral[800],
        hovered: color.neutral[800],
        active: color.neutral[800],
        focused: color.neutral[800],
    },
    500: {
        base: color.neutral[900],
        hovered: color.neutral[900],
        active: color.neutral[900],
        focused: color.neutral[900],
    },
    ok: {
        base: color.green[600],
        hovered: color.green[600],
        active: color.green[600],
        focused: color.green[600],
    },
    error: {
        base: color.red[400],
        hovered: color.red[400],
        active: color.red[400],
        focused: color.red[400],
    },
    warning: {
        base: color.amber[300],
        hovered: color.amber[300],
        active: color.amber[300],
        focused: color.amber[300],
    },
    info: {
        base: color.blue[500],
        hovered: color.blue[500],
        active: color.blue[500],
        focused: color.blue[500],
    },
};

const borderColor = {
    primary: color.neutral[850],
    secondary: color.neutral[700],
    muted: color.neutral[750],
    focused: color.neutral[100],
    active: color.neutral[500],
    ok: color.neutral[999],
    error: color.neutral[999],
    warning: color.neutral[999],
    info: color.neutral[999],
};

const textColor = {
    primary: color.neutral[150],
    secondary: color.neutral[350],
    muted: color.neutral[550],
    placeholder: color.neutral[750],
    active: color.neutral[0],
    //TODO: (design) define feature and it's correct value
    feature: color.sky[500],
    ok: color.green[600],
    error: color.red[400],
    warning: color.amber[300],
    info: color.blue[500],
};

const iconColor = {
    primary: color.neutral[300],
    secondary: color.neutral[500],
    muted: color.neutral[600],
    placeholder: color.neutral[700],
    active: color.neutral[50],
    //TODO: (design) define feature and it's correct value
    feature: color.sky[500],
    ok: color.green[600],
    error: color.red[400],
    warning: color.amber[300],
    info: color.blue[500],
};

const player = {
    1: {
        baseColor: color.blue[600],
        cursorColor: color.blue[600],
        selectionColor: color.blue[600],
        borderColor: color.blue[600],
    },
    2: {
        baseColor: color.indigo[500],
        cursorColor: color.indigo[500],
        selectionColor: color.indigo[500],
        borderColor: color.indigo[500],
    },
    3: {
        baseColor: color.green[500],
        cursorColor: color.green[500],
        selectionColor: color.green[500],
        borderColor: color.green[500],
    },
    4: {
        baseColor: color.orange[500],
        cursorColor: color.orange[500],
        selectionColor: color.orange[500],
        borderColor: color.orange[500],
    },
    5: {
        baseColor: color.purple[500],
        cursorColor: color.purple[500],
        selectionColor: color.purple[500],
        borderColor: color.purple[500],
    },
    6: {
        baseColor: color.teal[400],
        cursorColor: color.teal[400],
        selectionColor: color.teal[400],
        borderColor: color.teal[400],
    },
    7: {
        baseColor: color.pink[400],
        cursorColor: color.pink[400],
        selectionColor: color.pink[400],
        borderColor: color.pink[400],
    },
    8: {
        baseColor: color.yellow[400],
        cursorColor: color.yellow[400],
        selectionColor: color.yellow[400],
        borderColor: color.yellow[400],
    },
};

// TODO: Fixup
const editor = {
    background: backgroundColor[500].base,
    indent_guide: borderColor.muted,
    indent_guide_active: borderColor.secondary,
    line: {
        active: color.neutral[0],
        highlighted: color.neutral[0],
        inserted: backgroundColor.ok.active,
        deleted: backgroundColor.error.active,
        modified: backgroundColor.info.active,
    },
    highlight: {
        selection: player[1].selectionColor,
        occurrence: backgroundColor[500].active,
        activeOccurrence: color.neutral[0],
        matchingBracket: color.neutral[0],
        match: color.neutral[0],
        activeMatch: color.neutral[0],
        related: color.neutral[0],
    },
    gutter: {
        primary: color.neutral[0],
        active: color.neutral[0],
    },
};

const syntax: Syntax = {
    primary: {
        color: textColor.primary,
        weight: { value: "normal", type: "fontWeight" },
    },
    comment: {
        color: color.lime[200],
        weight: { value: "normal", type: "fontWeight" },
    },
    punctuation: {
        color: textColor.primary,
        weight: { value: "normal", type: "fontWeight" },
    },
    constant: {
        color: color.neutral[150],
        weight: { value: "normal", type: "fontWeight" },
    },
    keyword: {
        color: color.sky[400],
        weight: { value: "normal", type: "fontWeight" },
    },
    function: {
        color: color.yellow[200],
        weight: { value: "normal", type: "fontWeight" },
    },
    type: {
        color: color.teal[300],
        weight: { value: "normal", type: "fontWeight" },
    },
    variant: {
        color: color.teal[300],
        weight: { value: "normal", type: "fontWeight" },
    },
    property: {
        color: color.sky[300],
        weight: { value: "normal", type: "fontWeight" },
    },
    enum: {
        color: color.sky[400],
        weight: { value: "normal", type: "fontWeight" },
    },
    operator: {
        color: color.sky[400],
        weight: { value: "normal", type: "fontWeight" },
    },
    string: {
        color: color.orange[300],
        weight: { value: "normal", type: "fontWeight" },
    },
    number: {
        color: color.neutral[150],
        weight: { value: "normal", type: "fontWeight" },
    },
    boolean: {
        color: color.neutral[150],
        weight: { value: "normal", type: "fontWeight" },
    },
    predictive: {
        color: textColor.muted,
        weight: { value: "normal", type: "fontWeight" },
    },
};

const shadowAlpha: NumberToken = {
    value: 0.32,
    type: "number"
};

const theme: Theme = {
    name: "light",
    backgroundColor,
    borderColor,
    textColor,
    iconColor,
    editor,
    syntax,
    player,
    shadowAlpha,
};

export default theme;
