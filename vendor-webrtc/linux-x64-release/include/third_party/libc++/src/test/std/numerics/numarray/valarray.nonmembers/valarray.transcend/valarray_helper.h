#ifndef LIBCPP_TEST_VALARRAY_HELPER_H
#define LIBCPP_TEST_VALARRAY_HELPER_H

#include <cmath>

// Returns whether `x` and `y` are equal, up to the given number of
// significant digits after the decimal.
//
// Specifically, we look whether `abs(x - y) < epsilon`, where epsilon
// is `(1 / 10)^p`, assuming p is the number of digits we care about.
// This means we're basically looking whether `abs(x - y)` is less
// than `0.00..001` for some number of digits.
inline bool is_about(double x, double y, int significant_digits) {
    double epsilon = std::pow(1.0 / 10.0, significant_digits);
    return std::abs(x - y) < epsilon;
}

#endif /* LIBCPP_TEST_VALARRAY_HELPER */
