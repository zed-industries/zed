#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

///With doc attr, each attr contribute to one line of document
///like this one with a new line character at its end
///and this one as well. So they are in the same paragraph
///
///Line ends with one new line should not break
///
///Line ends with two spaces and a new line
///should break to next line
///
///Line ends with two new lines
///
///Should break to next paragraph
void root();

}  // extern "C"
