#[rustversion::nightly]
fn main() {
    println!("cargo:rustc-cfg=nightly_error_messages");
}

#[rustversion::not(nightly)]
fn main() {}
