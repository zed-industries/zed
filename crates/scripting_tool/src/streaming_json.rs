/// This module works with streaming_lua to allow us to run fragments of
/// Lua scripts that come back from LLM JSON tool calls immediately as they arrive,
/// even when the full script (and the full JSON) has not been received yet.

pub fn from_json(json_str: &str) {
    // The JSON structure we're looking for is very simple:
    // 1. Open curly bracket
    // 2. Optional whitespace
    // 3. Quoted key - either "lua_script" or "description" (if description, just parse it)
    // 4. Colon
    // 5. Optional whitespace
    // 6. Open quote
    // 7. Now we start streaming until we see a closed quote

    // TODO all of this needs to be stored in state in a struct instead of in variables,
    // and that includes the iterator part.
    let mut chars = json_str.trim_start().chars().peekable();

    // Skip the opening curly brace
    if chars.next() != Some('{') {
        return;
    }

    let key = parse_key(&mut chars);

    if key.map(|k| k.as_str()) == Some("description") {
        // TODO parse the description here
        parse_comma_then_quote(&mut chars);
        if parse_key(&mut chars).map(|k| k.as_str()) != Some("lua_script") {
            return; // This was the only remaining valid option.
        }
        // TODO parse the script here, remembering to s/backslash//g to unescape everything.
    } else if key.map(|k| k.as_str()) == Some("lua_script") {
        // TODO parse the script here, remembering to s/backslash//g to unescape everything.
        parse_comma_then_quote(&mut chars);
        if parse_key(&mut chars).map(|k| k.as_str()) != Some("description") {
            return; // This was the only remaining valid option.
        }
        // TODO parse the description here
    } else {
        // The key wasn't one of the two valid options.
        return;
    }

    // Parse value
    let mut value = String::new();
    let mut escape_next = false;

    while let Some(c) = chars.next() {
        if escape_next {
            value.push(match c {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '"' => '"',
                _ => c,
            });
            escape_next = false;
        } else if c == '\\' {
            escape_next = true;
        } else if c == '"' {
            break; // End of value
        } else {
            value.push(c);
        }
    }

    // Process the parsed key-value pair
    match key.as_str() {
        "lua_script" => {
            // Handle the lua script
            println!("Found lua script: {}", value);
        }
        "description" => {
            // Handle the description
            println!("Found description: {}", value);
        }
        _ => {} // Should not reach here due to earlier check
    }
}

fn parse_key(chars: &mut impl Iterator<Item = char>) -> Option<String> {
    // Skip whitespace until we reach the start of the key
    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            // Consume the whitespace and continue
        } else if c == '"' {
            break; // Found the start of the key
        } else {
            return None; // Invalid format - expected a quote to start the key
        }
    }

    // Parse the key. We don't need to escape backslashes because the exact key
    // we expect does not include backslashes or quotes.
    let mut key = String::new();

    while let Some(c) = chars.next() {
        if c == '"' {
            break; // End of key
        }
        key.push(c);
    }

    // Skip colon and whitespace and next opening quote.
    let mut found_colon = false;
    while let Some(c) = chars.next() {
        if c == ':' {
            found_colon = true;
        } else if found_colon && !c.is_whitespace() {
            if c == '"' {
                break; // Found the opening quote
            }
            return None; // Invalid format - expected a quote after colon and whitespace
        } else if !c.is_whitespace() {
            return None; // Invalid format - expected whitespace or colon
        }
    }

    Some(key)
}

fn parse_comma_then_quote(chars: &mut impl Iterator<Item = char>) -> bool {
    // Skip any whitespace
    while let Some(&c) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        chars.next();
    }

    // Check for comma
    if chars.next() != Some(',') {
        return false;
    }

    // Skip any whitespace after the comma
    while let Some(&c) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        chars.next();
    }

    // Check for opening quote
    if chars.next() != Some('"') {
        return false;
    }

    true
}
