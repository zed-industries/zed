#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct ABC {
  float a;
  uint32_t b;
  uint32_t c;
};
constexpr static const ABC ABC_abc = ABC{
  /* .a = */ 1.0,
  /* .b = */ 2,
  /* .c = */ 3
};
constexpr static const ABC ABC_bac = ABC{
  /* .a = */ 1.0,
  /* .b = */ 2,
  /* .c = */ 3
};
constexpr static const ABC ABC_cba = ABC{
  /* .a = */ 1.0,
  /* .b = */ 2,
  /* .c = */ 3
};

struct BAC {
  uint32_t b;
  float a;
  int32_t c;
};
constexpr static const BAC BAC_abc = BAC{
  /* .b = */ 1,
  /* .a = */ 2.0,
  /* .c = */ 3
};
constexpr static const BAC BAC_bac = BAC{
  /* .b = */ 1,
  /* .a = */ 2.0,
  /* .c = */ 3
};
constexpr static const BAC BAC_cba = BAC{
  /* .b = */ 1,
  /* .a = */ 2.0,
  /* .c = */ 3
};

extern "C" {

void root(ABC a1, BAC a2);

}  // extern "C"
