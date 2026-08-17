#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const int32_t FOO = 10;

constexpr static const uint32_t DELIMITER = ':';

constexpr static const uint32_t LEFTCURLY = '{';

constexpr static const uint32_t QUOTE = '\'';

constexpr static const uint32_t TAB = '\t';

constexpr static const uint32_t NEWLINE = '\n';

constexpr static const uint32_t HEART = U'\U00002764';

constexpr static const uint32_t EQUID = U'\U00010083';

constexpr static const float ZOM = 3.14;

/// A single-line doc comment.
constexpr static const int8_t POS_ONE = 1;

/// A
/// multi-line
/// doc
/// comment.
constexpr static const int8_t NEG_ONE = -1;

constexpr static const int64_t SHIFT = 3;

constexpr static const int64_t XBOOL = 1;

constexpr static const int64_t XFALSE = ((0 << SHIFT) | XBOOL);

constexpr static const int64_t XTRUE = (1 << (SHIFT | XBOOL));

constexpr static const uint8_t CAST = (uint8_t)'A';

constexpr static const uint32_t DOUBLE_CAST = (uint32_t)(float)1;

struct Foo {
  int32_t x[FOO];
};

extern "C" {

void root(Foo x);

}  // extern "C"
