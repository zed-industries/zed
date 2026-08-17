#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define EXPORT_ME_TOO 42

typedef struct {
  uint64_t val;
} ExportMe;

void export_me(ExportMe *val);
