#ifndef RUST_DEMANGLE_H_
#define RUST_DEMANGLE_H_

#ifdef __cplusplus
extern "C" {
#endif

// Demangles symbol given in `mangled` argument into `out` buffer
//
// Returns 0 if `mangled` is not Rust symbol or if `out` buffer is too small
// Returns 1 otherwise
int rustc_demangle(const char *mangled, char *out, size_t out_size);

#ifdef __cplusplus
}
#endif

#endif // RUSTC_DEMANGLE_H_
