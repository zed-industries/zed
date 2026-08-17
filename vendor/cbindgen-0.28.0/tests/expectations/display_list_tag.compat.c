#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Rect {
  float x;
  float y;
  float w;
  float h;
};

struct Color {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
};

enum DisplayItem_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Fill,
  Image,
  ClearScreen,
};
#ifndef __cplusplus
typedef uint8_t DisplayItem_Tag;
#endif // __cplusplus

struct Fill_Body {
  DisplayItem_Tag tag;
  struct Rect _0;
  struct Color _1;
};

struct Image_Body {
  DisplayItem_Tag tag;
  uint32_t id;
  struct Rect bounds;
};

union DisplayItem {
  DisplayItem_Tag tag;
  struct Fill_Body fill;
  struct Image_Body image;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

bool push_item(union DisplayItem item);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
