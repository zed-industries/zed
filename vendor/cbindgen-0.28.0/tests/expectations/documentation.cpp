#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

/// Some docs.
extern const uint32_t FOO;

/// The root of all evil.
///
/// But at least it contains some more documentation as someone would expect
/// from a simple test case like this.
///
/// # Hint
///
/// Always ensure that everything is properly documented, even if you feel lazy.
/// **Sometimes** it is also helpful to include some markdown formatting.
///
/// ////////////////////////////////////////////////////////////////////////////
///
/// Attention:
///
///    Rust is going to trim all leading `/` symbols. If you want to use them as a
///    marker you need to add at least a single whitespace inbetween the tripple
///    slash doc-comment marker and the rest.
///
void root();

}  // extern "C"
