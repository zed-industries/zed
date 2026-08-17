#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define TITLE_SIZE 80

typedef int8_t CArrayString_TITLE_SIZE[TITLE_SIZE];

typedef int8_t CArrayString_40[40];

typedef struct {
  CArrayString_TITLE_SIZE title;
  CArrayString_40 author;
} Book;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Book *a);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
