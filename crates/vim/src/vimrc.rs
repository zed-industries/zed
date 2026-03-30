use std::rc::Rc;

use gpui::{App, DummyKeyboardMapper, KeyBinding, KeyBindingContextPredicate, NoAction};
use log::warn;
use settings::KeybindSource;
use workspace::SendKeystrokes;

pub struct VimrcError {
    pub line_number: usize,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum VimrcMode {
    Normal,
    Insert,
    Visual,
    All,
}

struct VimrcMapping {
    mode: VimrcMode,
    lhs: String,
    rhs: String,
    line_number: usize,
}

struct VimrcUnmap {
    mode: VimrcMode,
    lhs: String,
    line_number: usize,
}

struct VimrcFile {
    mappings: Vec<VimrcMapping>,
    unmaps: Vec<VimrcUnmap>,
    leader: Option<String>,
    errors: Vec<VimrcError>,
}

pub fn load_vimrc(content: &str, cx: &App) -> (Vec<KeyBinding>, Vec<VimrcError>) {
    let parsed = parse(content);
    to_key_bindings(parsed, cx)
}

fn parse(content: &str) -> VimrcFile {
    let mut mappings = Vec::new();
    let mut unmaps = Vec::new();
    let mut leader: Option<String> = None;
    let mut errors = Vec::new();

    for (line_idx, raw_line) in content.lines().enumerate() {
        let line_number = line_idx + 1;
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('"') {
            continue;
        }

        let Some((command, rest)) = line.split_once(|c: char| c.is_whitespace()) else {
            errors.push(VimrcError {
                line_number,
                message: format!("Unrecognized command: {line}"),
            });
            continue;
        };

        let rest = rest.trim_start();

        match command {
            "nmap" | "nnoremap" => {
                if let Some(mapping) = parse_mapping(VimrcMode::Normal, rest, line_number) {
                    mappings.push(mapping);
                } else {
                    errors.push(VimrcError {
                        line_number,
                        message: format!("Invalid mapping: {line}"),
                    });
                }
            }
            "imap" | "inoremap" => {
                if let Some(mapping) = parse_mapping(VimrcMode::Insert, rest, line_number) {
                    mappings.push(mapping);
                } else {
                    errors.push(VimrcError {
                        line_number,
                        message: format!("Invalid mapping: {line}"),
                    });
                }
            }
            "vmap" | "vnoremap" => {
                if let Some(mapping) = parse_mapping(VimrcMode::Visual, rest, line_number) {
                    mappings.push(mapping);
                } else {
                    errors.push(VimrcError {
                        line_number,
                        message: format!("Invalid mapping: {line}"),
                    });
                }
            }
            "map" | "noremap" => {
                if let Some(mapping) = parse_mapping(VimrcMode::All, rest, line_number) {
                    mappings.push(mapping);
                } else {
                    errors.push(VimrcError {
                        line_number,
                        message: format!("Invalid mapping: {line}"),
                    });
                }
            }
            "nunmap" => {
                if let Some(unmap) = parse_unmap(VimrcMode::Normal, rest, line_number) {
                    unmaps.push(unmap);
                }
            }
            "iunmap" => {
                if let Some(unmap) = parse_unmap(VimrcMode::Insert, rest, line_number) {
                    unmaps.push(unmap);
                }
            }
            "vunmap" => {
                if let Some(unmap) = parse_unmap(VimrcMode::Visual, rest, line_number) {
                    unmaps.push(unmap);
                }
            }
            "unmap" => {
                if let Some(unmap) = parse_unmap(VimrcMode::All, rest, line_number) {
                    unmaps.push(unmap);
                }
            }
            "let" => {
                if let Some(new_leader) = parse_let_mapleader(rest) {
                    leader = Some(new_leader);
                } else {
                    errors.push(VimrcError {
                        line_number,
                        message: format!("Unsupported let statement: {line}"),
                    });
                }
            }
            "set" => {
                // set options are handled at a higher level; skip silently for now
                // since we apply them through VimSettings rather than keybindings
                errors.push(VimrcError {
                    line_number,
                    message: format!("'set' options are not yet supported in vimrc: {line}"),
                });
            }
            _ => {
                errors.push(VimrcError {
                    line_number,
                    message: format!("Unrecognized command: {command}"),
                });
            }
        }
    }

    VimrcFile {
        mappings,
        unmaps,
        leader,
        errors,
    }
}

fn parse_mapping(mode: VimrcMode, rest: &str, line_number: usize) -> Option<VimrcMapping> {
    let (lhs, rhs) = split_lhs_rhs(rest)?;
    Some(VimrcMapping {
        mode,
        lhs: lhs.to_string(),
        rhs: rhs.to_string(),
        line_number,
    })
}

fn parse_unmap(mode: VimrcMode, rest: &str, line_number: usize) -> Option<VimrcUnmap> {
    let lhs = rest.trim();
    if lhs.is_empty() {
        return None;
    }
    Some(VimrcUnmap {
        mode,
        lhs: lhs.to_string(),
        line_number,
    })
}

fn split_lhs_rhs(input: &str) -> Option<(&str, &str)> {
    // The LHS is the first whitespace-delimited token (handling <...> as atomic),
    // and the RHS is everything after the separating whitespace.
    let input = input.trim();
    let lhs_end = find_lhs_end(input)?;
    let rhs = input[lhs_end..].trim_start();
    if rhs.is_empty() {
        return None;
    }
    Some((input[..lhs_end].trim_end(), rhs))
}

fn find_lhs_end(input: &str) -> Option<usize> {
    let mut chars = input.char_indices().peekable();
    let mut last_end = 0;

    while let Some(&(idx, ch)) = chars.peek() {
        if ch.is_whitespace() {
            if last_end > 0 {
                return Some(last_end);
            }
            return Some(idx);
        }
        if ch == '<' {
            // Check if this looks like a <special> key (next char is not whitespace/end)
            let saved_pos = idx;
            chars.next();
            if let Some(&(_, next_ch)) = chars.peek() {
                if next_ch.is_whitespace() || next_ch == '<' {
                    // Bare '<' character, not a special key
                    last_end = saved_pos + 1;
                    continue;
                }
            } else {
                // '<' at end of string
                last_end = saved_pos + 1;
                continue;
            }
            // Scan forward to find matching '>'
            let mut found_close = false;
            while let Some(&(end_idx, c)) = chars.peek() {
                chars.next();
                if c == '>' {
                    last_end = end_idx + 1;
                    found_close = true;
                    break;
                }
                if c.is_whitespace() {
                    // No closing '>' before whitespace, treat '<' as plain char
                    last_end = saved_pos + 1;
                    return Some(last_end);
                }
            }
            if !found_close {
                last_end = saved_pos + 1;
            }
        } else {
            chars.next();
            last_end = idx + ch.len_utf8();
        }
    }

    // If we consumed everything, there's no RHS
    None
}

fn parse_let_mapleader(rest: &str) -> Option<String> {
    // Parses: mapleader = "..." or mapleader="..."
    let rest = rest.trim();
    let rest = rest.strip_prefix("mapleader")?;
    let rest = rest.trim();
    let rest = rest.strip_prefix('=')?;
    let rest = rest.trim();

    // Accept single or double quoted value
    if let Some(inner) = rest.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        Some(unescape_vim_string(inner))
    } else if let Some(inner) = rest.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
        Some(unescape_vim_string(inner))
    } else {
        None
    }
}

