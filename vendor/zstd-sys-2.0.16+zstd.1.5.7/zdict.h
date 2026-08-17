#ifdef PKG_CONFIG

/* Just use installed headers */
#include <zdict.h>
// Don't use experimental features like zstdmt

#else // #ifdef PKG_CONFIG

#include "zstd/lib/zdict.h"

#endif // #ifdef PKG_CONFIG


/* This file is used to generate bindings for both headers.
 * Check update_bindings.sh to see how to use it.
 * Or use the `bindgen` feature, which will create the bindings automatically. */

