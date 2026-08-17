#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Bar;

struct Foo {

};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const int32_t NUMBER;

extern struct Foo FOO;

extern const struct Bar BAR;

void root(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
