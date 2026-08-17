/* Copyright 2013 Google Inc. All Rights Reserved.

   Distributed under MIT license.
   See file LICENSE for detail or copy at https://opensource.org/licenses/MIT
*/

/* Helper for rounding */

#ifndef WOFF2_ROUND_H_
#define WOFF2_ROUND_H_

#include <limits>

namespace woff2 {

// Round a value up to the nearest multiple of 4. Don't round the value in the
// case that rounding up overflows.
template<typename T> T Round4(T value) {
  if (std::numeric_limits<T>::max() - value < 3) {
    return value;
  }
  return (value + 3) & ~3;
}

} // namespace woff2

#endif  // WOFF2_ROUND_H_
