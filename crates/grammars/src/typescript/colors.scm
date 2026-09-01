; Colors written as string literals: "#ff00aa", "rgb(255, 0, 170)".
((string) @color.text @color
 (#match? @color.text "^[\"'`](#[0-9a-fA-F]{3,8}|(rgb|rgba|hsl|hsla)\\([^)]*\\))[\"'`]$"))
