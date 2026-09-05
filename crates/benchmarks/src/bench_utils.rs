use rand::Rng;

pub const RUST_MODULE_HEADER_LINES: usize = 10;
pub const RUST_FUNCTION_LINES: usize = 12;
pub const RUST_FUNCTION_BODY_LINES: usize = 11;
pub const RUST_MODULE_FOOTER_LINES: usize = 5;

pub fn rust_file_line_count(function_count: usize) -> usize {
    RUST_MODULE_HEADER_LINES + function_count * RUST_FUNCTION_LINES + RUST_MODULE_FOOTER_LINES
}

pub fn random_rust_file(rng: &mut impl Rng, line_count: usize) -> Vec<String> {
    if line_count < RUST_MODULE_HEADER_LINES + RUST_MODULE_FOOTER_LINES {
        return (0..line_count)
            .map(|line_index| format!("// generated benchmark line {line_index}"))
            .collect();
    }

    let mut lines = vec![
        "use anyhow::{Context as _, Result};".to_string(),
        "use collections::HashMap;".to_string(),
        "".to_string(),
        "#[derive(Clone, Debug)]".to_string(),
        "pub struct WorkspaceSnapshot {".to_string(),
        "    buffers: HashMap<String, usize>,".to_string(),
        "    version: usize,".to_string(),
        "}".to_string(),
        "".to_string(),
        "impl WorkspaceSnapshot {".to_string(),
    ];

    let body_line_count = line_count - RUST_MODULE_HEADER_LINES - RUST_MODULE_FOOTER_LINES;
    let function_count = body_line_count / RUST_FUNCTION_LINES;
    let filler_line_count = body_line_count % RUST_FUNCTION_LINES;

    for function_index in 0..function_count {
        let function_name = rust_identifier(rng, function_index);
        let argument_name = rust_identifier(rng, function_index + 1_000);
        let local_name = rust_identifier(rng, function_index + 2_000);
        let branch_name = rust_identifier(rng, function_index + 3_000);
        let multiplier = rng.random_range(2..17);
        let offset = rng.random_range(1..128);

        lines.extend([
            format!(
                "    pub fn {function_name}(&mut self, {argument_name}: usize) -> Result<usize> {{"
            ),
            format!("        let mut {local_name} = {argument_name}.saturating_mul({multiplier});"),
            format!("        if {local_name} % 2 == 0 {{"),
            format!(
                "            {local_name} = {local_name}.saturating_add(self.version + {offset});"
            ),
            "        } else {".to_string(),
            format!("            {local_name} = {local_name}.saturating_sub({offset});"),
            "        }".to_string(),
            format!("        let {branch_name} = self.buffers.len().saturating_add({local_name});"),
            format!("        self.version = self.version.saturating_add({branch_name});"),
            format!("        Ok({branch_name})"),
            "    }".to_string(),
            "".to_string(),
        ]);
    }

    for filler_index in 0..filler_line_count {
        let filler_name = rust_identifier(rng, function_count + 4_000 + filler_index);
        lines.push(format!("    // benchmark filler {filler_name}"));
    }

    lines.push("}".to_string());
    lines.push("".to_string());
    lines.push("pub fn normalize_path(path: &str) -> String {".to_string());
    lines.push("    path.replace('\\\\', \"/\")".to_string());
    lines.push("}".to_string());

    debug_assert_eq!(lines.len(), line_count);
    lines
}

pub fn rust_identifier(rng: &mut impl Rng, salt: usize) -> String {
    const PARTS: &[&str] = &[
        "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "theta", "lambda", "sigma", "omega",
    ];
    format!(
        "{}_{}_{}",
        PARTS[rng.random_range(0..PARTS.len())],
        salt,
        rng.random_range(0..10_000)
    )
}
