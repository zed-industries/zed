#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Constants shared by multiple CSS Box Alignment properties
 *
 * These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
 */
typedef struct StyleAlignFlags {
  uint8_t bits;
} StyleAlignFlags;
/**
 * 'auto'
 */
#define StyleAlignFlags_AUTO (StyleAlignFlags){ .bits = (uint8_t)0 }
/**
 * 'normal'
 */
#define StyleAlignFlags_NORMAL (StyleAlignFlags){ .bits = (uint8_t)1 }
/**
 * 'start'
 */
#define StyleAlignFlags_START (StyleAlignFlags){ .bits = (uint8_t)(1 << 1) }
/**
 * 'end'
 */
#define StyleAlignFlags_END (StyleAlignFlags){ .bits = (uint8_t)(1 << 2) }
#define StyleAlignFlags_ALIAS (StyleAlignFlags){ .bits = (uint8_t)(StyleAlignFlags_END).bits }
/**
 * 'flex-start'
 */
#define StyleAlignFlags_FLEX_START (StyleAlignFlags){ .bits = (uint8_t)(1 << 3) }
#define StyleAlignFlags_MIXED (StyleAlignFlags){ .bits = (uint8_t)(((1 << 4) | (StyleAlignFlags_FLEX_START).bits) | (StyleAlignFlags_END).bits) }
#define StyleAlignFlags_MIXED_SELF (StyleAlignFlags){ .bits = (uint8_t)(((1 << 5) | (StyleAlignFlags_FLEX_START).bits) | (StyleAlignFlags_END).bits) }

/**
 * An arbitrary identifier for a native (OS compositor) surface
 */
typedef struct StyleNativeSurfaceId {
  uint64_t _0;
} StyleNativeSurfaceId;
/**
 * A special id for the native surface that is used for debug / profiler overlays.
 */
#define StyleNativeSurfaceId_DEBUG_OVERLAY (StyleNativeSurfaceId){ ._0 = UINT64_MAX }

typedef struct StyleNativeTileId {
  struct StyleNativeSurfaceId surface_id;
  int32_t x;
  int32_t y;
} StyleNativeTileId;
/**
 * A special id for the native surface that is used for debug / profiler overlays.
 */
#define StyleNativeTileId_DEBUG_OVERLAY (StyleNativeTileId){ .surface_id = StyleNativeSurfaceId_DEBUG_OVERLAY, .x = 0, .y = 0 }

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct StyleAlignFlags flags, struct StyleNativeTileId tile);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
