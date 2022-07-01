use plugin::prelude::*;

#[export]
pub fn noop() {}

#[export]
pub fn constant() -> u32 {
    27
}

#[export]
pub fn identity(i: u32) -> u32 {
    i
}

#[export]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[export]
pub fn swap(a: u32, b: u32) -> (u32, u32) {
    (b, a)
}

#[export]
pub fn sort(mut list: Vec<u32>) -> Vec<u32> {
    list.sort();
    list
}

#[export]
pub fn print(string: String) {
    println!("to stdout: {}", string);
    eprintln!("to stderr: {}", string);
}

#[import]
fn mystery_number(input: u32) -> u32;

#[export]
pub fn and_back(secret: u32) -> u32 {
    mystery_number(secret)
}

#[import]
fn import_noop() -> ();

#[import]
fn import_identity(i: u32) -> u32;

#[import]
fn import_swap(a: u32, b: u32) -> (u32, u32);

#[export]
pub fn imports(x: u32) -> u32 {
    let a = import_identity(7);
    import_noop();
    let (b, c) = import_swap(a, x);
    assert_eq!(a, c);
    assert_eq!(x, b);
    a + b // should be 7 + x
}

#[import]
fn import_half(a: u32) -> u32;

#[export]
pub fn half_async(a: u32) -> u32 {
    import_half(a)
}

#[import]
fn command_async(command: String) -> Option<Vec<u8>>;

#[export]
pub fn echo_async(message: String) -> String {
    let command = dbg!(format!("echo {}", message));
    let result = command_async(command);
    dbg!(&result);
    let result = result.expect("Could not run command");
    String::from_utf8_lossy(&result).to_string()
}
