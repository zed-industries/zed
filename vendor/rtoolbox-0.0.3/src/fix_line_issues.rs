/// Normalizes the return of `read_line()` in the context of a CLI application
pub fn fix_line_issues(mut line: String) -> std::io::Result<String> {
    if !line.ends_with('\n') {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "unexpected end of file",
        ));
    }

    // Remove the \n from the line.
    line.pop();

    // Remove the \r from the line if present
    if line.ends_with('\r') {
        line.pop();
    }

    // Ctrl-U should remove the line in terminals
    if line.contains('') {
        line = match line.rfind('') {
            Some(last_ctrl_u_index) => line[last_ctrl_u_index + 1..].to_string(),
            None => line,
        };
    }

    Ok(line)
}
