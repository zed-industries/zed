use super::*;

/// Attempts to load a function from a given library.
///
/// This is a small wrapper around `LoadLibrary` and `GetProcAddress`.
///
/// # Safety
///
/// * Both the library and function names must be valid null-terminated strings.
pub unsafe fn delay_load<T>(library: crate::PCSTR, function: crate::PCSTR) -> Option<T> {
    let library = LoadLibraryExA(library.0, 0, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS);

    if library == 0 {
        return None;
    }

    let address = GetProcAddress(library, function.0);

    if address.is_some() {
        return Some(std::mem::transmute_copy(&address));
    }

    FreeLibrary(library);
    None
}
