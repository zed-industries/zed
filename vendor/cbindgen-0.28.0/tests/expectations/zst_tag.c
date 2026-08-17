#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct TraitObject {
  void *data;
  void *vtable;
};

void *root(const void *ptr, struct TraitObject t);
