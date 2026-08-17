//! An example of running a `Command` with a timeout.

use async_io::Timer;
use async_process::{Command, Stdio};
use futures_lite::{future, prelude::*};
use std::io;

fn main() -> io::Result<()> {
    async_io::block_on(async {
        // Spawn a a command of your choice.
        let mut child = Command::new("sleep")
            .arg("3")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // Run a future to drain the stdout of the child.
        // We can't use output() here because it would be cancelled along with the child when the timeout
        // expires.
        let mut stdout = String::new();
        let drain_stdout = {
            let buffer = &mut stdout;
            let mut stdout = child.stdout.take().unwrap();

            async move {
                stdout.read_to_string(buffer).await?;

                // Wait for the child to exit or the timeout.
                future::pending().await
            }
        };

        // Run a future to drain the stderr of the child.
        let mut stderr = String::new();
        let drain_stderr = {
            let buffer = &mut stderr;
            let mut stderr = child.stderr.take().unwrap();

            async move {
                stderr.read_to_string(buffer).await?;

                // Wait for the child to exit or the timeout.
                future::pending().await
            }
        };

        // Run a future that waits for the child to exit.
        let wait = async move {
            child.status().await?;

            // Child exited.
            io::Result::Ok(false)
        };

        // Run a future that times out after 1 second.
        let timeout_s = 1;
        let timeout = async move {
            Timer::after(std::time::Duration::from_secs(timeout_s)).await;

            // Timed out.
            Ok(true)
        };

        // Run the futures concurrently.
        // Note: For larger scale programs than this you should probably spawn each individual future on
        // a separate task in an executor.
        let timed_out = drain_stdout.or(drain_stderr).or(wait).or(timeout).await?;

        if timed_out {
            println!("The child timed out.");
        } else {
            println!("The child exited.");
        }

        println!("Stdout:\n{stdout}");
        println!("Stderr:\n{stderr}");

        Ok(())
    })
}
