#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define PREFIX_LEN 22

#define PREFIX_X (22 << 22)

#define PREFIX_Y (PREFIX_X + PREFIX_X)

typedef int32_t PREFIX_NamedLenArray[PREFIX_LEN];

typedef int32_t PREFIX_ValuedLenArray[22];

enum PREFIX_AbsoluteFontWeight_Tag {
  Weight,
  Normal,
  Bold,
};
typedef uint8_t PREFIX_AbsoluteFontWeight_Tag;

union PREFIX_AbsoluteFontWeight {
  PREFIX_AbsoluteFontWeight_Tag tag;
  struct {
    PREFIX_AbsoluteFontWeight_Tag weight_tag;
    float weight;
  };
};

void root(PREFIX_NamedLenArray x, PREFIX_ValuedLenArray y, union PREFIX_AbsoluteFontWeight z);
