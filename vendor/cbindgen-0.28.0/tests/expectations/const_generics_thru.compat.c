#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint8_t bytes[1];
} Inner_1;

typedef struct {
  Inner_1 inner;
} Outer_1;

typedef struct {
  uint8_t bytes[2];
} Inner_2;

typedef struct {
  Inner_2 inner;
} Outer_2;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

Outer_1 one(void);

Outer_2 two(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
