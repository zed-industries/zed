#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using A = void(*)();

using B = void(*)();

using C = bool(*)(int32_t, int32_t);

using D = bool(*(*)(int32_t))(float);

using E = const int32_t(*(*)())[16];

using F = const int32_t*;

using G = const int32_t*const *;

using H = int32_t*const *;

using I = const int32_t(*)[16];

using J = double(**)(float);

using K = int32_t[16];

using L = const int32_t*[16];

using M = bool(*[16])(int32_t, int32_t);

using N = void(*[16])(int32_t, int32_t);

using P = void(*)(int32_t named1st, bool, bool named3rd, int32_t _);

extern "C" {

void (*O())();

void root(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l, M m, N n, P p);

}  // extern "C"
