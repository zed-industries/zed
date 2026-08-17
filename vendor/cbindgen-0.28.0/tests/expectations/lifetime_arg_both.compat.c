#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A {
  const int32_t *data;
} A;

typedef enum E_Tag {
  V,
  U,
} E_Tag;

typedef struct E {
  E_Tag tag;
  union {
    struct {
      const uint8_t *u;
    };
  };
} E;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A _a, struct E _e);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
