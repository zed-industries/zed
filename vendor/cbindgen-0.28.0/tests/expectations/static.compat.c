#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Bar Bar;

typedef struct {

} Foo;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const int32_t NUMBER;

extern Foo FOO;

extern const Bar BAR;

void root(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
