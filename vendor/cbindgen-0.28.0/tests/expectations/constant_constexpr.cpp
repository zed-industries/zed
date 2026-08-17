#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

static const int64_t CONSTANT_I64 = 216;

static const float CONSTANT_FLOAT32 = 312.292;

static const uint32_t DELIMITER = ':';

static const uint32_t LEFTCURLY = '{';

struct Foo {
  int32_t x;
  static const int64_t CONSTANT_I64_BODY;
};
inline const int64_t Foo::CONSTANT_I64_BODY = 216;

static const Foo SomeFoo = Foo{
  /* .x = */ 99
};
