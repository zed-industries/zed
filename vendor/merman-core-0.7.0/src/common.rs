pub fn count_occurrence(haystack: &str, needle: char) -> usize {
    haystack.chars().filter(|c| *c == needle).count()
}

fn should_combine_sets(previous_set: &str, next_set: &str) -> bool {
    let prev_count = count_occurrence(previous_set, '~');
    let next_count = count_occurrence(next_set, '~');
    prev_count == 1 && next_count == 1
}

fn process_set(mut input: String) -> String {
    let tilde_count = count_occurrence(&input, '~');
    if tilde_count <= 1 {
        return input;
    }

    let mut has_starting_tilde = false;
    if !tilde_count.is_multiple_of(2) && input.starts_with('~') {
        input = input[1..].to_string();
        has_starting_tilde = true;
    }

    let mut chars: Vec<char> = input.chars().collect();
    loop {
        let first = chars.iter().position(|c| *c == '~');
        let last = chars.iter().rposition(|c| *c == '~');
        let (Some(first), Some(last)) = (first, last) else {
            break;
        };
        if first == last {
            break;
        }
        chars[first] = '<';
        chars[last] = '>';
    }

    if has_starting_tilde {
        chars.insert(0, '~');
    }

    chars.into_iter().collect()
}

pub fn parse_generic_types(input: &str) -> String {
    // Mirrors Mermaid's `parseGenericTypes` logic (packages/mermaid/src/diagrams/common/common.ts).
    // Regex split `/(,)/` with capture: keep `,` as separate tokens.
    if !input.contains('~') {
        return input.to_string();
    }
    let mut input_sets: Vec<String> = Vec::new();
    let mut cur = String::new();
    for ch in input.chars() {
        if ch == ',' {
            input_sets.push(cur);
            cur = String::new();
            input_sets.push(",".to_string());
        } else {
            cur.push(ch);
        }
    }
    input_sets.push(cur);

    let mut output: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i < input_sets.len() {
        let mut this_set = input_sets[i].clone();
        if this_set == "," && i > 0 && i + 1 < input_sets.len() {
            let previous_set = input_sets[i - 1].clone();
            let next_set = input_sets[i + 1].clone();
            if should_combine_sets(&previous_set, &next_set) {
                this_set = format!("{previous_set},{next_set}");
                i += 1;
                output.pop();
            }
        }
        output.push(process_set(this_set));
        i += 1;
    }

    output.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_generic_types_matches_upstream_examples() {
        let cases = [
            ("test~T~", "test<T>"),
            ("test~Array~Array~string~~~", "test<Array<Array<string>>>"),
            (
                "test~Array~Array~string[]~~~",
                "test<Array<Array<string[]>>>",
            ),
            (
                "test ~Array~Array~string[]~~~",
                "test <Array<Array<string[]>>>",
            ),
            ("~test", "~test"),
            ("~test~T~", "~test<T>"),
        ];
        for (input, expected) in cases {
            assert_eq!(parse_generic_types(input), expected);
        }
    }
}
