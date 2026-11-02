use futures::AsyncRead;
use smol::io::AsyncReadExt;
use std::io;

pub(crate) async fn read(
    mut stderr: impl AsyncRead + Unpin,
    mut on_progress: impl FnMut(String),
) -> io::Result<Vec<u8>> {
    let mut stderr_output = Vec::new();
    let mut read_buffer = [0; 8192];
    let mut progress = GitCloneProgress::new();

    loop {
        let bytes_read = stderr.read(&mut read_buffer).await?;
        if bytes_read == 0 {
            break;
        }
        let bytes = &read_buffer[..bytes_read];
        stderr_output.extend_from_slice(bytes);
        progress.push(bytes, &mut on_progress);
    }
    progress.finish(on_progress);
    Ok(stderr_output)
}

pub(crate) fn failure_message(stderr: &[u8]) -> String {
    String::from_utf8_lossy(stderr)
        .split(['\r', '\n'])
        .rev()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("unknown error")
        .to_owned()
}

struct GitCloneProgress {
    pending: Vec<u8>,
    last_message: Option<String>,
}

impl GitCloneProgress {
    fn new() -> Self {
        Self {
            pending: Vec::new(),
            last_message: None,
        }
    }

    fn push(&mut self, bytes: &[u8], mut on_progress: impl FnMut(String)) {
        for byte in bytes {
            if matches!(*byte, b'\r' | b'\n') {
                self.flush(&mut on_progress);
            } else {
                self.pending.push(*byte);
            }
        }
    }

    fn finish(&mut self, mut on_progress: impl FnMut(String)) {
        self.flush(&mut on_progress);
    }

    fn flush(&mut self, on_progress: &mut impl FnMut(String)) {
        if self.pending.is_empty() {
            return;
        }

        let message = String::from_utf8_lossy(&self.pending).trim().to_owned();
        if !message.is_empty() && self.last_message.as_ref() != Some(&message) {
            self.last_message = Some(message.clone());
            on_progress(message);
        }
        self.pending.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_fragmented_progress_and_suppresses_duplicates() {
        let mut progress = GitCloneProgress::new();
        let mut messages = Vec::new();

        progress.push(
            b"remote: Enumerating objects: 100, done.\rremote: Count",
            |message| messages.push(message),
        );
        progress.push(
            b"ing objects: 42% (42/100)\rReceiving objects: 100% (100/100)\rReceiving objects: 100% (100/100)\n",
            |message| messages.push(message),
        );
        progress.push(b"Updating files: 75% (3/4)", |message| {
            messages.push(message)
        });
        progress.finish(|message| messages.push(message));

        assert_eq!(
            messages,
            [
                "remote: Enumerating objects: 100, done.",
                "remote: Counting objects: 42% (42/100)",
                "Receiving objects: 100% (100/100)",
                "Updating files: 75% (3/4)",
            ]
        );
    }

    #[test]
    fn forwards_unrecognized_git_output() {
        let mut progress = GitCloneProgress::new();
        let mut messages = Vec::new();

        progress.push(b"Cloning into 'repository'...\n", |message| {
            messages.push(message)
        });
        progress.push(b"A future Git progress phase: 12%\r", |message| {
            messages.push(message)
        });

        assert_eq!(
            messages,
            [
                "Cloning into 'repository'...",
                "A future Git progress phase: 12%"
            ]
        );
    }

    #[test]
    fn extracts_the_last_failure_line() {
        assert_eq!(
            failure_message(
                b"Receiving objects: 42% (42/100)\rfatal: unable to access repository\n"
            ),
            "fatal: unable to access repository"
        );
        assert_eq!(failure_message(b""), "unknown error");
    }
}
