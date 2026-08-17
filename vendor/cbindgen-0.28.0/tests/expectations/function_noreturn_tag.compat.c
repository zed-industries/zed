#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#ifndef NO_RETURN_ATTR
  #ifdef __GNUC__
    #define NO_RETURN_ATTR __attribute__ ((noreturn))
  #else // __GNUC__
    #define NO_RETURN_ATTR
  #endif // __GNUC__
#endif // NO_RETURN_ATTR


struct Example {
  void (*f)(uintptr_t, uintptr_t) NO_RETURN_ATTR;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void loop_forever(void) NO_RETURN_ATTR;

uint8_t normal_return(struct Example arg, void (*other)(uint8_t) NO_RETURN_ATTR);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
