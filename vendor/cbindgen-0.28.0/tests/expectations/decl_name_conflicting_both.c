#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum BindingType {
  Buffer = 0,
  NotBuffer = 1,
};
typedef uint32_t BindingType;

typedef struct BindGroupLayoutEntry {
  BindingType ty;
} BindGroupLayoutEntry;

void root(struct BindGroupLayoutEntry entry);
