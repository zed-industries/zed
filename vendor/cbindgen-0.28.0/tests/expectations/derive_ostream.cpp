#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class C : uint32_t {
  X = 2,
  Y,
};

inline std::ostream& operator<<(std::ostream& stream, const C& instance) {
  switch (instance) {
    case C::X: stream << "X"; break;
    case C::Y: stream << "Y"; break;
  }
  return stream;
}

struct A {
  int32_t _0;

  friend std::ostream& operator<<(std::ostream& stream, const A& instance) {
    return stream << "{ " << "_0=" << instance._0 << " }";
  }
};

struct B {
  int32_t x;
  float y;

  friend std::ostream& operator<<(std::ostream& stream, const B& instance) {
    return stream << "{ " << "x=" << instance.x << ", "
                          << "y=" << instance.y << " }";
  }
};

struct D {
  uint8_t List;
  uintptr_t Of;
  B Things;

  friend std::ostream& operator<<(std::ostream& stream, const D& instance) {
    return stream << "{ " << "List=" << instance.List << ", "
                          << "Of=" << instance.Of << ", "
                          << "Things=" << instance.Things << " }";
  }
};

union F {
  enum class Tag : uint8_t {
    Foo,
    Bar,
    Baz,
  };

  friend std::ostream& operator<<(std::ostream& stream, const Tag& instance) {
    using Tag = F::Tag;
    switch (instance) {
      case Tag::Foo: stream << "Foo"; break;
      case Tag::Bar: stream << "Bar"; break;
      case Tag::Baz: stream << "Baz"; break;
    }
    return stream;
  }

  friend std::ostream& operator<<(std::ostream& stream, const F& instance) {
    using Tag = F::Tag;
    switch (instance.tag) {
      case Tag::Foo: stream << instance.foo; break;
      case Tag::Bar: stream << instance.bar; break;
      case Tag::Baz: stream << "Baz"; break;
    }
    return stream;
  }

  struct Foo_Body {
    Tag tag;
    int16_t _0;

    friend std::ostream& operator<<(std::ostream& stream, const Foo_Body& instance) {
      return stream << "{ " << "tag=" << instance.tag << ", "
                            << "_0=" << instance._0 << " }";
    }
  };

  struct Bar_Body {
    Tag tag;
    uint8_t x;
    int16_t y;

    friend std::ostream& operator<<(std::ostream& stream, const Bar_Body& instance) {
      return stream << "{ " << "tag=" << instance.tag << ", "
                            << "x=" << instance.x << ", "
                            << "y=" << instance.y << " }";
    }
  };

  struct {
    Tag tag;
  };
  Foo_Body foo;
  Bar_Body bar;
};

struct H {
  enum class Tag : uint8_t {
    Hello,
    There,
    Everyone,
  };

  friend std::ostream& operator<<(std::ostream& stream, const Tag& instance) {
    using Tag = H::Tag;
    switch (instance) {
      case Tag::Hello: stream << "Hello"; break;
      case Tag::There: stream << "There"; break;
      case Tag::Everyone: stream << "Everyone"; break;
    }
    return stream;
  }

  friend std::ostream& operator<<(std::ostream& stream, const H& instance) {
    using Tag = H::Tag;
    switch (instance.tag) {
      case Tag::Hello: stream << "Hello" << instance.hello; break;
      case Tag::There: stream << "There" << instance.there; break;
      case Tag::Everyone: stream << "Everyone"; break;
    }
    return stream;
  }

  struct Hello_Body {
    int16_t _0;

    friend std::ostream& operator<<(std::ostream& stream, const Hello_Body& instance) {
      return stream << "{ " << "_0=" << instance._0 << " }";
    }
  };

  struct There_Body {
    uint8_t x;
    int16_t y;

    friend std::ostream& operator<<(std::ostream& stream, const There_Body& instance) {
      return stream << "{ " << "x=" << instance.x << ", "
                            << "y=" << instance.y << " }";
    }
  };

  Tag tag;
  union {
    Hello_Body hello;
    There_Body there;
  };
};

struct I {
  enum class Tag : uint8_t {
    ThereAgain,
    SomethingElse,
  };

  struct ThereAgain_Body {
    uint8_t x;
    int16_t y;

    friend std::ostream& operator<<(std::ostream& stream, const ThereAgain_Body& instance) {
      return stream << "{ " << "x=" << instance.x << ", "
                            << "y=" << instance.y << " }";
    }
  };

  Tag tag;
  union {
    ThereAgain_Body there_again;
  };
};

extern "C" {

void root(A a, B b, C c, D d, F f, H h, I i);

}  // extern "C"
