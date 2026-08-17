#ifndef testing_helpers_h
#define testing_helpers_h

// This is a helper file to easily add static_asserts to C / C++ tests.

#ifndef __cplusplus
#include <assert.h>
#endif

#if defined(CBINDGEN_STYLE_TAG) && !defined(__cplusplus)
#define CBINDGEN_STRUCT(name) struct name
#define CBINDGEN_UNION(name) union name
#define CBINDGEN_ENUM(name) enum name
#else
#define CBINDGEN_STRUCT(name) name
#define CBINDGEN_UNION(name) name
#define CBINDGEN_ENUM(name) name
#endif

#endif
