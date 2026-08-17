#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum BindingType
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  Buffer = 0,
  NotBuffer = 1,
};
#ifndef __cplusplus
typedef uint32_t BindingType;
#endif // __cplusplus

typedef struct {
  BindingType ty;
} BindGroupLayoutEntry;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(BindGroupLayoutEntry entry);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
