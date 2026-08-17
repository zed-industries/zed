#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Enum {
  a,
  b,
};
typedef uint8_t Enum;

struct Struct {
  Enum field;
};

extern const Enum STATIC;

void fn(struct Struct arg);
