#if 0
DEF PLATFORM_UNIX = 0
DEF PLATFORM_WIN = 0
DEF X11 = 0
DEF M_32 = 0
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

#if (defined(PLATFORM_WIN) || defined(M_32))
enum class BarType : uint32_t {
  A,
  B,
  C,
};
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
enum class FooType : uint32_t {
  A,
  B,
  C,
};
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
struct FooHandle {
  FooType ty;
  int32_t x;
  float y;

  bool operator==(const FooHandle& other) const {
    return ty == other.ty &&
           x == other.x &&
           y == other.y;
  }
  bool operator!=(const FooHandle& other) const {
    return ty != other.ty ||
           x != other.x ||
           y != other.y;
  }
};
#endif

union C {
  enum class Tag : uint8_t {
    C1,
    C2,
#if defined(PLATFORM_WIN)
    C3,
#endif
#if defined(PLATFORM_UNIX)
    C5,
#endif
  };

#if defined(PLATFORM_UNIX)
  struct C5_Body {
    Tag tag;
    int32_t int_;

    bool operator==(const C5_Body& other) const {
      return int_ == other.int_;
    }
    bool operator!=(const C5_Body& other) const {
      return int_ != other.int_;
    }
  };
#endif

  struct {
    Tag tag;
  };
#if defined(PLATFORM_UNIX)
  C5_Body c5;
#endif

  static C C1() {
    C result;
    result.tag = Tag::C1;
    return result;
  }

  bool IsC1() const {
    return tag == Tag::C1;
  }

  static C C2() {
    C result;
    result.tag = Tag::C2;
    return result;
  }

  bool IsC2() const {
    return tag == Tag::C2;
  }

#if defined(PLATFORM_WIN)
  static C C3() {
    C result;
    result.tag = Tag::C3;
    return result;
  }

  bool IsC3() const {
    return tag == Tag::C3;
  }
#endif

#if defined(PLATFORM_UNIX)
  static C C5(const int32_t &int_) {
    C result;
    ::new (&result.c5.int_) (int32_t)(int_);
    result.tag = Tag::C5;
    return result;
  }

  bool IsC5() const {
    return tag == Tag::C5;
  }
#endif

  bool operator==(const C& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
#if defined(PLATFORM_UNIX)
      case Tag::C5: return c5 == other.c5;
#endif
      default: break;
    }
    return true;
  }

  bool operator!=(const C& other) const {
    return !(*this == other);
  }

  private:
  C() {

  }
  public:


  ~C() {
    switch (tag) {
#if defined(PLATFORM_UNIX)
      case Tag::C5: c5.~C5_Body(); break;
#endif
      default: break;
    }
  }

  C(const C& other)
   : tag(other.tag) {
    switch (tag) {
#if defined(PLATFORM_UNIX)
      case Tag::C5: ::new (&c5) (C5_Body)(other.c5); break;
#endif
      default: break;
    }
  }
  C& operator=(const C& other) {
    if (this != &other) {
      this->~C();
      new (this) C(other);
    }
    return *this;
  }
};

#if (defined(PLATFORM_WIN) || defined(M_32))
struct BarHandle {
  BarType ty;
  int32_t x;
  float y;

  bool operator==(const BarHandle& other) const {
    return ty == other.ty &&
           x == other.x &&
           y == other.y;
  }
  bool operator!=(const BarHandle& other) const {
    return ty != other.ty ||
           x != other.x ||
           y != other.y;
  }
};
#endif

struct ConditionalField {
#if defined(X11)
  int32_t field
#endif
  ;
};

struct Normal {
  int32_t x;
  float y;

  bool operator==(const Normal& other) const {
    return x == other.x &&
           y == other.y;
  }
  bool operator!=(const Normal& other) const {
    return x != other.x ||
           y != other.y;
  }
};

extern "C" {

#if defined(PLATFORM_WIN)
extern int32_t global_array_with_different_sizes[2];
#endif

#if defined(PLATFORM_UNIX)
extern int32_t global_array_with_different_sizes[1];
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
void root(FooHandle a, C c);
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
void root(BarHandle a, C c);
#endif

void cond(ConditionalField a);

#if defined(PLATFORM_WIN)
extern int32_t foo();
#endif

#if defined(PLATFORM_WIN)
extern void bar(Normal a);
#endif

}  // extern "C"
