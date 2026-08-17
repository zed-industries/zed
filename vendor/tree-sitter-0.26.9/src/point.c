#include "point.h"

void ts_point_edit(TSPoint *point, uint32_t *byte, const TSInputEdit *edit) {
  uint32_t start_byte = *byte;
  TSPoint start_point = *point;

  if (start_byte >= edit->old_end_byte) {
    start_byte = edit->new_end_byte + (start_byte - edit->old_end_byte);
    start_point = point_add(edit->new_end_point, point_sub(start_point, edit->old_end_point));
  } else if (start_byte > edit->start_byte) {
    start_byte = edit->new_end_byte;
    start_point = edit->new_end_point;
  }

  *point = start_point;
  *byte = start_byte;
}
