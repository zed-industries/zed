// This file exists to override the processor logging for unit tests,
// since it confuses XCode into thinking unit tests have failed.
#include <sstream>

namespace google_breakpad {
extern std::ostringstream info_log;
}

#define BPLOG_INFO_STREAM google_breakpad::info_log
