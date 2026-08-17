#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Enum
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  a,
  b,
};
#ifndef __cplusplus
typedef uint8_t Enum;
#endif // __cplusplus

typedef struct {
  Enum field;
} Struct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const Enum STATIC;

void fn(Struct arg);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
