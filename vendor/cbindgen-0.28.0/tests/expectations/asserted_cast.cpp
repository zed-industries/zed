#define MY_ASSERT(...) do { } while (0)
#define MY_ATTRS __attribute((noinline))


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct I;

struct H {
  enum class Tag : uint8_t {
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

  static H H_Foo(const int16_t &_0) {
    H result;
    ::new (&result.foo._0) (int16_t)(_0);
    result.tag = Tag::H_Foo;
    return result;
  }

  bool IsH_Foo() const {
    return tag == Tag::H_Foo;
  }

  const int16_t& AsH_Foo() const {
    MY_ASSERT(IsH_Foo());
    return foo._0;
  }

  MY_ATTRS int16_t& AsH_Foo() {
    MY_ASSERT(IsH_Foo());
    return foo._0;
  }

  static H H_Bar(const uint8_t &x,
                 const int16_t &y) {
    H result;
    ::new (&result.bar.x) (uint8_t)(x);
    ::new (&result.bar.y) (int16_t)(y);
    result.tag = Tag::H_Bar;
    return result;
  }

  bool IsH_Bar() const {
    return tag == Tag::H_Bar;
  }

  MY_ATTRS const H_Bar_Body& AsH_Bar() const {
    MY_ASSERT(IsH_Bar());
    return bar;
  }

  H_Bar_Body& AsH_Bar() {
    MY_ASSERT(IsH_Bar());
    return bar;
  }

  static H H_Baz() {
    H result;
    result.tag = Tag::H_Baz;
    return result;
  }

  MY_ATTRS bool IsH_Baz() const {
    return tag == Tag::H_Baz;
  }
};

struct J {
  enum class Tag : uint8_t {
    J_Foo,
    J_Bar,
    J_Baz,
  };

  struct J_Foo_Body {
    int16_t _0;
  };

  struct J_Bar_Body {
    uint8_t x;
    int16_t y;
  };

  Tag tag;
  union {
    J_Foo_Body foo;
    J_Bar_Body bar;
  };

  static J J_Foo(const int16_t &_0) {
    J result;
    ::new (&result.foo._0) (int16_t)(_0);
    result.tag = Tag::J_Foo;
    return result;
  }

  bool IsJ_Foo() const {
    return tag == Tag::J_Foo;
  }

  const int16_t& AsJ_Foo() const {
    MY_ASSERT(IsJ_Foo());
    return foo._0;
  }

  int16_t& AsJ_Foo() {
    MY_ASSERT(IsJ_Foo());
    return foo._0;
  }

  static J J_Bar(const uint8_t &x,
                 const int16_t &y) {
    J result;
    ::new (&result.bar.x) (uint8_t)(x);
    ::new (&result.bar.y) (int16_t)(y);
    result.tag = Tag::J_Bar;
    return result;
  }

  bool IsJ_Bar() const {
    return tag == Tag::J_Bar;
  }

  const J_Bar_Body& AsJ_Bar() const {
    MY_ASSERT(IsJ_Bar());
    return bar;
  }

  J_Bar_Body& AsJ_Bar() {
    MY_ASSERT(IsJ_Bar());
    return bar;
  }

  static J J_Baz() {
    J result;
    result.tag = Tag::J_Baz;
    return result;
  }

  bool IsJ_Baz() const {
    return tag == Tag::J_Baz;
  }
};

union K {
  enum class Tag : uint8_t {
    K_Foo,
    K_Bar,
    K_Baz,
  };

  struct K_Foo_Body {
    Tag tag;
    int16_t _0;
  };

  struct K_Bar_Body {
    Tag tag;
    uint8_t x;
    int16_t y;
  };

  struct {
    Tag tag;
  };
  K_Foo_Body foo;
  K_Bar_Body bar;

  static K K_Foo(const int16_t &_0) {
    K result;
    ::new (&result.foo._0) (int16_t)(_0);
    result.tag = Tag::K_Foo;
    return result;
  }

  bool IsK_Foo() const {
    return tag == Tag::K_Foo;
  }

  const int16_t& AsK_Foo() const {
    MY_ASSERT(IsK_Foo());
    return foo._0;
  }

  int16_t& AsK_Foo() {
    MY_ASSERT(IsK_Foo());
    return foo._0;
  }

  static K K_Bar(const uint8_t &x,
                 const int16_t &y) {
    K result;
    ::new (&result.bar.x) (uint8_t)(x);
    ::new (&result.bar.y) (int16_t)(y);
    result.tag = Tag::K_Bar;
    return result;
  }

  bool IsK_Bar() const {
    return tag == Tag::K_Bar;
  }

  const K_Bar_Body& AsK_Bar() const {
    MY_ASSERT(IsK_Bar());
    return bar;
  }

  K_Bar_Body& AsK_Bar() {
    MY_ASSERT(IsK_Bar());
    return bar;
  }

  static K K_Baz() {
    K result;
    result.tag = Tag::K_Baz;
    return result;
  }

  bool IsK_Baz() const {
    return tag == Tag::K_Baz;
  }
};

extern "C" {

void foo(H h, I i, J j, K k);

}  // extern "C"
