extern crate cc;

fn main() {
    cc::Build::new()
        .file("extern/exception.m")
        .compile("libexception.a");
}
