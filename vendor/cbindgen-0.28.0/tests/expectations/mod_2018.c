#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define EXPORT_ME_TOO 42

typedef struct {
  uint64_t val;
} ExportMe;

typedef struct {
  uint64_t val;
} ExportMe2;

void export_me(ExportMe *val);

void export_me_2(ExportMe2*);

void from_really_nested_mod(void);
