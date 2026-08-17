#define MUST_USE_FUNC __attribute__((warn_unused_result))
#define MUST_USE_STRUCT __attribute__((warn_unused))
#define MUST_USE_ENUM /* nothing */


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct MUST_USE_STRUCT MaybeOwnedPtr {
  enum class MUST_USE_ENUM Tag : uint8_t {
    Owned,
    None,
  };

  struct Owned_Body {
    T *_0;
  };

  Tag tag;
  union {
    Owned_Body owned;
  };
};

template<typename T>
struct MUST_USE_STRUCT OwnedPtr {
  T *ptr;
};

extern "C" {

MUST_USE_FUNC MaybeOwnedPtr<int32_t> maybe_consume(OwnedPtr<int32_t> input);

}  // extern "C"
