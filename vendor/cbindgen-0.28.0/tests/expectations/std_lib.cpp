#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct Option;

template<typename T = void, typename E = void>
struct Result;

struct String;

template<typename T = void>
struct Vec;

extern "C" {

void root(const Vec<String> *a, const Option<int32_t> *b, const Result<int32_t, String> *c);

}  // extern "C"
