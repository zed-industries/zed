pub(crate) fn map_keystroke(keystroke: &str) -> String {
    match keystroke {
        "`" => "\\",
        "-" => "=",
        "=" => "'",
        "[" => "ú",
        "]" => ")",
        "\\" => "¨",
        ";" => "ů",
        "'" => "§",
        "," => ",",
        // "." => ".", same on both layouts
        "/" => "-",
        "1" => "+",
        "2" => "ě",
        "3" => "š",
        "4" => "č",
        "5" => "ř",
        "6" => "ž",
        "7" => "ý",
        "8" => "á",
        "9" => "í",
        "0" => "é",
        // mapping shift-;
        ":" => "\"",
        _ => keystroke,
    }
    .to_string()
}
