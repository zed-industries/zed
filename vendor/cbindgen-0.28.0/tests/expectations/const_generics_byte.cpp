#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<uint8_t OPEN, uint8_t CLOSE>
struct Parser {
  uint8_t *buf;
  uintptr_t len;
};

extern "C" {

void init_parens_parser(Parser<40, 41> *p, uint8_t *buf, uintptr_t len);

void destroy_parens_parser(Parser<40, 41> *p);

void init_braces_parser(Parser<123, 125> *p, uint8_t *buf, uintptr_t len);

}  // extern "C"
