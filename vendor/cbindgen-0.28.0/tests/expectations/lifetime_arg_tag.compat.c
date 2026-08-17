#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct A {
  const int32_t *data;
};

enum E_Tag {
  V,
  U,
};

struct E {
  enum E_Tag tag;
  union {
    struct {
      const uint8_t *u;
    };
  };
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A _a, struct E _e);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
