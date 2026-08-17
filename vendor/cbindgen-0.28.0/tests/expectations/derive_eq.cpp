#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Foo {
  bool a;
  int32_t b;

  bool operator==(const Foo& aOther) const {
    return a == aOther.a &&
           b == aOther.b;
  }
  bool operator!=(const Foo& aOther) const {
    return a != aOther.a ||
           b != aOther.b;
  }
};

union Bar {
  enum class Tag : uint8_t {
    Baz,
    Bazz,
    FooNamed,
    FooParen,
  };

  struct Bazz_Body {
    Tag tag;
    Foo named;

    bool operator==(const Bazz_Body& aOther) const {
      return named == aOther.named;
    }
    bool operator!=(const Bazz_Body& aOther) const {
      return named != aOther.named;
    }
  };

  struct FooNamed_Body {
    Tag tag;
    int32_t different;
    uint32_t fields;

    bool operator==(const FooNamed_Body& aOther) const {
      return different == aOther.different &&
             fields == aOther.fields;
    }
    bool operator!=(const FooNamed_Body& aOther) const {
      return different != aOther.different ||
             fields != aOther.fields;
    }
  };

  struct FooParen_Body {
    Tag tag;
    int32_t _0;
    Foo _1;

    bool operator==(const FooParen_Body& aOther) const {
      return _0 == aOther._0 &&
             _1 == aOther._1;
    }
    bool operator!=(const FooParen_Body& aOther) const {
      return _0 != aOther._0 ||
             _1 != aOther._1;
    }
  };

  struct {
    Tag tag;
  };
  Bazz_Body bazz;
  FooNamed_Body foo_named;
  FooParen_Body foo_paren;

  bool operator==(const Bar& aOther) const {
    if (tag != aOther.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Bazz: return bazz == aOther.bazz;
      case Tag::FooNamed: return foo_named == aOther.foo_named;
      case Tag::FooParen: return foo_paren == aOther.foo_paren;
      default: break;
    }
    return true;
  }

  bool operator!=(const Bar& aOther) const {
    return !(*this == aOther);
  }
};

extern "C" {

Foo root(Bar aBar);

}  // extern "C"
