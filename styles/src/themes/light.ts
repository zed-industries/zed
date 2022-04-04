import { colors, fontWeights, NumberToken } from "../tokens";
import { withOpacity } from "../utils/color";
import Theme, { buildPlayer, Syntax } from "./theme";

const backgroundColor = {
    100: {
        base: colors.neutral[100],
        hovered: colors.neutral[150],
        active: colors.neutral[200],
        focused: colors.neutral[150],
    },
    300: {
        base: colors.neutral[50],
        hovered: colors.neutral[100],
        active: colors.neutral[150],
        focused: colors.neutral[100],
    },
    500: {
        base: colors.neutral[0],
        hovered: colors.neutral[25],
        active: colors.neutral[50],
        focused: colors.neutral[75],
    },
    ok: {
        base: colors.green[100],
        hovered: colors.green[100],
        active: colors.green[100],
        focused: colors.green[100],
    },
    error: {
        base: colors.red[100],
        hovered: colors.red[100],
        active: colors.red[100],
        focused: colors.red[100],
    },
    warning: {
        base: colors.yellow[100],
        hovered: colors.yellow[100],
        active: colors.yellow[100],
        focused: colors.yellow[100],
    },
    info: {
        base: colors.blue[100],
        hovered: colors.blue[100],
        active: colors.blue[100],
        focused: colors.blue[100],
    },
};

const borderColor = {
    primary: colors.neutral[200],
    secondary: colors.neutral[100],
    muted: colors.neutral[50],
    focused: colors.neutral[100],
    active: colors.neutral[250],
    ok: colors.green[200],
    error: colors.red[200],
    warning: colors.yellow[200],
    info: colors.blue[200],
};

const textColor = {
    primary: colors.neutral[750],
    secondary: colors.neutral[600],
    muted: colors.neutral[450],
    placeholder: colors.neutral[300],
    active: colors.neutral[900],
    feature: colors.blue[500],
    ok: colors.green[500],
    error: colors.red[500],
    warning: colors.yellow[500],
    info: colors.blue[500],
};

const iconColor = {
    primary: colors.neutral[300],
    secondary: colors.neutral[500],
    muted: colors.neutral[600],
    placeholder: colors.neutral[700],
    active: colors.neutral[900],
    feature: colors.sky[600],
    ok: colors.green[600],
    error: colors.red[600],
    warning: colors.yellow[400],
    info: colors.blue[600],
};

const player = {
    1: buildPlayer(colors.blue[500]),
    2: buildPlayer(colors.lime[500]),
    3: buildPlayer(colors.indigo[500]),
    4: buildPlayer(colors.orange[500]),
    5: buildPlayer(colors.purple[500]),
    6: buildPlayer(colors.teal[400]),
    7: buildPlayer(colors.pink[400]),
    8: buildPlayer(colors.yellow[400]),
};

// TODO: Fixup
const editor = {
    background: backgroundColor[500].base,
    indent_guide: borderColor.muted,
    indent_guide_active: borderColor.secondary,
    line: {
        active: backgroundColor[500].active,
        highlighted: backgroundColor[500].active,
        inserted: backgroundColor.ok.active,
        deleted: backgroundColor.error.active,
        modified: backgroundColor.info.active,
    },
    highlight: {
        selection: player[1].selectionColor,
        occurrence: withOpacity(colors.teal[500], 0.16),
        activeOccurrence: withOpacity(colors.teal[500], 0.32),
        matchingBracket: colors.neutral[0],
        match: withOpacity(colors.sky[500], 0.16),
        activeMatch: withOpacity(colors.sky[800], 0.32),
        related: colors.neutral[0],
    },
    gutter: {
        primary: colors.neutral[300],
        active: textColor.active,
    },
};

const syntax: Syntax = {
    primary: {
        color: colors.neutral[750],
        weight: fontWeights.normal,
    },
    comment: {
        color: colors.neutral[600],
        weight: fontWeights.normal,
    },
    punctuation: {
        color: colors.neutral[700],
        weight: fontWeights.normal,
    },
    constant: {
        color: colors.neutral[700],
        weight: fontWeights.normal,
    },
    keyword: {
        color: colors.blue[800],
        weight: fontWeights.normal,
    },
    function: {
        color: colors.green[600],
        weight: fontWeights.normal,
    },
    type: {
        color: colors.teal[600],
        weight: fontWeights.normal,
    },
    variant: {
        color: colors.sky[600],
        weight: fontWeights.normal,
    },
    property: {
        color: colors.blue[700],
        weight: fontWeights.normal,
    },
    enum: {
        color: colors.orange[600],
        weight: fontWeights.normal,
    },
    operator: {
        color: colors.orange[600],
        weight: fontWeights.normal,
    },
    string: {
        color: colors.orange[600],
        weight: fontWeights.normal,
    },
    number: {
        color: colors.teal[500],
        weight: fontWeights.normal,
    },
    boolean: {
        color: colors.amber[600],
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
        color: colors.teal[500],
        weight: fontWeights.normal,
        // TODO: add underline
    },
    linkText: {
        color: colors.orange[500],
        weight: fontWeights.normal,
        // TODO: add italic
    },
};

const shadowAlpha: NumberToken = {
    value: 0.12,
    type: "number",
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
