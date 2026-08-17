use criterion::{criterion_group, Criterion};
use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
    str::FromStr,
    time::Duration,
};

fn create_command() -> Command {
    let mut command = Command::new("python3");
    command
        .arg("benches/benchmarks/external_process.py")
        .arg("10");
    command
}

fn python_fibonacci(c: &mut Criterion) {
    let has_python3 = Command::new("python3")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .is_ok();

    if has_python3 {
        let process = create_command()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Unable to start python process");

        let mut stdin = process
            .stdin
            .expect("Unable to get stdin for child process");
        let stdout = process
            .stdout
            .expect("Unable to get stdout for child process");
        let mut stdout = BufReader::new(stdout);
        c.bench_function("fibonacci-python", |b| {
            b.iter_custom(|iters| {
                writeln!(stdin, "{}", iters)
                    .expect("Unable to send iteration count to child process");
                let mut line = String::new();
                stdout
                    .read_line(&mut line)
                    .expect("Unable to read time from child process");
                let nanoseconds: u64 =
                    u64::from_str(line.trim()).expect("Unable to parse time from child process");
                Duration::from_nanos(nanoseconds)
            })
        });

        // Ensure that your child process terminates itself gracefully!
    }
}

criterion_group!(benches, python_fibonacci);
