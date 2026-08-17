fn count_char(ch: char, s: &str) -> usize {
    s.chars().filter(|&c| c == ch).count()
}

pub(super) fn destruct_start_link(s: &str) -> (&'static str, &'static str) {
    let mut str = s.trim();
    let mut edge_type = "arrow_open";
    if let Some(first) = str.as_bytes().first().copied() {
        match first {
            b'<' => {
                edge_type = "arrow_point";
                str = &str[1..];
            }
            b'x' => {
                edge_type = "arrow_cross";
                str = &str[1..];
            }
            b'o' => {
                edge_type = "arrow_circle";
                str = &str[1..];
            }
            _ => {}
        }
    }

    let mut stroke = "normal";
    if str.contains('=') {
        stroke = "thick";
    }
    if str.contains('.') {
        stroke = "dotted";
    }
    (edge_type, stroke)
}

pub(super) fn destruct_end_link(s: &str) -> (String, String, usize) {
    let str = s.trim();
    if str.len() < 2 {
        return ("arrow_open".to_string(), "normal".to_string(), 1);
    }
    let mut line = &str[..str.len() - 1];
    let mut edge_type = "arrow_open".to_string();

    match str.as_bytes()[str.len() - 1] {
        b'x' => {
            edge_type = "arrow_cross".to_string();
            if str.as_bytes().first().copied() == Some(b'x') {
                edge_type = format!("double_{edge_type}");
                line = &line[1..];
            }
        }
        b'>' => {
            edge_type = "arrow_point".to_string();
            if str.as_bytes().first().copied() == Some(b'<') {
                edge_type = format!("double_{edge_type}");
                line = &line[1..];
            }
        }
        b'o' => {
            edge_type = "arrow_circle".to_string();
            if str.as_bytes().first().copied() == Some(b'o') {
                edge_type = format!("double_{edge_type}");
                line = &line[1..];
            }
        }
        _ => {}
    }

    let mut stroke = "normal".to_string();
    let mut length = line.len().saturating_sub(1);

    if line.starts_with('=') {
        stroke = "thick".to_string();
    }
    if line.starts_with('~') {
        stroke = "invisible".to_string();
    }

    let dots = count_char('.', line);
    if dots > 0 {
        stroke = "dotted".to_string();
        length = dots;
    }

    (edge_type, stroke, length)
}
