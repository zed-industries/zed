#ifndef MEDIAPIPE_WEB_JSPI_CHECK_H_
#define MEDIAPIPE_WEB_JSPI_CHECK_H_

#ifdef __EMSCRIPTEN__
#include <emscripten.h>
#endif

namespace mediapipe {

inline bool IsJspiAvailable() {
#ifdef __EMSCRIPTEN__
  // This matches the -sASYNCIFY=2 linkopt for emscripten.
  return emscripten_has_asyncify() == 2;
#else
  return false;
#endif
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_WEB_JSPI_CHECK_H_