fn unescape_vim_string(s: &str) -> String {
    s.replace("\\<", "<")
        .replace("\\\\", "\\")
        .replace("\\\"", "\"")
}

fn context_for_mode(mode: VimrcMode) -> &'static str {
    match mode {
        VimrcMode::Normal => "vim_mode == normal",
        VimrcMode::Insert => "vim_mode == insert",
        VimrcMode::Visual => "vim_mode == visual",
        VimrcMode::All => "VimControl && !menu",
    }
}

fn translate_vim_keys(vim_keys: &str, leader: &Option<String>) -> Result<String, String> {
    let mut result = Vec::new();
    let mut chars = vim_keys.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '<' {
            let mut special = String::new();
            for c in chars.by_ref() {
                if c == '>' {
                    break;
                }
                special.push(c);
            }
            let translated = translate_special_key(&special, leader)?;
            result.push(translated);
        } else {
            result.push(translate_plain_char(ch));
        }
    }

    Ok(result.join(" "))
}

fn translate_special_key(key: &str, leader: &Option<String>) -> Result<String, String> {
    let lower = key.to_lowercase();

    if lower == "leader" {
        let leader_val = leader.as_deref().unwrap_or("\\");
        return translate_vim_keys(leader_val, &None);
    }

    // Modifier combos: C-x, S-x, A-x, M-x, D-x
    if let Some(rest) = lower
        .strip_prefix("c-")
        .or_else(|| lower.strip_prefix("ctrl-"))
    {
        return Ok(format!("ctrl-{rest}"));
    }
    if let Some(rest) = lower
        .strip_prefix("s-")
        .or_else(|| lower.strip_prefix("shift-"))
    {
        return Ok(format!("shift-{rest}"));
    }
    if let Some(rest) = lower
        .strip_prefix("a-")
        .or_else(|| lower.strip_prefix("alt-"))
        .or_else(|| lower.strip_prefix("m-"))
    {
        return Ok(format!("alt-{rest}"));
    }
    if let Some(rest) = lower
        .strip_prefix("d-")
        .or_else(|| lower.strip_prefix("cmd-"))
    {
        return Ok(format!("cmd-{rest}"));
    }

    match lower.as_str() {
        "cr" | "enter" | "return" => Ok("enter".to_string()),
        "esc" | "escape" => Ok("escape".to_string()),
        "space" => Ok("space".to_string()),
        "tab" => Ok("tab".to_string()),
        "bs" | "backspace" => Ok("backspace".to_string()),
        "del" | "delete" => Ok("delete".to_string()),
        "up" => Ok("up".to_string()),
        "down" => Ok("down".to_string()),
        "left" => Ok("left".to_string()),
        "right" => Ok("right".to_string()),
        "home" => Ok("home".to_string()),
        "end" => Ok("end".to_string()),
        "pageup" => Ok("pageup".to_string()),
        "pagedown" => Ok("pagedown".to_string()),
        "nop" => Ok("".to_string()),
        "bar" => Ok("|".to_string()),
        "lt" => Ok("<".to_string()),
        "gt" => Ok(">".to_string()),
        "bslash" => Ok("\\".to_string()),
        "f1" => Ok("f1".to_string()),
        "f2" => Ok("f2".to_string()),
        "f3" => Ok("f3".to_string()),
        "f4" => Ok("f4".to_string()),
        "f5" => Ok("f5".to_string()),
        "f6" => Ok("f6".to_string()),
        "f7" => Ok("f7".to_string()),
        "f8" => Ok("f8".to_string()),
        "f9" => Ok("f9".to_string()),
        "f10" => Ok("f10".to_string()),
        "f11" => Ok("f11".to_string()),
        "f12" => Ok("f12".to_string()),
        other => Err(format!("Unknown special key: <{other}>")),
    }
}

