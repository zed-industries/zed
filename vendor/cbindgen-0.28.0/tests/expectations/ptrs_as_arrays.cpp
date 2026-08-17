#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

void ptr_as_array(uint32_t n, const uint32_t arg[3], const uint64_t *v);

void ptr_as_array1(uint32_t n, const uint32_t arg[3], uint64_t v[4]);

void ptr_as_array2(uint32_t n, uint32_t arg[], const uint64_t v[]);

void ptr_as_array_wrong_syntax(uint32_t *arg, const uint32_t *v, const uint32_t*);

void ptr_as_array_unnamed(uint32_t*, const uint32_t*);

}  // extern "C"
