#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
namespace constants {
namespace test {
#endif  // __cplusplus

#define FOO 10

#define ZOM 3.14

struct Foo {
  int32_t x[FOO];
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Foo x);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#ifdef __cplusplus
}  // namespace test
}  // namespace constants
#endif  // __cplusplus
