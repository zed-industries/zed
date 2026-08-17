#if 0
''' '
#endif

#ifdef __cplusplus
template <typename T>
using MaybeUninit = T;
#endif

#if 0
' '''
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct NotReprC;

using Foo = NotReprC<MaybeUninit<const int32_t*>>;

struct MyStruct {
  MaybeUninit<const int32_t*> number;
};

extern "C" {

void root(const Foo *a, const MyStruct *with_maybe_uninit);

}  // extern "C"
