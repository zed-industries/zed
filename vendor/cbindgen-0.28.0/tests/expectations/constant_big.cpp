#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uint64_t UNSIGNED_NEEDS_ULL_SUFFIX = 9223372036854775808ull;

constexpr static const uint64_t UNSIGNED_DOESNT_NEED_ULL_SUFFIX = 8070450532247928832;

constexpr static const int64_t SIGNED_NEEDS_ULL_SUFFIX = -9223372036854775808ull;

constexpr static const int64_t SIGNED_DOESNT_NEED_ULL_SUFFIX = -9223372036854775807;