fn translate_plain_char(ch: char) -> String {
    match ch {
        ' ' => "space".to_string(),
        _ => ch.to_string(),
    }
}

fn to_key_bindings(vimrc: VimrcFile, cx: &App) -> (Vec<KeyBinding>, Vec<VimrcError>) {
    let mut bindings = Vec::new();
    let mut errors = vimrc.errors;

    for mapping in &vimrc.mappings {
        let lhs = match translate_vim_keys(&mapping.lhs, &vimrc.leader) {
            Ok(k) => k,
            Err(err) => {
                errors.push(VimrcError {
                    line_number: mapping.line_number,
                    message: err,
                });
                continue;
            }
        };

        if lhs.is_empty() {
            continue;
        }

        let action = resolve_rhs(&mapping.rhs, &vimrc.leader, cx);
        let action = match action {
            Ok(a) => a,
            Err(err) => {
                errors.push(VimrcError {
                    line_number: mapping.line_number,
                    message: err,
                });
                continue;
            }
        };

        let modes = if mapping.mode == VimrcMode::All {
            vec![VimrcMode::Normal, VimrcMode::Visual]
        } else {
            vec![mapping.mode]
        };

        for mode in modes {
            let context_str = context_for_mode(mode);
            let context_predicate = match KeyBindingContextPredicate::parse(context_str) {
                Ok(pred) => Some(Rc::new(pred)),
                Err(err) => {
                    warn!("Failed to parse context predicate '{context_str}': {err}");
                    continue;
                }
            };

            match KeyBinding::load(
                &lhs,
                action.boxed_clone(),
                context_predicate,
                false,
                None,
                &DummyKeyboardMapper,
            ) {
                Ok(mut binding) => {
                    binding.set_meta(KeybindSource::Vimrc.meta());
                    bindings.push(binding);
                }
                Err(err) => {
                    errors.push(VimrcError {
                        line_number: mapping.line_number,
                        message: format!("Invalid keystroke in LHS '{lhs}': {err}"),
                    });
                }
            }
        }
    }

    for unmap in &vimrc.unmaps {
        let lhs = match translate_vim_keys(&unmap.lhs, &vimrc.leader) {
            Ok(k) => k,
            Err(err) => {
                errors.push(VimrcError {
                    line_number: unmap.line_number,
                    message: err,
                });
                continue;
            }
        };

        if lhs.is_empty() {
            continue;
        }

        let modes = if unmap.mode == VimrcMode::All {
            vec![VimrcMode::Normal, VimrcMode::Visual]
        } else {
            vec![unmap.mode]
        };

        for mode in modes {
            let context_str = context_for_mode(mode);
            let context_predicate = match KeyBindingContextPredicate::parse(context_str) {
                Ok(pred) => Some(Rc::new(pred)),
                Err(err) => {
                    warn!("Failed to parse context predicate '{context_str}': {err}");
                    continue;
                }
            };

            match KeyBinding::load(
                &lhs,
                Box::new(NoAction {}),
                context_predicate,
                false,
                None,
                &DummyKeyboardMapper,
            ) {
                Ok(mut binding) => {
                    binding.set_meta(KeybindSource::Vimrc.meta());
                    bindings.push(binding);
                }
                Err(err) => {
                    errors.push(VimrcError {
                        line_number: unmap.line_number,
                        message: format!("Invalid keystroke in unmap '{lhs}': {err}"),
                    });
                }
            }
        }
    }

    (bindings, errors)
}

