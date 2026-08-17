#ifdef PKG_CONFIG

/* Just use installed headers */
#include <zstd.h>
#ifdef ZSTD_RUST_BINDINGS_EXPERIMENTAL
#include <zstd_errors.h>
#endif  // #ifdef ZSTD_RUST_BINDINGS_EXPERIMENTAL

#else // #ifdef PKG_CONFIG

#include "zstd/lib/zstd.h"
#ifdef ZSTD_RUST_BINDINGS_EXPERIMENTAL
#include "zstd/lib/zstd_errors.h"
#endif // #ifdef ZSTD_RUST_BINDINGS_EXPERIMENTAL

#endif // #ifdef PKG_CONFIG


/* This file is used to generate bindings for both headers.
 * Check update_bindings.sh to see how to use it.
 * Or use the `bindgen` feature, which will create the bindings automatically. */
