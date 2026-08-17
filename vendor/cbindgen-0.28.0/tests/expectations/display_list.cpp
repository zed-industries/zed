#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

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

union DisplayItem {
  enum class Tag : uint8_t {
    Fill,
    Image,
    ClearScreen,
  };

  struct Fill_Body {
    Tag tag;
    Rect _0;
    Color _1;
  };

  struct Image_Body {
    Tag tag;
    uint32_t id;
    Rect bounds;
  };

  struct {
    Tag tag;
  };
  Fill_Body fill;
  Image_Body image;
};

extern "C" {

bool push_item(DisplayItem item);

}  // extern "C"
