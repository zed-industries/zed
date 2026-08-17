#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  float a;
  uint32_t b;
  uint32_t c;
} ABC;
#define ABC_abc (ABC){ .a = 1.0, .b = 2, .c = 3 }
#define ABC_bac (ABC){ .a = 1.0, .b = 2, .c = 3 }
#define ABC_cba (ABC){ .a = 1.0, .b = 2, .c = 3 }

typedef struct {
  uint32_t b;
  float a;
  int32_t c;
} BAC;
#define BAC_abc (BAC){ .b = 1, .a = 2.0, .c = 3 }
#define BAC_bac (BAC){ .b = 1, .a = 2.0, .c = 3 }
#define BAC_cba (BAC){ .b = 1, .a = 2.0, .c = 3 }

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(ABC a1, BAC a2);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
