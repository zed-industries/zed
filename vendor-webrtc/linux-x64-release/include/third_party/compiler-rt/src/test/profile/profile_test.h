

#ifndef PROFILE_TEST_H
#define PROFILE_TEST_H

#if defined(_MSC_VER)
# define ALIGNED(x) __declspec(align(x))
#else  // _MSC_VER
# define ALIGNED(x) __attribute__((aligned(x)))
#endif

#endif  // PROFILE_TEST_H
