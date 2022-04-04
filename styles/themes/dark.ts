import { colors, fontWeights, NumberToken } from "../tokens";
import Theme, { Syntax } from "./theme";

const backgroundColor = {
    100: {
        base: colors.neutral[750],
        hovered: colors.neutral[725],
        active: colors.neutral[700],
        focused: colors.neutral[675],
    },
    300: {
        base: colors.neutral[800],
        hovered: colors.neutral[775],
        active: colors.neutral[750],
        focused: colors.neutral[725],
    },
    500: {
        base: colors.neutral[900],
        hovered: colors.neutral[875],
        active: colors.neutral[850],
        focused: colors.neutral[825],
    },
    ok: {
        base: colors.green[600],
        hovered: colors.green[600],
        active: colors.green[600],
        focused: colors.green[600],
    },
    error: {
        base: colors.red[400],
        hovered: colors.red[400],
        active: colors.red[400],
        focused: colors.red[400],
    },
    warning: {
        base: colors.amber[300],
        hovered: colors.amber[300],
        active: colors.amber[300],
        focused: colors.amber[300],
    },
    info: {
        base: colors.blue[500],
        hovered: colors.blue[500],
        active: colors.blue[500],
        focused: colors.blue[500],
    },
};

const borderColor = {
    primary: colors.neutral[875],
    secondary: colors.neutral[825],
    muted: colors.neutral[775],
    focused: colors.neutral[500],
    active: colors.neutral[900],
    ok: colors.green[500],
    error: colors.red[500],
    warning: colors.amber[500],
    info: colors.blue[500],
};

const textColor = {
    primary: colors.neutral[50],
    secondary: colors.neutral[350],
    muted: colors.neutral[550],
    placeholder: colors.neutral[750],
    active: colors.neutral[0],
    //TODO: (design) define feature and it's correct value
    feature: colors.sky[500],
    ok: colors.green[600],
    error: colors.red[400],
    warning: colors.amber[300],
    info: colors.blue[500],
};

const iconColor = {
    primary: colors.neutral[200],
    secondary: colors.neutral[350],
    muted: colors.neutral[600],
    placeholder: colors.neutral[700],
    active: colors.neutral[0],
    //TODO: (design) define feature and it's correct value
    feature: colors.sky[500],
    ok: colors.green[600],
    error: colors.red[500],
    warning: colors.amber[400],
    info: colors.blue[600],
};

const player = {
    1: {
        baseColor: colors.blue[500],
        cursorColor: colors.blue[500],
        selectionColor: colors.blue[800],
        borderColor: colors.blue[800],
    },
    2: {
        baseColor: colors.lime[500],
        cursorColor: colors.lime[500],
        selectionColor: colors.lime[800],
        borderColor: colors.lime[500],
    },
    3: {
        baseColor: colors.indigo[500],
        cursorColor: colors.indigo[500],
        selectionColor: colors.indigo[800],
        borderColor: colors.indigo[500],
    },
    4: {
        baseColor: colors.orange[500],
        cursorColor: colors.orange[500],
        selectionColor: colors.orange[800],
        borderColor: colors.orange[500],
    },
    5: {
        baseColor: colors.purple[500],
        cursorColor: colors.purple[500],
        selectionColor: colors.purple[800],
        borderColor: colors.purple[500],
    },
    6: {
        baseColor: colors.teal[400],
        cursorColor: colors.teal[400],
        selectionColor: colors.teal[800],
        borderColor: colors.teal[400],
    },
    7: {
        baseColor: colors.pink[400],
        cursorColor: colors.pink[400],
        selectionColor: colors.pink[100],
        borderColor: colors.pink[400],
    },
    8: {
        baseColor: colors.yellow[400],
        cursorColor: colors.yellow[400],
        selectionColor: colors.yellow[100],
        borderColor: colors.yellow[400],
    },
};

// TODO: Fixup
const editor = {
    background: backgroundColor[500].base,
    indent_guide: borderColor.muted,
    indent_guide_active: borderColor.secondary,
    line: {
        active: backgroundColor[500].active,
        highlighted: backgroundColor[500].hovered,
        inserted: backgroundColor.ok.active,
        deleted: backgroundColor.error.active,
        modified: backgroundColor.info.active,
    },
    highlight: {
        selection: player[1].selectionColor,
        occurrence: colors.neutral[750], 
        activeOccurrence: colors.neutral[700],
        matchingBracket: backgroundColor[500].active,
        match: colors.sky[900],
        activeMatch: colors.sky[800],
        related: backgroundColor[500].focused,
    },
    gutter: {
        primary: textColor.muted,
        active: textColor.active,
    },
};

const syntax: Syntax = {
    primary: {
        color: colors.neutral[150],
        weight: fontWeights.normal,
    },
    comment: {
        color: colors.neutral[300],
        weight: fontWeights.normal,
    },
    punctuation: {
        color: colors.neutral[200],
        weight: fontWeights.normal,
    },
    constant: {
        color: colors.neutral[150],
        weight: fontWeights.normal,
    },
    keyword: {
        color: colors.blue[400],
        weight: fontWeights.normal,
    },
    function: {
        color: colors.yellow[200],
        weight: fontWeights.normal,
    },
    type: {
        color: colors.teal[300],
        weight: fontWeights.normal,
    },
    variant: {
        color: colors.sky[300],
        weight: fontWeights.normal,
    },
    property: {
        color: colors.blue[400],
        weight: fontWeights.normal,
    },
    enum: {
        color: colors.orange[500],
        weight: fontWeights.normal,
    },
    operator: {
        color: colors.orange[500],
        weight: fontWeights.normal,
    },
    string: {
        color: colors.orange[300],
        weight: fontWeights.normal,
    },
    number: {
        color: colors.lime[300],
        weight: fontWeights.normal,
    },
    boolean: {
        color: colors.lime[300],
        weight: fontWeights.normal,
    },
    predictive: {
        color: textColor.muted,
        weight: fontWeights.normal,
    },
    title: {
        color: colors.amber[500],
        weight: fontWeights.bold,
    },
    emphasis: {
        color: textColor.active,
        weight: fontWeights.normal,
    },
    emphasisStrong: {
        color: textColor.active,
        weight: fontWeights.bold,
    },
    linkUrl: {
        color: colors.lime[500],
        weight: fontWeights.normal,
        // TODO: add underline
    },
    linkText: {
        color: colors.orange[500],
        weight: fontWeights.normal,
        // TODO: add italic
    },
    listMarker: {
        color: colors.sky[400],
        weight: fontWeights.normal,
    }
};

const shadowAlpha: NumberToken = {
    value: 0.32,
    type: "number",
};

const theme: Theme = {
    name: "dark",
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
