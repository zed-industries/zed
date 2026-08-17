#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint8_t *buf;
  uintptr_t len;
} Parser_40__41;

typedef struct {
  uint8_t *buf;
  uintptr_t len;
} Parser_123__125;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void init_parens_parser(Parser_40__41 *p, uint8_t *buf, uintptr_t len);

void destroy_parens_parser(Parser_40__41 *p);

void init_braces_parser(Parser_123__125 *p, uint8_t *buf, uintptr_t len);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
