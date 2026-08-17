// Copyright (c) Microsoft Corporation

#ifndef THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_TRACE_H_
#define THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_TRACE_H_
//
// Define the tracing flags.
//
// Tracing GUID - b254994f-46e6-4718-80a0-0a3aa50d6ce4
//

#define WPP_CONTROL_GUIDS                                             \
  WPP_DEFINE_CONTROL_GUID(                                            \
      MyDriver1TraceGuid, (b254994f, 46e6, 4718, 80a0, 0a3aa50d6ce4), \
                                                                      \
      WPP_DEFINE_BIT(MYDRIVER_ALL_INFO) WPP_DEFINE_BIT(TRACE_DRIVER)  \
          WPP_DEFINE_BIT(TRACE_DEVICE) WPP_DEFINE_BIT(TRACE_QUEUE))

#define WPP_FLAG_LEVEL_LOGGER(flag, level) WPP_LEVEL_LOGGER(flag)

#define WPP_FLAG_LEVEL_ENABLED(flag, level) \
  (WPP_LEVEL_ENABLED(flag) && WPP_CONTROL(WPP_BIT_##flag).Level >= level)

#define WPP_LEVEL_FLAGS_LOGGER(lvl, flags) WPP_LEVEL_LOGGER(flags)

#define WPP_LEVEL_FLAGS_ENABLED(lvl, flags) \
  (WPP_LEVEL_ENABLED(flags) && WPP_CONTROL(WPP_BIT_##flags).Level >= lvl)

//
// This comment block is scanned by the trace preprocessor to define our
// Trace function.
//
// begin_wpp config
// FUNC Trace{FLAG=MYDRIVER_ALL_INFO}(LEVEL, MSG, ...);
// FUNC TraceEvents(LEVEL, FLAGS, MSG, ...);
// end_wpp

//
//
// Driver specific #defines
//

#define MYDRIVER_TRACING_ID \
  L"Microsoft\\UMDF2.25\\ChromiumVirtualDisplayDriver v1.0"

#endif  // THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_TRACE_H_
