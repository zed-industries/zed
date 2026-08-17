#if 0
''' '
#endif

#ifdef __cplusplus
// These could be added as opaque types I guess.
template <typename T>
struct BuildHasherDefault;

struct DefaultHasher;
#endif

#if 0
' '''
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename K = void, typename V = void, typename Hasher = void>
struct HashMap;

template<typename T = void, typename E = void>
struct Result;

/// Fast hash map used internally.
template<typename K, typename V>
using FastHashMap = HashMap<K, V, BuildHasherDefault<DefaultHasher>>;

using Foo = FastHashMap<int32_t, int32_t>;

using Bar = Result<Foo>;

extern "C" {

void root(const Foo *a, const Bar *b);

}  // extern "C"
