#include <errno.h>

int *errno_location() { return &errno; }