fn resolve_rhs(
    rhs: &str,
    leader: &Option<String>,
    cx: &App,
) -> Result<Box<dyn gpui::Action>, String> {
    // Case 1: Ex-command — starts with ':' and optionally ends with <CR>
    if let Some(command) = rhs.strip_prefix(':') {
        let command = command
            .strip_suffix("<CR>")
            .or_else(|| command.strip_suffix("<cr>"))
            .or_else(|| command.strip_suffix("<Enter>"))
            .or_else(|| command.strip_suffix("<enter>"))
            .unwrap_or(command)
            .trim();

        // Try to resolve as a Zed action name directly (e.g., `:edit` -> known command)
        if let Ok(action) = cx.build_action(command, None) {
            return Ok(action);
        }

        // Fall back to SendKeystrokes with the : prefix and enter
        let keystrokes = format!(": {command} enter");
        return Ok(Box::new(SendKeystrokes(keystrokes)));
    }

    // Case 2: Zed action — contains "::" (e.g., "editor::GoToDefinition")
    if rhs.contains("::") {
        return cx
            .build_action(rhs, None)
            .map_err(|err| format!("Unknown action '{rhs}': {err}"));
    }

    // Case 3: <Nop> — no operation
    let lower = rhs.to_lowercase();
    if lower == "<nop>" {
        return Ok(Box::new(NoAction {}));
    }

    // Case 4: Key sequence — translate and use SendKeystrokes
    let translated = translate_vim_keys(rhs, leader)?;
    if translated.is_empty() {
        return Ok(Box::new(NoAction {}));
    }
    Ok(Box::new(SendKeystrokes(translated)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let vimrc = parse("");
        assert!(vimrc.mappings.is_empty());
        assert!(vimrc.errors.is_empty());
        assert!(vimrc.leader.is_none());
    }

    #[test]
    fn test_parse_comments_and_blank_lines() {
        let content = r#"
" This is a comment
" Another comment

"#;
        let vimrc = parse(content);
        assert!(vimrc.mappings.is_empty());
        assert!(vimrc.errors.is_empty());
    }

    #[test]
    fn test_parse_nnoremap() {
        let content = "nnoremap <leader>f :Files<CR>";
        let vimrc = parse(content);
        assert_eq!(vimrc.mappings.len(), 1);
        assert_eq!(vimrc.mappings[0].mode, VimrcMode::Normal);
        assert_eq!(vimrc.mappings[0].lhs, "<leader>f");
        assert_eq!(vimrc.mappings[0].rhs, ":Files<CR>");
    }

    #[test]
    fn test_parse_inoremap() {
        let content = "inoremap jk <Esc>";
        let vimrc = parse(content);
        assert_eq!(vimrc.mappings.len(), 1);
        assert_eq!(vimrc.mappings[0].mode, VimrcMode::Insert);
        assert_eq!(vimrc.mappings[0].lhs, "jk");
        assert_eq!(vimrc.mappings[0].rhs, "<Esc>");
    }

    #[test]
    fn test_parse_vnoremap() {
        let content = "vnoremap < <gv";
        let vimrc = parse(content);
        assert_eq!(vimrc.mappings.len(), 1);
        assert_eq!(vimrc.mappings[0].mode, VimrcMode::Visual);
        assert_eq!(vimrc.mappings[0].lhs, "<");
        assert_eq!(vimrc.mappings[0].rhs, "<gv");
    }

    #[test]
    fn test_parse_let_mapleader_space() {
        let content = r#"let mapleader = " ""#;
        let vimrc = parse(content);
        assert_eq!(vimrc.leader.as_deref(), Some(" "));
    }

    #[test]
    fn test_parse_let_mapleader_comma() {
        let content = "let mapleader = ','";
        let vimrc = parse(content);
        assert_eq!(vimrc.leader.as_deref(), Some(","));
    }

    #[test]
    fn test_parse_unmap() {
        let content = "nunmap <C-w>";
        let vimrc = parse(content);
        assert_eq!(vimrc.unmaps.len(), 1);
        assert_eq!(vimrc.unmaps[0].mode, VimrcMode::Normal);
        assert_eq!(vimrc.unmaps[0].lhs, "<C-w>");
    }

    #[test]
    fn test_parse_unrecognized_command() {
        let content = "autocmd BufRead * echo 'hello'";
        let vimrc = parse(content);
        assert_eq!(vimrc.errors.len(), 1);
        assert!(vimrc.errors[0].message.contains("Unrecognized"));
    }

    #[test]
    fn test_parse_multiple_mappings() {
        let content = r#"
let mapleader = " "
nnoremap <leader>f :Files<CR>
inoremap jk <Esc>
vnoremap < <gv
nunmap <C-a>
"#;
        let vimrc = parse(content);
        assert_eq!(vimrc.mappings.len(), 3);
        assert_eq!(vimrc.unmaps.len(), 1);
        assert_eq!(vimrc.leader.as_deref(), Some(" "));
        assert!(vimrc.errors.is_empty());
    }

    #[test]
    fn test_translate_plain_chars() {
        let result = translate_vim_keys("jk", &None).unwrap();
        assert_eq!(result, "j k");
    }

    #[test]
    fn test_translate_escape() {
        let result = translate_vim_keys("<Esc>", &None).unwrap();
        assert_eq!(result, "escape");
    }

    #[test]
    fn test_translate_ctrl() {
        let result = translate_vim_keys("<C-w>", &None).unwrap();
        assert_eq!(result, "ctrl-w");
    }

    #[test]
    fn test_translate_shift() {
        let result = translate_vim_keys("<S-Tab>", &None).unwrap();
        assert_eq!(result, "shift-tab");
    }

    #[test]
    fn test_translate_alt() {
        let result = translate_vim_keys("<A-x>", &None).unwrap();
        assert_eq!(result, "alt-x");
    }

    #[test]
    fn test_translate_cmd() {
        let result = translate_vim_keys("<D-s>", &None).unwrap();
        assert_eq!(result, "cmd-s");
    }

    #[test]
    fn test_translate_cr() {
        let result = translate_vim_keys("<CR>", &None).unwrap();
        assert_eq!(result, "enter");
    }

    #[test]
    fn test_translate_space() {
        let result = translate_vim_keys("<Space>", &None).unwrap();
        assert_eq!(result, "space");
    }

    #[test]
    fn test_translate_leader_default() {
        let result = translate_vim_keys("<leader>f", &None).unwrap();
        assert_eq!(result, "\\ f");
    }

    #[test]
    fn test_translate_leader_space() {
        let leader = Some(" ".to_string());
        let result = translate_vim_keys("<leader>ff", &leader).unwrap();
        assert_eq!(result, "space f f");
    }

    #[test]
    fn test_translate_leader_comma() {
        let leader = Some(",".to_string());
        let result = translate_vim_keys("<leader>w", &leader).unwrap();
        assert_eq!(result, ", w");
    }

    #[test]
    fn test_translate_complex_sequence() {
        let leader = Some(" ".to_string());
        let result = translate_vim_keys("<leader><C-w>h", &leader).unwrap();
        assert_eq!(result, "space ctrl-w h");
    }

    #[test]
    fn test_translate_nop() {
        let result = translate_vim_keys("<Nop>", &None).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_translate_function_keys() {
        let result = translate_vim_keys("<F5>", &None).unwrap();
        assert_eq!(result, "f5");
    }

    #[test]
    fn test_split_lhs_rhs_simple() {
        let (lhs, rhs) = split_lhs_rhs("jk <Esc>").unwrap();
        assert_eq!(lhs, "jk");
        assert_eq!(rhs, "<Esc>");
    }

    #[test]
    fn test_split_lhs_rhs_special_key_lhs() {
        let (lhs, rhs) = split_lhs_rhs("<leader>f :Files<CR>").unwrap();
        assert_eq!(lhs, "<leader>f");
        assert_eq!(rhs, ":Files<CR>");
    }

    #[test]
    fn test_split_lhs_rhs_angle_bracket_char() {
        let (lhs, rhs) = split_lhs_rhs("< <gv").unwrap();
        assert_eq!(lhs, "<");
        assert_eq!(rhs, "<gv");
    }

    #[test]
    fn test_split_lhs_rhs_ctrl_combo() {
        let (lhs, rhs) = split_lhs_rhs("<C-w>h <C-w>l").unwrap();
        assert_eq!(lhs, "<C-w>h");
        assert_eq!(rhs, "<C-w>l");
    }

    #[test]
    fn test_parse_map_command() {
        let content = "map <C-n> :noh<CR>";
        let vimrc = parse(content);
        assert_eq!(vimrc.mappings.len(), 1);
        assert_eq!(vimrc.mappings[0].mode, VimrcMode::All);
    }

    #[test]
    fn test_context_for_mode() {
        assert_eq!(context_for_mode(VimrcMode::Normal), "vim_mode == normal");
        assert_eq!(context_for_mode(VimrcMode::Insert), "vim_mode == insert");
        assert_eq!(context_for_mode(VimrcMode::Visual), "vim_mode == visual");
        assert_eq!(
            context_for_mode(VimrcMode::All),
            "VimControl && !menu"
        );
    }
}
