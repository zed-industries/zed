#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum StyleOnlyThisShouldBeGenerated
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Foo,
  Bar,
};
#ifndef __cplusplus
typedef uint8_t StyleOnlyThisShouldBeGenerated;
#endif // __cplusplus
