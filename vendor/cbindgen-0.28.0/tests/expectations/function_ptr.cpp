#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using MyCallback = void(*)(uintptr_t a, uintptr_t b);

using MyOtherCallback = void(*)(uintptr_t a,
                                uintptr_t lot,
                                uintptr_t of,
                                uintptr_t args,
                                uintptr_t and_then_some);

extern "C" {

void my_function(MyCallback a, MyOtherCallback b);

}  // extern "C"
