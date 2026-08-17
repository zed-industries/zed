#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct TraitObject {
  void *data;
  void *vtable;
} TraitObject;

void *root(const void *ptr, struct TraitObject t);
