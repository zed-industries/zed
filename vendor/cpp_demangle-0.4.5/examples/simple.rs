use cpp_demangle::Symbol;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mangled = b"_ZN5space3fooEibc";

    let sym = Symbol::new(&mangled[..])?;
    let demangled = sym.to_string();

    println!("{}", demangled);

    assert_eq!(demangled, "space::foo(int, bool, char)");

    Ok(())
}
