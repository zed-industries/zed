#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef struct Option_____Opaque Option_____Opaque;

typedef struct {
  const Opaque *x;
  Opaque *y;
  void (*z)(void);
  void (**zz)(void);
} Foo;

typedef union {
  const Opaque *x;
  Opaque *y;
  void (*z)(void);
  void (**zz)(void);
} Bar;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const Opaque *a, Opaque *b, Foo c, Bar d, Option_____Opaque *e, void (*f)(const Opaque*));

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
