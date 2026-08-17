use std::fmt::Write;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=isle_examples");

    let out_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR").expect("The OUT_DIR environment variable must be set"),
    );

    let mut out = String::new();

    emit_tests(&mut out, "isle_examples/pass", "run_pass");
    emit_tests(&mut out, "isle_examples/fail", "run_fail");
    emit_tests(&mut out, "isle_examples/link", "run_link");
    emit_tests(&mut out, "isle_examples/run", "run_run");

    emit_tests(&mut out, "isle_examples/pass", "run_print");
    emit_tests(&mut out, "isle_examples/link", "run_print");
    emit_tests(&mut out, "isle_examples/run", "run_print");

    let output = out_dir.join("isle_tests.rs");
    std::fs::write(output, out).unwrap();
}

fn emit_tests(out: &mut String, dir_name: &str, runner_func: &str) {
    for test_file in std::fs::read_dir(dir_name).unwrap() {
        let test_file = test_file.unwrap().file_name().into_string().unwrap();
        if !test_file.ends_with(".isle") {
            continue;
        }
        let test_file_base = test_file.replace(".isle", "");

        writeln!(out, "#[test]").unwrap();
        writeln!(out, "fn test_{runner_func}_{test_file_base}() {{").unwrap();
        writeln!(out, "    {runner_func}(\"{dir_name}/{test_file}\");").unwrap();
        writeln!(out, "}}").unwrap();
    }
}
