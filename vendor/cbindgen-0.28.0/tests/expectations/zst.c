#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  void *data;
  void *vtable;
} TraitObject;

void *root(const void *ptr, TraitObject t);
