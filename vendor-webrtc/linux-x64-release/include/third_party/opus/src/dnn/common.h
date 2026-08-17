

#ifndef COMMON_H
#define COMMON_H

#include <stdlib.h>
#include <string.h>
#include <math.h>
#include "opus_defines.h"

#define LOG256 5.5451774445f
static OPUS_INLINE float log2_approx(float x)
{
   int integer;
   float frac;
   union {
      float f;
      int i;
   } in;
   in.f = x;
   integer = (in.i>>23)-127;
   in.i -= integer<<23;
   frac = in.f - 1.5f;
   frac = -0.41445418f + frac*(0.95909232f
          + frac*(-0.33951290f + frac*0.16541097f));
   return 1+integer+frac;
}

#define log_approx(x) (0.69315f*log2_approx(x))

static OPUS_INLINE float ulaw2lin(float u)
{
    float s;
    float scale_1 = 32768.f/255.f;
    u = u - 128.f;
    s = u >= 0.f ? 1.f : -1.f;
    u = fabs(u);
    return s*scale_1*(exp(u/128.*LOG256)-1);
}

static OPUS_INLINE int lin2ulaw(float x)
{
    float u;
    float scale = 255.f/32768.f;
    int s = x >= 0 ? 1 : -1;
    x = fabs(x);
    u = (s*(128*log_approx(1+scale*x)/LOG256));
    u = 128 + u;
    if (u < 0) u = 0;
    if (u > 255) u = 255;
    return (int)floor(.5 + u);
}



#endif
