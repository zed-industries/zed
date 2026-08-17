#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define TITLE_SIZE 80

typedef int8_t CArrayString_TITLE_SIZE[TITLE_SIZE];

typedef int8_t CArrayString_40[40];

struct Book {
  CArrayString_TITLE_SIZE title;
  CArrayString_40 author;
};

void root(struct Book *a);
