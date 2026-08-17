#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using VaListFnPtr = int32_t(*)(int32_t count, ...);

using VaListFnPtr2 = int32_t(*)(int32_t count, ...);

template<typename T>
struct Interface {
  T fn1;
};

extern "C" {

int32_t va_list_test(int32_t count, ...);

int32_t va_list_test2(int32_t count, ...);

void va_list_fn_ptrs(int32_t (*fn1)(int32_t count, ...),
                     int32_t (*fn2)(int32_t count, ...),
                     VaListFnPtr fn3,
                     VaListFnPtr2 fn4,
                     Interface<int32_t(*)(int32_t count, ...)> fn5,
                     Interface<int32_t(*)(int32_t count, ...)> fn6);

}  // extern "C"
