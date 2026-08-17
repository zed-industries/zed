#if 0
''' '
#endif

#ifdef __cplusplus
template <typename T>
using Pin = T;
template <typename T>
using Box = T*;
#endif

#if 0
' '''
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct PinTest {
  Pin<Box<int32_t>> pinned_box;
  Pin<int32_t*> pinned_ref;
};

extern "C" {

void root(Pin<int32_t*> s, PinTest p);

}  // extern "C"
