#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct A {
  int32_t namespace_;
  float float_;
};

struct B {
  int32_t namespace_;
  float float_;
};

struct C {
  enum class Tag : uint8_t {
    D,
  };

  struct D_Body {
    int32_t namespace_;
    float float_;
  };

  Tag tag;
  union {
    D_Body d;
  };
};

struct E {
  enum class Tag : uint8_t {
    Double,
    Float,
  };

  struct Double_Body {
    double _0;
  };

  struct Float_Body {
    float _0;
  };

  Tag tag;
  union {
    Double_Body double_;
    Float_Body float_;
  };
};

struct F {
  enum class Tag : uint8_t {
    double_,
    float_,
  };

  struct double_Body {
    double _0;
  };

  struct float_Body {
    float _0;
  };

  Tag tag;
  union {
    double_Body double_;
    float_Body float_;
  };
};

extern "C" {

void root(A a, B b, C c, E e, F f, int32_t namespace_, float float_);

}  // extern "C"
