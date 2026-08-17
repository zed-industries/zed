// Used as part of the ignorelist.c test
// tests if the "src:" ignorelist directive works
#include <stdio.h>

void typeViolationMultiFile(void *value) {
  printf("As long: %ld\n", *(long *)value);
}
