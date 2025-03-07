/// This module accepts fragments of Lua code from LLM responses, and executes
/// them as they come in (to the extent possible) rather than having to wait
/// for the entire script to arrive to execute it. (Since these are tool calls,
/// they will presumably come back in JSON; it's up to the caller to deal with
/// parsing the JSON, escaping `\\` and `\"` in the JSON-quoted Lua, etc.)
///
/// By design, Lua does not preserve top-level locals across chunks ("chunk" is a
/// Lua term for a chunk of Lua code that can be executed), and chunks are the
/// smallest unit of execution you can run in Lua. To make sure that top-level
/// locals the LLM writes are preserved across multiple silently translates
/// locals to globals. This should be harmless for our use case, because we only
/// have a single "file" and not multiple files where the distinction could matter.
///
/// Since fragments will invariably arrive that don't happen to correspond to valid
/// Lua chunks (e.g. maybe they have an opening quote for a string literal and the
/// close quote will be coming in the next fragment), we use a simple heuristic to
/// split them up: we take each fragment and split it into lines, and then whenever
/// we have a complete line, we send it to Lua to process as a chunk. If it comes back
/// with a syntax error due to it being incomplete (which mlua tells us), then we
/// know to keep waiting for more lines and try again.
///
/// Eventually we'll either succeed, or else the response will end and we'll know it
/// had an actual syntax error. (Again, it's the caller's responsibility to deal
/// with detecting when the response ends due to the JSON quote having finally closed.)
///
/// This heuristic relies on the assumption that the LLM is generating normal-looking
/// Lua code where statements are split using newlines rather than semicolons.
/// In practice, this is a safe assumption.

#[derive(Default)]
struct ChunkBuffer {
    buffer: String,
    incomplete_multiline_string: bool,
    last_newline_index: usize,
}

impl ChunkBuffer {
    pub fn receive_chunk(
        &mut self,
        src_chunk: &str,
        exec_chunk: &mut impl FnMut(&str) -> mlua::Result<()>,
    ) -> mlua::Result<()> {
        self.buffer.push_str(src_chunk);

        // Execute each line until we hit an incomplete parse
        while let Some(index) = &self.buffer[self.last_newline_index..].find('\n') {
            let mut index = *index;

            // LLMs can produce incredibly long multiline strings. We don't want to keep
            // attempting to re-parse those every time a new line of the string comes in.
            // that would be extremely wasteful! Instead, just keep waiting until it ends.
            {
                let line = &self.buffer[self.last_newline_index..index];

                const LOCAL_PREFIX: &str = "local ";

                // It's safe to assume we'll never see a line which
                // includes both "]]" and "[[" other than single-line
                // assignments which are just using them to escape quotes.
                //
                // If that assumption turns out not to hold, we can always
                // make this more robust.
                if line.contains("[[") && !line.contains("]]") {
                    self.incomplete_multiline_string = true;
                }

                // In practice, LLMs produce multiline strings that always end
                // with the ]] at the start of the line.
                if line.starts_with("]]") {
                    self.incomplete_multiline_string = false;
                } else if line.starts_with("local ") {
                    // We can't have top-level locals because they don't preserve
                    // across chunk executions. So just turn locals into globals.
                    // Since this is just one script, they're the same anyway.
                    self.buffer
                        .replace_range(self.last_newline_index..LOCAL_PREFIX.len(), "");

                    index -= LOCAL_PREFIX.len();
                }
            }

            self.last_newline_index = index;

            if self.incomplete_multiline_string {
                continue;
            }

            // Execute all lines up to (and including) this one.
            match exec_chunk(&self.buffer[..index]) {
                Ok(()) => {
                    // The chunk executed successfully. Advance the buffer
                    // to reflect the fact that we've executed that code.
                    self.buffer = self.buffer[index + 1..].to_string();
                    self.last_newline_index = 0;
                }
                Err(mlua::Error::SyntaxError {
                    incomplete_input: true,
                    message: _,
                }) => {
                    // If it errored specifically because the input was incomplete, no problem.
                    // We'll keep trying with more and more lines until eventually we find a
                    // sequence of lines that are valid together!
                }
                Err(other) => {
                    return Err(other);
                }
            }
        }

        Ok(())
    }

