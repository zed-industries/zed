; Hex colors in strings, as used throughout theme and config files.
((string) @color.text @color
 (#match? @color.text "^\"#[0-9a-fA-F]{3,8}\"$"))
