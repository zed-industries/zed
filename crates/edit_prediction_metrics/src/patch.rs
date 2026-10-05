#[derive(Debug, Default, Clone)]
pub struct Patch {
    pub hunks: Vec<Hunk>,
}

impl Patch {
    pub fn parse_unified_diff(unified_diff: &str) -> Patch {
        let mut current_file = String::new();
        let mut is_filename_inherited = false;
        let mut hunk = Hunk::default();
        let mut patch = Patch::default();
        let mut in_header = true;

        for line in unified_diff.lines() {
            if line.starts_with("--- ") || line.starts_with("+++") || line.starts_with("@@") {
                in_header = false;
            }

            if in_header {
                continue;
            }

            if line.starts_with("@@") {
                if !hunk.lines.is_empty() {
                    patch.hunks.push(hunk);
                }
                hunk = Hunk::from_header(line, &current_file, is_filename_inherited);
                is_filename_inherited = true;
            } else if let Some(path) = line.strip_prefix("--- ") {
                is_filename_inherited = false;
                let path = path.trim().strip_prefix("a/").unwrap_or(path);
                if path != "/dev/null" {
                    current_file = path.into();
                }
            } else if let Some(path) = line.strip_prefix("+++ ") {
                is_filename_inherited = false;
                let path = path.trim().strip_prefix("b/").unwrap_or(path);
                if path != "/dev/null" {
                    current_file = path.into();
                }
            } else if let Some(line) = line.strip_prefix('+') {
                hunk.lines.push(PatchLine::Addition(line.to_string()));
            } else if let Some(line) = line.strip_prefix('-') {
                hunk.lines.push(PatchLine::Deletion(line.to_string()));
            } else if let Some(line) = line.strip_prefix(' ') {
                hunk.lines.push(PatchLine::Context(line.to_string()));
            } else {
                hunk.lines.push(PatchLine::Garbage(line.to_string()));
            }
        }

        if !hunk.lines.is_empty() {
            patch.hunks.push(hunk);
        }

        patch
    }
}

#[derive(Debug, Default, Clone)]
pub struct Hunk {
    pub old_start: isize,
    pub new_start: isize,
    pub lines: Vec<PatchLine>,
    pub filename: String,
}

impl Hunk {
    pub fn from_header(header: &str, filename: &str, _is_filename_inherited: bool) -> Self {
        let (old_start, _, new_start, _, _) = Self::parse_hunk_header(header);
        Self {
            old_start,
            new_start,
            lines: Vec::new(),
            filename: filename.to_string(),
        }
    }

    fn parse_hunk_header(line: &str) -> (isize, isize, isize, isize, String) {
        let header_part = line.trim_start_matches("@@").trim();
        let parts: Vec<&str> = header_part.split_whitespace().collect();

        if parts.len() < 2 {
            return (0, 0, 0, 0, String::new());
        }

        let old_part = parts[0].trim_start_matches('-');
        let new_part = parts[1].trim_start_matches('+');

        let (old_start, old_count) = Hunk::parse_hunk_header_range(old_part);
        let (new_start, new_count) = Hunk::parse_hunk_header_range(new_part);

        let comment = if parts.len() > 2 {
            parts[2..]
                .join(" ")
                .trim_start_matches("@@")
                .trim()
                .to_string()
        } else {
            String::new()
        };

        (
            old_start as isize,
            old_count as isize,
            new_start as isize,
            new_count as isize,
            comment,
        )
    }

    fn parse_hunk_header_range(part: &str) -> (usize, usize) {
        if let Some((start, count)) = part.split_once(',') {
            (start.parse().unwrap_or(0), count.parse().unwrap_or(0))
        } else {
            (part.parse().unwrap_or(0), 1)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PatchLine {
    Context(String),
    Addition(String),
    Deletion(String),
    Garbage(String),
}
