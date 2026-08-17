pub(super) fn extract_flowchart_accessibility_statements(
    code: &str,
) -> (String, Option<String>, Option<String>) {
    let mut acc_title: Option<String> = None;
    let mut acc_descr: Option<String> = None;
    let mut out = String::with_capacity(code.len());

    let mut lines = code.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();

        if let Some(rest) = trimmed.strip_prefix("accTitle") {
            let rest = rest.trim_start();
            if let Some(after) = rest.strip_prefix(':') {
                acc_title = Some(after.trim().to_string());
                continue;
            }
        }

        if let Some(rest) = trimmed.strip_prefix("accDescr") {
            let rest = rest.trim_start();
            if let Some(after) = rest.strip_prefix(':') {
                acc_descr = Some(after.trim().to_string());
                continue;
            }

            if let Some(after_lbrace) = rest.strip_prefix('{') {
                let mut buf = String::new();

                let mut after = after_lbrace.to_string();
                if let Some(end) = after.find('}') {
                    after.truncate(end);
                    acc_descr = Some(after.trim().to_string());
                    continue;
                }
                let after = after.trim_start();
                if !after.is_empty() {
                    buf.push_str(after);
                }

                for raw in lines.by_ref() {
                    if let Some(pos) = raw.find('}') {
                        let part = &raw[..pos];
                        if !buf.is_empty() {
                            buf.push('\n');
                        }
                        buf.push_str(part);
                        break;
                    }

                    if !buf.is_empty() {
                        buf.push('\n');
                    }
                    buf.push_str(raw);
                }

                acc_descr = Some(buf.trim().to_string());
                continue;
            }
        }

        out.push_str(line);
        out.push('\n');
    }

    (out, acc_title, acc_descr)
}
