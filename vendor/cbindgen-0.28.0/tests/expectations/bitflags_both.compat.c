#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Constants shared by multiple CSS Box Alignment properties
 *
 * These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
 */
typedef struct AlignFlags {
  uint8_t bits;
} AlignFlags;
/**
 * 'auto'
 */
#define AlignFlags_AUTO (AlignFlags){ .bits = (uint8_t)0 }
/**
 * 'normal'
 */
#define AlignFlags_NORMAL (AlignFlags){ .bits = (uint8_t)1 }
/**
 * 'start'
 */
#define AlignFlags_START (AlignFlags){ .bits = (uint8_t)(1 << 1) }
/**
 * 'end'
 */
#define AlignFlags_END (AlignFlags){ .bits = (uint8_t)(1 << 2) }
#define AlignFlags_ALIAS (AlignFlags){ .bits = (uint8_t)(AlignFlags_END).bits }
/**
 * 'flex-start'
 */
#define AlignFlags_FLEX_START (AlignFlags){ .bits = (uint8_t)(1 << 3) }
#define AlignFlags_MIXED (AlignFlags){ .bits = (uint8_t)(((1 << 4) | (AlignFlags_FLEX_START).bits) | (AlignFlags_END).bits) }
#define AlignFlags_MIXED_SELF (AlignFlags){ .bits = (uint8_t)(((1 << 5) | (AlignFlags_FLEX_START).bits) | (AlignFlags_END).bits) }

typedef struct DebugFlags {
  uint32_t bits;
} DebugFlags;
/**
 * Flag with the topmost bit set of the u32
 */
#define DebugFlags_BIGGEST_ALLOWED (DebugFlags){ .bits = (uint32_t)(1 << 31) }

typedef struct LargeFlags {
  uint64_t bits;
} LargeFlags;
/**
 * Flag with a very large shift that usually would be narrowed.
 */
#define LargeFlags_LARGE_SHIFT (LargeFlags){ .bits = (uint64_t)(1ull << 44) }
#define LargeFlags_INVERTED (LargeFlags){ .bits = (uint64_t)~(LargeFlags_LARGE_SHIFT).bits }

typedef struct OutOfLine {
  uint32_t _0;
} OutOfLine;
#define OutOfLine_A (OutOfLine){ ._0 = (uint32_t)1 }
#define OutOfLine_B (OutOfLine){ ._0 = (uint32_t)2 }
#define OutOfLine_AB (OutOfLine){ ._0 = (uint32_t)((OutOfLine_A)._0 | (OutOfLine_B)._0) }

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct AlignFlags flags,
          struct DebugFlags bigger_flags,
          struct LargeFlags largest_flags,
          struct OutOfLine out_of_line);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
