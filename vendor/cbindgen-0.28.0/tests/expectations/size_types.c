#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

enum IE {
  IV,
};
typedef ptrdiff_t IE;

enum UE {
  UV,
};
typedef size_t UE;

typedef size_t Usize;

typedef ptrdiff_t Isize;

void root(Usize, Isize, UE, IE);
