use std::{
    cell::Cell,
    io::{self, Write as _},
    process::{Child, Command},
};

use crate::{
    parallel::{
        async_executor::{block_on, YieldOnce},
        job_token,
    },
    spawn, CargoOutput, Error, ErrorKind, StderrForwarder,
};

struct KillOnDrop(Child, StderrForwarder);

impl Drop for KillOnDrop {
    fn drop(&mut self) {
        let child = &mut self.0;

        child.kill().ok();
    }
}

fn cell_update<T, F>(cell: &Cell<T>, f: F)
where
    T: Default,
    F: FnOnce(T) -> T,
{
    let old = cell.take();
    let new = f(old);
    cell.set(new);
}

fn try_wait_on_child(
    cmd: &Command,
    child: &mut Child,
    mut stdout: impl io::Write,
    stderr_forwarder: &mut StderrForwarder,
) -> Result<Option<()>, Error> {
    stderr_forwarder.forward_available();

    match child.try_wait() {
        Ok(Some(status)) => {
            stderr_forwarder.forward_all();

            let _ = writeln!(stdout, "{}", status);

            if status.success() {
                Ok(Some(()))
            } else {
                Err(Error::new(
                    ErrorKind::ToolExecError,
                    format!("command did not execute successfully (status code {status}): {cmd:?}"),
                ))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => {
            stderr_forwarder.forward_all();
            Err(Error::new(
                ErrorKind::ToolExecError,
                format!("failed to wait on spawned child process `{cmd:?}`: {e}"),
            ))
        }
    }
}

pub(crate) fn run_commands_in_parallel(
    cargo_output: &CargoOutput,
    cmds: &mut dyn Iterator<Item = Result<Command, Error>>,
) -> Result<(), Error> {
    // Limit our parallelism globally with a jobserver.
    let mut tokens = job_token::ActiveJobTokenServer::new();

    // When compiling objects in parallel we do a few dirty tricks to speed
    // things up:
    //
    // * First is that we use the `jobserver` crate to limit the parallelism
    //   of this build script. The `jobserver` crate will use a jobserver
    //   configured by Cargo for build scripts to ensure that parallelism is
    //   coordinated across C compilations and Rust compilations. Before we
    //   compile anything we make sure to wait until we acquire a token.
    //
    //   Note that this jobserver is cached globally so we only used one per
    //   process and only worry about creating it once.
    //
    // * Next we use spawn the process to actually compile objects in
    //   parallel after we've acquired a token to perform some work
    //
    // With all that in mind we compile all objects in a loop here, after we
    // acquire the appropriate tokens, Once all objects have been compiled
    // we wait on all the processes and propagate the results of compilation.

    let pendings = Cell::new(Vec::<(Command, KillOnDrop, job_token::JobToken)>::new());
    let is_disconnected = Cell::new(false);
    let has_made_progress = Cell::new(false);

    let wait_future = async {
        let mut error = None;
        // Buffer the stdout
        let mut stdout = io::BufWriter::with_capacity(128, io::stdout());

        loop {
            // If the other end of the pipe is already disconnected, then we're not gonna get any new jobs,
            // so it doesn't make sense to reuse the tokens; in fact,
            // releasing them as soon as possible (once we know that the other end is disconnected) is beneficial.
            // Imagine that the last file built takes an hour to finish; in this scenario,
            // by not releasing the tokens before that last file is done we would effectively block other processes from
            // starting sooner - even though we only need one token for that last file, not N others that were acquired.

            let mut pendings_is_empty = false;

            cell_update(&pendings, |mut pendings| {
                // Try waiting on them.
                pendings.retain_mut(|(cmd, child, _token)| {
                    match try_wait_on_child(cmd, &mut child.0, &mut stdout, &mut child.1) {
                        Ok(Some(())) => {
                            // Task done, remove the entry
                            has_made_progress.set(true);
                            false
                        }
                        Ok(None) => true, // Task still not finished, keep the entry
                        Err(err) => {
                            // Task fail, remove the entry.
                            // Since we can only return one error, log the error to make
                            // sure users always see all the compilation failures.
                            has_made_progress.set(true);

                            if cargo_output.warnings {
                                let _ = writeln!(stdout, "cargo:warning={}", err);
                            }
                            error = Some(err);

                            false
                        }
                    }
                });
                pendings_is_empty = pendings.is_empty();
                pendings
            });

            if pendings_is_empty && is_disconnected.get() {
                break if let Some(err) = error {
                    Err(err)
                } else {
                    Ok(())
                };
            }

            YieldOnce::default().await;
        }
    };
    let spawn_future = async {
        for res in cmds {
            let mut cmd = res?;
            let token = tokens.acquire().await?;
            let mut child = spawn(&mut cmd, cargo_output)?;
            let mut stderr_forwarder = StderrForwarder::new(&mut child);
            stderr_forwarder.set_non_blocking()?;

            cell_update(&pendings, |mut pendings| {
                pendings.push((cmd, KillOnDrop(child, stderr_forwarder), token));
                pendings
            });

            has_made_progress.set(true);
        }
        is_disconnected.set(true);

        Ok::<_, Error>(())
    };

    block_on(wait_future, spawn_future, &has_made_progress)
}