    pub fn finish(
        &mut self,
        exec_chunk: &mut impl FnMut(&str) -> mlua::Result<()>,
    ) -> mlua::Result<()> {
        if !self.buffer.is_empty() {
            // Execute whatever is left in the buffer
            match exec_chunk(&self.buffer) {
                Ok(()) => {
                    // Clear the buffer as everything has been executed
                    self.buffer.clear();
                    self.last_newline_index = 0;
                    self.incomplete_multiline_string = false;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_lua_runtime_receive_chunk() {
        let mut chunk_buffer = ChunkBuffer::default();
        let output = Rc::new(RefCell::new(String::new()));

        let mut exec_chunk = |chunk: &str| -> mlua::Result<()> {
            let lua = Lua::new();

            // Clone the Rc to share ownership of the same RefCell
            let output_ref = output.clone();

            lua.globals().set(
                "print",
                lua.create_function(move |_, msg: String| {
                    let mut output = output_ref.borrow_mut();
                    output.push_str(&msg);
                    output.push('\n');
                    Ok(())
                })?,
            )?;

            lua.load(chunk).exec()
        };

        exec_chunk("print('Hello, World!')").unwrap();

        chunk_buffer
            .receive_chunk("print('Hello, World!')", &mut exec_chunk)
            .unwrap();

        assert_eq!(*output.borrow(), "Hello, World!\n");
    }

    #[test]
    fn test_lua_runtime_receive_chunk_shared_lua() {
        let mut chunk_buffer = ChunkBuffer::default();
        let output = Rc::new(RefCell::new(String::new()));
        let lua = Lua::new();

        // Set up the print function once for the shared Lua instance
        {
            let output_ref = output.clone();
            lua.globals()
                .set(
                    "print",
                    lua.create_function(move |_, msg: String| {
                        let mut output = output_ref.borrow_mut();
                        output.push_str(&msg);
                        output.push('\n');
                        Ok(())
                    })
                    .unwrap(),
                )
                .unwrap();
        }

        let mut exec_chunk = |chunk: &str| -> mlua::Result<()> { lua.load(chunk).exec() };

        // Send first incomplete chunk
        chunk_buffer
            .receive_chunk("local message = 'Hello, '\n", &mut exec_chunk)
            .unwrap();

        // Send second chunk that completes the code
        chunk_buffer
            .receive_chunk(
                "message = message .. 'World!'\nprint(message)",
                &mut exec_chunk,
            )
            .unwrap();

        chunk_buffer.finish(&mut exec_chunk).unwrap();

        assert_eq!(*output.borrow(), "Hello, World!\n");
    }

    #[test]
    fn test_multiline_string_across_chunks() {
        let mut chunk_buffer = ChunkBuffer::default();
        let output = Rc::new(RefCell::new(String::new()));
        let lua = Lua::new();

        // Set up the print function for the shared Lua instance
        {
            let output_ref = output.clone();
            lua.globals()
                .set(
                    "print",
                    lua.create_function(move |_, msg: String| {
                        let mut output = output_ref.borrow_mut();
                        output.push_str(&msg);
                        output.push('\n');
                        Ok(())
                    })
                    .unwrap(),
                )
                .unwrap();
        }

        let mut exec_chunk = |chunk: &str| -> mlua::Result<()> { lua.load(chunk).exec() };

        // Send first chunk with the beginning of a multiline string
        chunk_buffer
            .receive_chunk("local multiline = [[This is the start\n", &mut exec_chunk)
            .unwrap();

        // Send second chunk with more lines
        chunk_buffer
            .receive_chunk("of a very long\nmultiline string\n", &mut exec_chunk)
            .unwrap();

        // Send third chunk with more content
        chunk_buffer
            .receive_chunk("that spans across\n", &mut exec_chunk)
            .unwrap();

        // Send final chunk that completes the multiline string
        chunk_buffer
            .receive_chunk("multiple chunks]]\nprint(multiline)", &mut exec_chunk)
            .unwrap();

        chunk_buffer.finish(&mut exec_chunk).unwrap();

        let expected = "This is the start\nof a very long\nmultiline string\nthat spans across\nmultiple chunks\n";
        assert_eq!(*output.borrow(), expected);
    }
}
