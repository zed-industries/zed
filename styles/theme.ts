import { colorRamp } from "./lib";

export type Color = string;
export type Weight =
  | "thin"
  | "extra_light"
  | "light"
  | "normal"
  | "medium"
  | "semibold"
  | "bold"
  | "extra_bold"
  | "black";

interface SyntaxHighlightStyle {
  color: { value: Color };
  weight: { value: Weight };
}

interface Player {
  baseColor: {
    value: Color;
  };
  cursorColor: {
    value: Color;
  };
  selectionColor: {
    value: Color;
  };
  borderColor: {
    value: Color;
  };
}

export interface BackgroundColor {
  base: {
    value: Color;
  };
  hover: {
    value: Color;
  };
  active: {
    value: Color;
  };
  focused: {
    value: Color;
  };
}

export default interface Theme {
  backgroundColor: {
    100: BackgroundColor;
    300: BackgroundColor;
    500: BackgroundColor;
  };
  borderColor: {
    primary: {
      value: Color;
    };
    secondary: {
      value: Color;
    };
    muted: {
      value: Color;
    };
    focused: {
      value: Color;
    };
    active: {
      value: Color;
    };
  };
  textColor: {
    primary: {
      value: Color;
    };
    secondary: {
      value: Color;
    };
    muted: {
      value: Color;
    };
    placeholder: {
      value: Color;
    };
    active: {
      value: Color;
    };
    feature: {
      value: Color;
    };
    ok: {
      value: Color;
    };
    error: {
      value: Color;
    };
    warning: {
      value: Color;
    };
    info: {
      value: Color;
    };
  };
  iconColor: {
    primary: {
      value: Color;
    };
    secondary: {
      value: Color;
    };
    muted: {
      value: Color;
    };
    placeholder: {
      value: Color;
    };
    active: {
      value: Color;
    };
    feature: {
      value: Color;
    };
    ok: {
      value: Color;
    };
    error: {
      value: Color;
    };
    warning: {
      value: Color;
    };
    info: {
      value: Color;
    };
  };
  syntax: {
    primary: SyntaxHighlightStyle;
    comment: SyntaxHighlightStyle;
    punctuation: SyntaxHighlightStyle;
    constant: SyntaxHighlightStyle;
    keyword: SyntaxHighlightStyle;
    function: SyntaxHighlightStyle;
    type: SyntaxHighlightStyle;
    variant: SyntaxHighlightStyle;
    property: SyntaxHighlightStyle;
    enum: SyntaxHighlightStyle;
    operator: SyntaxHighlightStyle;
    string: SyntaxHighlightStyle;
    number: SyntaxHighlightStyle;
    boolean: SyntaxHighlightStyle;
    predictive: SyntaxHighlightStyle;
  };
  player: {
    1: Player;
    2: Player;
    3: Player;
    4: Player;
    5: Player;
    6: Player;
    7: Player;
    8: Player;
  };
  shadowAlpha: {
    value: number;
  };
}
