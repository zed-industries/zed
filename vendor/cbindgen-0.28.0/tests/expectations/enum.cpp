#if 0
''' '
#endif

#ifdef __cplusplus
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

enum class A : uint64_t {
  a1 = 0,
  a2 = 2,
  a3,
  a4 = 5,
};

enum class B : uint32_t {
  b1 = 0,
  b2 = 2,
  b3,
  b4 = 5,
};

enum class C : uint16_t {
  c1 = 0,
  c2 = 2,
  c3,
  c4 = 5,
};

enum class D : uint8_t {
  d1 = 0,
  d2 = 2,
  d3,
  d4 = 5,
};

enum class E : uintptr_t {
  e1 = 0,
  e2 = 2,
  e3,
  e4 = 5,
};

enum class F : intptr_t {
  f1 = 0,
  f2 = 2,
  f3,
  f4 = 5,
};

enum class L {
  l1,
  l2,
  l3,
  l4,
};

enum class M : int8_t {
  m1 = -1,
  m2 = 0,
  m3 = 1,
};

enum N {
  n1,
  n2,
  n3,
  n4,
};

enum O : int8_t {
  o1,
  o2,
  o3,
  o4,
};

struct J;

struct K;

struct Opaque;

union G {
  enum class Tag : uint8_t {
    Foo,
    Bar,
    Baz,
  };

  struct Foo_Body {
    Tag tag;
    int16_t _0;
  };

  struct Bar_Body {
    Tag tag;
    uint8_t x;
    int16_t y;
  };

  struct {
    Tag tag;
  };
  Foo_Body foo;
  Bar_Body bar;
};

struct H {
  enum class Tag {
    H_Foo,
    H_Bar,
    H_Baz,
  };

  struct H_Foo_Body {
    int16_t _0;
  };

  struct H_Bar_Body {
    uint8_t x;
    int16_t y;
  };

  Tag tag;
  union {
    H_Foo_Body foo;
    H_Bar_Body bar;
  };
};

struct ExI {
  enum class Tag : uint8_t {
    ExI_Foo,
    ExI_Bar,
    ExI_Baz,
  };

  struct ExI_Foo_Body {
    int16_t _0;
  };

  struct ExI_Bar_Body {
    uint8_t x;
    int16_t y;
  };

  Tag tag;
  union {
    ExI_Foo_Body foo;
    ExI_Bar_Body bar;
  };
};

struct P {
  enum class Tag : uint8_t {
    P0,
    P1,
  };

  struct P0_Body {
    uint8_t _0;
  };

  struct P1_Body {
    uint8_t _0;
    uint8_t _1;
    uint8_t _2;
  };

  Tag tag;
  union {
    P0_Body p0;
    P1_Body p1;
  };
};

struct Q {
  enum class Tag {
    Ok,
    Err,
  };

  struct Ok_Body {
    Box<uint32_t> _0;
  };

  struct Err_Body {
    uint32_t _0;
  };

  Tag tag;
  union {
    Ok_Body ok;
    Err_Body err;
  };
};

struct R {
  enum class Tag {
    IRFoo,
    IRBar,
    IRBaz,
  };

  struct IRFoo_Body {
    int16_t _0;
  };

  struct IRBar_Body {
    uint8_t x;
    int16_t y;
  };

  Tag tag;
  union {
    IRFoo_Body IRFoo;
    IRBar_Body IRBar;
  };
};

extern "C" {

void root(Opaque *opaque,
          A a,
          B b,
          C c,
          D d,
          E e,
          F f,
          G g,
          H h,
          ExI i,
          J j,
          K k,
          L l,
          M m,
          N n,
          O o,
          P p,
          Q q,
          R r);

}  // extern "C"

#if 0
''' '
#endif

#include <stddef.h>
#include "testing-helpers.h"
static_assert(offsetof(CBINDGEN_STRUCT(P), tag) == 0, "unexpected offset for tag");
static_assert(offsetof(CBINDGEN_STRUCT(P), p0) == 1, "unexpected offset for p0");
static_assert(offsetof(CBINDGEN_STRUCT(P), p0) == 1, "unexpected offset for p1");
static_assert(sizeof(CBINDGEN_STRUCT(P)) == 4, "unexpected size for P");

#if 0
' '''
#endif
