#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  float x;
  float y;
  float w;
  float h;
} Rect;

typedef struct {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
} Color;

enum DisplayItem_Tag {
  Fill,
  Image,
  ClearScreen,
};
typedef uint8_t DisplayItem_Tag;

typedef struct {
  DisplayItem_Tag tag;
  Rect _0;
  Color _1;
} Fill_Body;

typedef struct {
  DisplayItem_Tag tag;
  uint32_t id;
  Rect bounds;
} Image_Body;

typedef union {
  DisplayItem_Tag tag;
  Fill_Body fill;
  Image_Body image;
} DisplayItem;

bool push_item(DisplayItem item);
