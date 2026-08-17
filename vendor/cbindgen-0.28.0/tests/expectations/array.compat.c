#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  A,
} Foo_Tag;

typedef struct {
  Foo_Tag tag;
  union {
    struct {
      float a[20];
    };
  };
} Foo;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Foo a);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
