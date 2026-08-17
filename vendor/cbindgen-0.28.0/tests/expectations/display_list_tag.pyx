from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Rect:
    float x;
    float y;
    float w;
    float h;

  cdef struct Color:
    uint8_t r;
    uint8_t g;
    uint8_t b;
    uint8_t a;

  cdef enum:
    Fill,
    Image,
    ClearScreen,
  ctypedef uint8_t DisplayItem_Tag;

  cdef struct Fill_Body:
    DisplayItem_Tag tag;
    Rect _0;
    Color _1;

  cdef struct Image_Body:
    DisplayItem_Tag tag;
    uint32_t id;
    Rect bounds;

  cdef union DisplayItem:
    DisplayItem_Tag tag;
    Fill_Body fill;
    Image_Body image;

  bool push_item(DisplayItem item);
