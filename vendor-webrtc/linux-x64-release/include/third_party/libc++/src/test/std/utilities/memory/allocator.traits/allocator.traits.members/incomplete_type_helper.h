#ifndef TEST_INCOMPLETE_TYPE_HELPER_H
#define TEST_INCOMPLETE_TYPE_HELPER_H

#include "min_allocator.h"

namespace NS {
  struct Incomplete;
}

template <class T> struct Holder { T value; };

typedef Holder<NS::Incomplete> IncompleteHolder;

#endif
