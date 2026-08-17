/// Implementation must match exactly
/// `ToJsonName()` function in C++ `descriptor.cc`.
pub fn json_name(input: &str) -> String {
    let mut capitalize_next = false;
    let mut result = String::with_capacity(input.len());

    for c in input.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}
