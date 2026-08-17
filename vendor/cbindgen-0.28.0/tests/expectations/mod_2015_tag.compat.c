#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define EXPORT_ME_TOO 42

struct ExportMe {
  uint64_t val;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void export_me(struct ExportMe *val);

void from_really_nested_mod(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
