//! Helpers for demangling function names.

/// Demangles a single function name into a user-readable form.
///
/// Currently supported: Rust/C/C++ function names.
pub fn demangle_function_name(writer: &mut impl core::fmt::Write, name: &str) -> core::fmt::Result {
    #[cfg(feature = "demangle")]
    if let Ok(demangled) = rustc_demangle::try_demangle(name) {
        return write!(writer, "{demangled}");
    } else if let Ok(symbol) = cpp_demangle::Symbol::new(name) {
        let options = cpp_demangle::DemangleOptions::default();
        if let Ok(demangled) = symbol.demangle(&options) {
            return write!(writer, "{demangled}");
        }
    }

    write!(writer, "{name}")
}

/// Demangles a function name if it's provided, or returns a unified representation based on the
/// function index otherwise.
pub fn demangle_function_name_or_index(
    writer: &mut impl core::fmt::Write,
    name: Option<&str>,
    func_id: usize,
) -> core::fmt::Result {
    match name {
        Some(name) => demangle_function_name(writer, name),
        None => write!(writer, "<wasm function {func_id}>"),
    }
}
