#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum Foo_Tag {
  A,
} Foo_Tag;

typedef struct Foo {
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

void root(struct Foo a);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
