#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Foo {
  enum class Tag {
    A,
  };

  struct A_Body {
    float _0[20];
  };

  Tag tag;
  union {
    A_Body a;
  };

  static Foo A(const float (&_0)[20]) {
    Foo result;
    for (int i = 0; i < 20; i++) {
      ::new (&result.a._0[i]) (float)(_0[i]);
    }
    result.tag = Tag::A;
    return result;
  }

  bool IsA() const {
    return tag == Tag::A;
  }
};

extern "C" {

void root(Foo a);

}  // extern "C"
