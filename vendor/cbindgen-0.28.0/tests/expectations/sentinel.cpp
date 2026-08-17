#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class A : uint8_t {
  A_A1,
  A_A2,
  A_A3,
  /// Must be last for serialization purposes
  A_Sentinel,
};

enum class B : uint8_t {
  B_B1,
  B_B2,
  B_B3,
  /// Must be last for serialization purposes
  B_Sentinel,
};

union C {
  enum class Tag : uint8_t {
    C_C1,
    C_C2,
    C_C3,
    /// Must be last for serialization purposes
    C_Sentinel,
  };

  struct C_C1_Body {
    Tag tag;
    uint32_t a;
  };

  struct C_C2_Body {
    Tag tag;
    uint32_t b;
  };

  struct {
    Tag tag;
  };
  C_C1_Body c1;
  C_C2_Body c2;
};

extern "C" {

void root(A a, B b, C c);

}  // extern "C"
