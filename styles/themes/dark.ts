import { colors, fontWeights, NumberToken } from "../tokens";
import Theme, { Syntax } from "./theme";

const backgroundColor = {
    100: {
        base: colors.neutral[750],
        hovered: colors.neutral[750],
        active: colors.neutral[750],
        focused: colors.neutral[750],
    },
    300: {
        base: colors.neutral[800],
        hovered: colors.neutral[800],
        active: colors.neutral[800],
        focused: colors.neutral[800],
    },
    500: {
        base: colors.neutral[900],
        hovered: colors.neutral[900],
        active: colors.neutral[900],
        focused: colors.neutral[900],
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
    primary: colors.neutral[850],
    secondary: colors.neutral[700],
    muted: colors.neutral[750],
    focused: colors.neutral[100],
    active: colors.neutral[500],
    ok: colors.neutral[1000],
    error: colors.neutral[1000],
    warning: colors.neutral[1000],
    info: colors.neutral[1000],
};

const textColor = {
    primary: colors.neutral[150],
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
    primary: colors.neutral[300],
    secondary: colors.neutral[500],
    muted: colors.neutral[600],
    placeholder: colors.neutral[700],
    active: colors.neutral[50],
    //TODO: (design) define feature and it's correct value
    feature: colors.sky[500],
    ok: colors.green[600],
    error: colors.red[400],
    warning: colors.amber[300],
    info: colors.blue[500],
};

const player = {
    1: {
        baseColor: colors.blue[600],
        cursorColor: colors.blue[600],
        selectionColor: colors.blue[600],
        borderColor: colors.blue[600],
    },
    2: {
        baseColor: colors.indigo[500],
        cursorColor: colors.indigo[500],
        selectionColor: colors.indigo[500],
        borderColor: colors.indigo[500],
    },
    3: {
        baseColor: colors.green[500],
        cursorColor: colors.green[500],
        selectionColor: colors.green[500],
        borderColor: colors.green[500],
    },
    4: {
        baseColor: colors.orange[500],
        cursorColor: colors.orange[500],
        selectionColor: colors.orange[500],
        borderColor: colors.orange[500],
    },
    5: {
        baseColor: colors.purple[500],
        cursorColor: colors.purple[500],
        selectionColor: colors.purple[500],
        borderColor: colors.purple[500],
    },
    6: {
        baseColor: colors.teal[400],
        cursorColor: colors.teal[400],
        selectionColor: colors.teal[400],
        borderColor: colors.teal[400],
    },
    7: {
        baseColor: colors.pink[400],
        cursorColor: colors.pink[400],
        selectionColor: colors.pink[400],
        borderColor: colors.pink[400],
    },
    8: {
        baseColor: colors.yellow[400],
        cursorColor: colors.yellow[400],
        selectionColor: colors.yellow[400],
        borderColor: colors.yellow[400],
    },
};

// TODO: Fixup
const editor = {
    background: backgroundColor[500].base,
    indent_guide: borderColor.muted,
    indent_guide_active: borderColor.secondary,
    line: {
        active: colors.neutral[0],
        highlighted: colors.neutral[0],
        inserted: backgroundColor.ok.active,
        deleted: backgroundColor.error.active,
        modified: backgroundColor.info.active,
    },
    highlight: {
        selection: player[1].selectionColor,
        occurrence: backgroundColor[500].active,
        activeOccurrence: colors.neutral[0],
        matchingBracket: colors.neutral[0],
        match: colors.neutral[0],
        activeMatch: colors.neutral[0],
        related: colors.neutral[0],
    },
    gutter: {
        primary: colors.neutral[0],
        active: colors.neutral[0],
    },
};

const syntax: Syntax = {
    primary: {
        color: textColor.primary,
        weight: fontWeights.normal,
    },
    comment: {
        color: colors.lime[200],
        weight: fontWeights.normal,
    },
    punctuation: {
        color: textColor.primary,
        weight: fontWeights.normal,
    },
    constant: {
        color: colors.neutral[150],
        weight: fontWeights.normal,
    },
    keyword: {
        color: colors.sky[400],
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
        color: colors.teal[300],
        weight: fontWeights.normal,
    },
    property: {
        color: colors.sky[300],
        weight: fontWeights.normal,
    },
    enum: {
        color: colors.sky[400],
        weight: fontWeights.normal,
    },
    operator: {
        color: colors.sky[400],
        weight: fontWeights.normal,
    },
    string: {
        color: colors.orange[300],
        weight: fontWeights.normal,
    },
    number: {
        color: colors.neutral[150],
        weight: fontWeights.normal,
    },
    boolean: {
        color: colors.neutral[150],
        weight: fontWeights.normal,
    },
    predictive: {
        color: textColor.muted,
        weight: fontWeights.normal,
    },
    title: {
        color: colors.sky[500],
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
