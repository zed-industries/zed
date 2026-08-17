#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct dep_struct {
  uint32_t x;
  double y;
};

extern "C" {

uint32_t get_x(const dep_struct *dep_struct);

}  // extern "C"
