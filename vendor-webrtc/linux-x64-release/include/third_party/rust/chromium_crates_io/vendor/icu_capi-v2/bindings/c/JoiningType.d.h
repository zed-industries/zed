#ifndef JoiningType_D_H
#define JoiningType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum JoiningType {
  JoiningType_NonJoining = 0,
  JoiningType_JoinCausing = 1,
  JoiningType_DualJoining = 2,
  JoiningType_LeftJoining = 3,
  JoiningType_RightJoining = 4,
  JoiningType_Transparent = 5,
} JoiningType;

typedef struct JoiningType_option {union { JoiningType ok; }; bool is_ok; } JoiningType_option;



#endif // JoiningType_D_H
