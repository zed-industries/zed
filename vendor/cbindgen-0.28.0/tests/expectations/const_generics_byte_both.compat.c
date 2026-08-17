#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Parser_40__41 {
  uint8_t *buf;
  uintptr_t len;
} Parser_40__41;

typedef struct Parser_123__125 {
  uint8_t *buf;
  uintptr_t len;
} Parser_123__125;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void init_parens_parser(struct Parser_40__41 *p, uint8_t *buf, uintptr_t len);

void destroy_parens_parser(struct Parser_40__41 *p);

void init_braces_parser(struct Parser_123__125 *p, uint8_t *buf, uintptr_t len);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
