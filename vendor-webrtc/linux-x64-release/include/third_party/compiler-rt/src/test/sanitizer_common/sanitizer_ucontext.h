#ifdef __APPLE__
// ucontext.h is deprecated on macOS, so tests that include it may stop working
// someday. We define _XOPEN_SOURCE to keep using ucontext.h for now.
#ifdef _STRUCT_UCONTEXT
#error incomplete ucontext_t already defined, change #include order
#endif
#define _XOPEN_SOURCE 700
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#endif

#include <ucontext.h>
