#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uintptr_t TITLE_SIZE = 80;

template<uintptr_t CAP>
using CArrayString = int8_t[CAP];

struct Book {
  CArrayString<TITLE_SIZE> title;
  CArrayString<40> author;
};

extern "C" {

void root(Book *a);

}  // extern "C"
