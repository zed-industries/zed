#ifndef __INPUTSTREAM_H__
#define __INPUTSTREAM_H__

#include <cstddef>

struct InputStream {
  virtual int read (void* ptr, size_t len) = 0;
};

#endif //__INPUTSTREAM_H__
