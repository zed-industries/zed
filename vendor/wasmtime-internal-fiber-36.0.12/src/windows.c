#include <windows.h>

#define CONCAT2(a, b) a##b
#define CONCAT(a, b) CONCAT2(a, b)
#define VERSIONED_SYMBOL(a) CONCAT(a, VERSIONED_SUFFIX)

LPVOID VERSIONED_SYMBOL(wasmtime_fiber_get_current)() {
  return GetCurrentFiber();
}
