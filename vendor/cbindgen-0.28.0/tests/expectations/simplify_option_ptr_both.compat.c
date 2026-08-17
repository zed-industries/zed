#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef struct Option_____Opaque Option_____Opaque;

typedef struct Foo {
  const struct Opaque *x;
  struct Opaque *y;
  void (*z)(void);
  void (**zz)(void);
} Foo;

typedef union Bar {
  const struct Opaque *x;
  struct Opaque *y;
  void (*z)(void);
  void (**zz)(void);
} Bar;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const struct Opaque *a,
          struct Opaque *b,
          struct Foo c,
          union Bar d,
          struct Option_____Opaque *e,
          void (*f)(const struct Opaque*));

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
