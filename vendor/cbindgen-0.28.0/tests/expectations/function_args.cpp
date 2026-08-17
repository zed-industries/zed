#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

void unnamed(const uint64_t*);

void pointer_test(const uint64_t *a);

void print_from_rust();

}  // extern "C"
