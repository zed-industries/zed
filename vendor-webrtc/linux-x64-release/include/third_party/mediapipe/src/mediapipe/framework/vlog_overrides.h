#ifndef MEDIAPIPE_FRAMEWORK_VLOG_OVERRIDES_H_
#define MEDIAPIPE_FRAMEWORK_VLOG_OVERRIDES_H_

namespace mediapipe {

// If possible, rely on --v / --vmodule to set VLOG level and modules.
//
// However, in cases when --v / --vmodule cannot be used (e.g. running an
// Android app and enabling VLOGs), MediaPipe allows to set VLOG --v / --vmodule
// overrides for debugging purposes which are applied when `CalculatorGraph` is
// created.
//
// Overrides:
// - MEDIAPIPE_VLOG_V (define and provide value you provide for --v)
// - MEDIAPIPE_VLOG_VMODULE (define and provide value you provide for --vmodule)
//
// You can set overrides by adding:
// ```
//   --copt=-DMEDIAPIPE_VLOG_VMODULE=\"*calculator*=5\"
// ```
// with your desired module patterns and VLOG levels (see more details for
// --vmodule) to your build command.
//
// IMPORTANT: mind that adding the above to your build command will trigger
// rebuild of the whole binary including dependencies. So, considering vlog
// overrides exist for debugging purposes only, it is faster to simply modify
// `vlog_overrides.cc` adding MEDIAPIPE_VLOG_V/VMODULE at the very top.
void SetVLogOverrides();

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_VLOG_OVERRIDES_H_
