// Copyright 2006 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#ifndef CLIENT_WINDOWS_SENDER_CRASH_REPORT_SENDER_H__
#define CLIENT_WINDOWS_SENDER_CRASH_REPORT_SENDER_H__

// CrashReportSender is a "static" class which provides an API to upload
// crash reports via HTTP(S).  A crash report is formatted as a multipart POST
// request, which contains a set of caller-supplied string key/value pairs,
// and a minidump file to upload.
//
// To use this library in your project, you will need to link against
// wininet.lib.

#pragma warning( push )
// Disable exception handler warnings.
#pragma warning( disable : 4530 ) 

#include <map>
#include <string>

namespace google_breakpad {

using std::wstring;
using std::map;

typedef enum {
  RESULT_FAILED = 0,  // Failed to communicate with the server; try later.
  RESULT_REJECTED,    // Successfully sent the crash report, but the
                      // server rejected it; don't resend this report.
  RESULT_SUCCEEDED,   // The server accepted the crash report.
  RESULT_THROTTLED    // No attempt was made to send the crash report, because
                      // we exceeded the maximum reports per day.
} ReportResult;

class CrashReportSender {
 public:
  // Initializes a CrashReportSender instance.
  // If checkpoint_file is non-empty, breakpad will persist crash report
  // state to this file.  A checkpoint file is required for
  // set_max_reports_per_day() to function properly.
  explicit CrashReportSender(const wstring& checkpoint_file);
  ~CrashReportSender() {}

  // Sets the maximum number of crash reports that will be sent in a 24-hour
  // period.  This uses the state persisted to the checkpoint file.
  // The default value of -1 means that there is no limit on reports sent.
  void set_max_reports_per_day(int reports) {
    max_reports_per_day_ = reports;
  }

  int max_reports_per_day() const { return max_reports_per_day_; }

  // Sends the specified files, along with the map of
  // name value pairs, as a multipart POST request to the given URL.
  // Parameter names must contain only printable ASCII characters,
  // and may not contain a quote (") character.
  // Only HTTP(S) URLs are currently supported.  The return value indicates
  // the result of the operation (see above for possible results).
  // If report_code is non-NULL and the report is sent successfully (that is,
  // the return value is RESULT_SUCCEEDED), a code uniquely identifying the
  // report will be returned in report_code.
  // (Otherwise, report_code will be unchanged.)
  ReportResult SendCrashReport(const wstring& url,
                               const map<wstring, wstring>& parameters,
                               const map<wstring, wstring>& files,
                               wstring* report_code);

 private:
  // Reads persistent state from a checkpoint file.
  void ReadCheckpoint(FILE* fd);

  // Called when a new report has been sent, to update the checkpoint state.
  void ReportSent(int today);

  // Returns today's date (UTC) formatted as YYYYMMDD.
  int GetCurrentDate() const;

  // Opens the checkpoint file with the specified mode.
  // Returns zero on success, or an error code on failure.
  int OpenCheckpointFile(const wchar_t* mode, FILE** fd);

  wstring checkpoint_file_;
  int max_reports_per_day_;
  // The last date on which we sent a report, expressed as YYYYMMDD.
  int last_sent_date_;
  // Number of reports sent on last_sent_date_
  int reports_sent_;

  // Disallow copy constructor and operator=
  explicit CrashReportSender(const CrashReportSender&);
  void operator=(const CrashReportSender&);
};

}  // namespace google_breakpad

#pragma warning( pop )

#endif  // CLIENT_WINDOWS_SENDER_CRASH_REPORT_SENDER_H__
