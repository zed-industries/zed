use std::str::FromStr;
use std::ffi::OsString;

use pico_args::*;

fn to_vec(args: &[&str]) -> Vec<OsString> {
    args.iter().map(|s| s.to_string().into()).collect()
}

#[test]
fn no_args() {
    let _ = Arguments::from_vec(to_vec(&[]));
}

#[test]
fn single_short_contains() {
    let mut args = Arguments::from_vec(to_vec(&["-V"]));
    assert!(args.contains("-V"));
}

#[test]
fn single_long_contains() {
    let mut args = Arguments::from_vec(to_vec(&["--version"]));
    assert!(args.contains("--version"));
}

#[test]
fn contains_two_01() {
    let mut args = Arguments::from_vec(to_vec(&["--version"]));
    assert!(args.contains(["-v", "--version"]));
}

#[test]
fn contains_two_02() {
    let mut args = Arguments::from_vec(to_vec(&["-v"]));
    assert!(args.contains(["-v", "--version"]));
}

#[test]
fn contains_two_03() {
    let mut args = Arguments::from_vec(to_vec(&["-v", "--version"]));
    assert!(args.contains(["-v", "--version"]));
}

#[test]
#[should_panic]
fn invalid_flag_01() {
    let mut args = Arguments::from_vec(to_vec(&["-v", "--version"]));
    assert!(args.contains("v"));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn invalid_flag_02() {
    let mut args = Arguments::from_vec(to_vec(&["-v", "--version"]));
    assert!(args.contains(["v", "--version"]));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn invalid_flag_03() {
    let mut args = Arguments::from_vec(to_vec(&["-v", "--version"]));
    assert!(args.contains(["-v", "-version"]));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn invalid_flag_04() {
    let mut args = Arguments::from_vec(to_vec(&["-v", "--version"]));
    assert!(args.contains(["-v", "version"]));
}


#[test]
fn option_01() {
    let mut args = Arguments::from_vec(to_vec(&["-w", "10"]));
    let value: Option<u32> = args.opt_value_from_str("-w").unwrap();
    assert_eq!(value.unwrap(), 10);
}

#[test]
fn option_02() {
    let mut args = Arguments::from_vec(to_vec(&["--width", "10"]));
    let value: Option<u32> = args.opt_value_from_str("--width").unwrap();
    assert_eq!(value.unwrap(), 10);
}

#[test]
fn option_03() {
    let mut args = Arguments::from_vec(to_vec(&["--name", "test"]));
    let value: Option<String> = args.opt_value_from_str("--name").unwrap();
    assert_eq!(value.unwrap(), "test");
}

#[cfg(feature = "eq-separator")]
#[test]
fn eq_option_01() {
    let mut args = Arguments::from_vec(to_vec(&["-w=10"]));
    let value: Option<u32> = args.opt_value_from_str("-w").unwrap();
    assert_eq!(value.unwrap(), 10);
}

#[cfg(feature = "eq-separator")]
#[test]
fn eq_option_02() {
    let mut args = Arguments::from_vec(to_vec(&["-w='10'"]));
    let value: Option<u32> = args.opt_value_from_str("-w").unwrap();
    assert_eq!(value.unwrap(), 10);
}

#[cfg(feature = "eq-separator")]
#[test]
fn eq_option_03() {
    let mut args = Arguments::from_vec(to_vec(&["-w=\"10\""]));
    let value: Option<u32> = args.opt_value_from_str("-w").unwrap();
    assert_eq!(value.unwrap(), 10);
}

#[cfg(feature = "eq-separator")]
#[test]
fn eq_option_04() {
    let mut args = Arguments::from_vec(to_vec(&["--width2=15", "--width=10"]));
    let value: Option<u32> = args.opt_value_from_str("--width").unwrap();
    assert_eq!(value.unwrap(), 10);
}

#[cfg(feature = "eq-separator")]
#[test]
fn eq_option_err_01() {
    let mut args = Arguments::from_vec(to_vec(&["-w="]));
    let value: Result<Option<u32>, Error> = args.opt_value_from_str("-w");
    assert_eq!(value.unwrap_err().to_string(),
               "the '-w' option doesn't have an associated value");
}

#[cfg(feature = "eq-separator")]
#[test]
fn eq_option_err_02() {
    let mut args = Arguments::from_vec(to_vec(&["-w='"]));
    let value: Result<Option<u32>, Error> = args.opt_value_from_str("-w");
    assert_eq!(value.unwrap_err().to_string(),
               "the '-w' option doesn't have an associated value");
}

#[cfg(feature = "eq-separator")]
#[test]
fn eq_option_err_03() {
    let mut args = Arguments::from_vec(to_vec(&["-w=''"]));
    let value: Result<Option<u32>, Error> = args.opt_value_from_str("-w");
    assert_eq!(value.unwrap_err().to_string(),
               "the '-w' option doesn't have an associated value");
}

#[cfg(feature = "eq-separator")]
#[test]
fn eq_option_err_04() {
    let mut args = Arguments::from_vec(to_vec(&["-w='\""]));
    let value: Result<Option<u32>, Error> = args.opt_value_from_str("-w");
    assert_eq!(value.unwrap_err().to_string(),
               "the '-w' option doesn't have an associated value");
}

#[cfg(feature = "eq-separator")]
#[test]
fn eq_option_err_05() {
    let mut args = Arguments::from_vec(to_vec(&["-w='10\""]));
    let value: Result<Option<u32>, Error> = args.opt_value_from_str("-w");
    assert_eq!(value.unwrap_err().to_string(),
               "the '-w' option doesn't have an associated value");
}

#[cfg(all(feature = "eq-separator", not(feature = "short-space-opt")))]
#[test]
fn eq_option_err_06() {
    let mut args = Arguments::from_vec(to_vec(&["-w-10"]));
    let value: Result<Option<u32>, Error> = args.opt_value_from_str("-w");
    assert_eq!(value.unwrap(), None);
}

#[cfg(feature = "eq-separator")]
#[test]
fn eq_option_err_07() {
    let mut args = Arguments::from_vec(to_vec(&["-w=a"]));
    let value: Result<Option<u32>, Error> = args.opt_value_from_str("-w");
    assert_eq!(value.unwrap_err().to_string(),
               "failed to parse 'a': invalid digit found in string");
}

#[cfg(not(any(feature = "eq-separator", feature = "short-space-opt")))]
#[test]
fn no_eq_separator_01() {
    let mut args = Arguments::from_vec(to_vec(&["-w=a"]));
    let value: Result<Option<u32>, Error> = args.opt_value_from_str("-w");
    assert_eq!(value.unwrap(), None);
}

#[cfg(feature = "combined-flags")]
#[test]
fn combined_flags_01() {
    let mut args = Arguments::from_vec(to_vec(&["-ab"]));
    assert!(args.contains("-b"));
    assert!(args.contains("-a"));
}

#[cfg(feature = "combined-flags")]
#[test]
fn combined_flags_repeated_01() {
    let mut args = Arguments::from_vec(to_vec(&["-aa"]));
    assert!(args.contains("-a"));
    assert!(args.contains("-a"));
    assert!(!args.contains("-a"));
}

#[cfg(feature = "combined-flags")]
#[test]
fn combined_flags_repeated_02() {
    let mut args = Arguments::from_vec(to_vec(&["-aaa", "-a"]));
    assert!(args.contains("-aaa"));
    assert!(args.contains("-a"));
}

#[cfg(feature = "combined-flags")]
#[test]
fn combined_flags_leftover() {
    let mut args = Arguments::from_vec(to_vec(&["-ab"]));
    assert!(args.contains("-a"));
    assert_eq!(args.finish(), vec![OsString::from("-b")]);
}

#[test]
fn long_flag_with_character_from_short_flag() {
    let mut args = Arguments::from_vec(to_vec(&["--version"]));
    assert!(!args.contains("-s"));
    assert!(args.contains("--version"));
}

#[cfg(feature = "combined-flags")]
#[test]
fn combined_long_flag_with_character_from_short_flag() {
    let mut args = Arguments::from_vec(to_vec(&["--version"]));
    assert!(!args.contains("-s"));
    assert!(args.contains("--version"));
}

#[cfg(feature = "short-space-opt")]
#[test]
fn space_option_01() {
    let mut args = Arguments::from_vec(to_vec(&["-w10"]));
    let value: Option<u32> = args.opt_value_from_str("-w").unwrap();
    assert_eq!(value.unwrap(), 10);
}

#[cfg(feature = "short-space-opt")]
#[test]
fn space_option_02() {
    let mut args = Arguments::from_vec(to_vec(&["-w--width"]));
    let value: Option<String> = args.opt_value_from_str(["-w", "--width"]).unwrap();
    assert_eq!(value.unwrap(), "--width");
}

#[cfg(feature = "short-space-opt")]
#[test]
fn space_option_03() {
    let mut args = Arguments::from_vec(to_vec(&["-w'10'"]));
    let value: Option<u32> = args.opt_value_from_str(["-w", "--width"]).unwrap();
    assert_eq!(value.unwrap(), 10);
}

#[cfg(feature = "short-space-opt")]
#[test]
fn space_option_04() {
    let mut args = Arguments::from_vec(to_vec(&["-w\"10\""]));
    let value: Option<u32> = args.opt_value_from_str(["-w", "--width"]).unwrap();
    assert_eq!(value.unwrap(), 10);
}

#[cfg(all(feature = "short-space-opt", not(feature = "eq-separator")))]
#[test]
fn space_not_eq_option_err_01() {
    let mut args = Arguments::from_vec(to_vec(&["-w=10"]));
    let value: Result<Option<String>, Error> = args.opt_value_from_str("-w");
    assert_eq!(value.unwrap_err().to_string(),
               "the \'-w\' option doesn\'t have an associated value");
}

#[cfg(all(feature = "short-space-opt", not(feature = "eq-separator")))]
#[test]
fn space_not_eq_option_err_02() {
    let mut args = Arguments::from_vec(to_vec(&["--width=10"]));
    let value: Option<String> = args.opt_value_from_str("--width").unwrap();
    assert_eq!(value, None);
}

#[cfg(all(feature = "short-space-opt", feature = "eq-separator"))]
#[test]
fn space_eq_option_01() {
    let mut args = Arguments::from_vec(to_vec(&["-w=10"]));
    let value: Option<String> = args.opt_value_from_str("-w").unwrap();
    assert_eq!(value.unwrap(), "10");
}

#[cfg(all(feature = "short-space-opt", feature = "eq-separator"))]
#[test]
fn space_eq_option_02() {
    let mut args = Arguments::from_vec(to_vec(&["-w'=10'"]));
    let value: Option<String> = args.opt_value_from_str("-w").unwrap();
    assert_eq!(value.unwrap(), "=10");
}

#[cfg(feature = "short-space-opt")]
#[test]
fn space_option_err_01() {
    let mut args = Arguments::from_vec(to_vec(&["--width10"]));
    let value: Option<String> = args.opt_value_from_str("--width").unwrap();
    assert_eq!(value, None);
}

#[cfg(feature = "short-space-opt")]
#[test]
fn space_option_err_02() {
    let mut args = Arguments::from_vec(to_vec(&["-w'10"]));
    let value: Result<Option<u32>, Error> = args.opt_value_from_str("-w");
    assert_eq!(value.unwrap_err().to_string(), "the \'-w\' option doesn\'t have an associated value");
}

#[test]
fn duplicated_options_01() {
    let mut args = Arguments::from_vec(to_vec(&["--name", "test1", "--name", "test2"]));
    let value1: Option<String> = args.opt_value_from_str("--name").unwrap();
    let value2: Option<String> = args.opt_value_from_str("--name").unwrap();
    assert_eq!(value1.unwrap(), "test1");
    assert_eq!(value2.unwrap(), "test2");
}

#[test]
fn option_from_os_str_01() {
    use std::path::PathBuf;

    fn parse_path(s: &std::ffi::OsStr) -> Result<PathBuf, &'static str> {
        Ok(s.into())
    }

    let mut args = Arguments::from_vec(to_vec(&["--input", "text.txt"]));
    let value: Result<Option<PathBuf>, Error> = args.opt_value_from_os_str("--input", parse_path);
    assert_eq!(value.unwrap().unwrap().display().to_string(), "text.txt");
}

#[test]
fn missing_option_value_01() {
    let mut args = Arguments::from_vec(to_vec(&["--value"]));
    let value: Result<Option<u32>, Error> = args.opt_value_from_str("--value");
    assert_eq!(value.unwrap_err().to_string(),
               "the '--value' option doesn't have an associated value");
}

#[test]
fn missing_option_value_02() {
    let mut args = Arguments::from_vec(to_vec(&["--value"]));
    let value: Result<Option<u32>, Error> = args.opt_value_from_str("--value");
    assert!(value.is_err()); // ignore error
    // the `--value` flag should not be removed by the previous command
    assert_eq!(args.finish(), vec![OsString::from("--value")]);
}

#[test]
fn missing_option_value_03() {
    let mut args = Arguments::from_vec(to_vec(&["--value", "q"]));
    let value: Result<Option<u32>, Error> = args.opt_value_from_str("--value");
    assert!(value.is_err()); // ignore error
    // the `--value` flag should not be removed by the previous command
    assert_eq!(args.finish(), vec![OsString::from("--value"), OsString::from("q")]);
}

#[test]
fn multiple_options_01() {
    let mut args = Arguments::from_vec(to_vec(&["-w", "10", "-w", "20"]));
    let value: Vec<u32> = args.values_from_str("-w").unwrap();
    assert_eq!(value, &[10, 20]);
}

#[test]
fn multiple_options_02() {
    // No values is not an error.
    let mut args = Arguments::from_vec(to_vec(&[]));
    let value: Vec<u32> = args.values_from_str("-w").unwrap();
    assert_eq!(value, &[]);
}

#[test]
fn multiple_options_03() {
    // Argument can be split.
    let mut args = Arguments::from_vec(to_vec(&["-w", "10", "--other", "-w", "20"]));
    let value: Vec<u32> = args.values_from_str("-w").unwrap();
    assert_eq!(value, &[10, 20]);
}

#[test]
fn free_from_str_01() {
    let mut args = Arguments::from_vec(to_vec(&["5"]));
    let value: u32 = args.free_from_str().unwrap();
    assert_eq!(value, 5);
}

#[test]
fn opt_free_from_fn_01() {
    let mut args = Arguments::from_vec(to_vec(&["5"]));
    assert_eq!(args.opt_free_from_fn(u32::from_str).unwrap(), Some(5));
}

#[test]
fn opt_free_from_fn_02() {
    let mut args = Arguments::from_vec(to_vec(&[]));
    assert_eq!(args.opt_free_from_fn(u32::from_str).unwrap(), None);
}

#[test]
fn opt_free_from_fn_03() {
    let mut args = Arguments::from_vec(to_vec(&["-h"]));
    assert_eq!(args.opt_free_from_fn(u32::from_str).unwrap_err().to_string(),
               "failed to parse '-h': invalid digit found in string");
}

#[test]
fn opt_free_from_fn_04() {
    let mut args = Arguments::from_vec(to_vec(&["a"]));
    assert_eq!(args.opt_free_from_fn(u32::from_str).unwrap_err().to_string(),
               "failed to parse 'a': invalid digit found in string");
}

#[test]
fn opt_free_from_fn_05() {
    let mut args = Arguments::from_vec(to_vec(&["-5"]));
    assert_eq!(args.opt_free_from_fn(i32::from_str).unwrap(), Some(-5));
}

#[test]
fn opt_free_from_fn_06() {
    let mut args = Arguments::from_vec(to_vec(&["-3.14"]));
    assert_eq!(args.opt_free_from_fn(f32::from_str).unwrap(), Some(-3.14f32));
}

#[test]
fn opt_free_from_str_01() {
    let mut args = Arguments::from_vec(to_vec(&["5"]));
    let value: Result<Option<u32>, Error> = args.opt_free_from_str();
    assert_eq!(value.unwrap(), Some(5));
}

#[test]
fn required_option_01() {
    let mut args = Arguments::from_vec(to_vec(&["--width", "10"]));
    let value: u32 = args.value_from_str("--width").unwrap();
    assert_eq!(value, 10);
}

#[test]
fn missing_required_option_01() {
    let mut args = Arguments::from_vec(to_vec(&[]));
    let value: Result<u32, Error> = args.value_from_str("-w");
    assert_eq!(value.unwrap_err().to_string(),
               "the '-w' option must be set");
}

#[test]
fn missing_required_option_02() {
    let mut args = Arguments::from_vec(to_vec(&[]));
    let value: Result<u32, Error> = args.value_from_str("--width");
    assert_eq!(value.unwrap_err().to_string(),
               "the '--width' option must be set");
}

#[test]
fn missing_required_option_03() {
    let mut args = Arguments::from_vec(to_vec(&[]));
    let value: Result<u32, Error> = args.value_from_str(["-w", "--width"]);
    assert_eq!(value.unwrap_err().to_string(),
               "the '-w/--width' option must be set");
}

#[test]
fn subcommand() {
    let mut args = Arguments::from_vec(to_vec(&["toolchain", "install", "--help"]));

    let cmd = args.subcommand().unwrap();
    assert_eq!(cmd, Some("toolchain".to_string()));

    let cmd = args.subcommand().unwrap();
    assert_eq!(cmd, Some("install".to_string()));

    let cmd = args.subcommand().unwrap();
    assert_eq!(cmd, None);
}
