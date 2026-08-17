#if 0
''' '
#endif

#ifdef __cplusplus
template <typename T>
using ManuallyDrop = T;
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

struct Point {
  int32_t x;
  int32_t y;
};

using Foo = NotReprC<ManuallyDrop<Point>>;

struct MyStruct {
  ManuallyDrop<Point> point;
};

extern "C" {

void root(const Foo *a, const MyStruct *with_manual_drop);

void take(ManuallyDrop<Point> with_manual_drop);

}  // extern "C"
