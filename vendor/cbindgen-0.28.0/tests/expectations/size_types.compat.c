#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

enum IE
#ifdef __cplusplus
  : ptrdiff_t
#endif // __cplusplus
 {
  IV,
};
#ifndef __cplusplus
typedef ptrdiff_t IE;
#endif // __cplusplus

enum UE
#ifdef __cplusplus
  : size_t
#endif // __cplusplus
 {
  UV,
};
#ifndef __cplusplus
typedef size_t UE;
#endif // __cplusplus

typedef size_t Usize;

typedef ptrdiff_t Isize;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Usize, Isize, UE, IE);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
