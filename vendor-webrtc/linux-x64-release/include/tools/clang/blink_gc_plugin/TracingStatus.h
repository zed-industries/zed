// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BLINK_GC_PLUGIN_TRACING_STATUS_H_
#define TOOLS_BLINK_GC_PLUGIN_TRACING_STATUS_H_

// TracingStatus is a four-point value ordered by
//       illegal < unneeded < unknown < needed
//
// It is used to categorize tracing of fields:
//
//  * illegal  field is invalid/illegal to trace.
//  * unneeded field has type with no traceable fields of its own;
//             it may have an empty trace() method. Not harmful
//             to trace, but not needed.
//  * unknown  initial TracingStatus value.
//  * needed   field is a heap reference or an object containing
//             traceable fields.
//
// Tracing status |illegal| is considered an error; treating |unneeded| also
// as an error would detect and report unnecessary tracing of objects that
// probably don't need to be on the Blink GC heap. However, template use
// and instantiation can leave us with classes that do have empty trace
// methods and no traceable fields -- reporting these as errors/warnings
// wouldn't work. Hence, only consider |illegal| as an error TracingStatus
// state.
class TracingStatus {
 public:
  static TracingStatus Illegal() { return kIllegal; }
  static TracingStatus Unneeded() { return kUnneeded; }
  static TracingStatus Unknown() { return kUnknown; }
  static TracingStatus Needed() { return kNeeded; }
  bool IsIllegal() const { return status_ == kIllegal; }
  bool IsUnneeded() const { return status_ == kUnneeded; }
  bool IsUnknown() const { return status_ == kUnknown; }
  bool IsNeeded() const { return status_ == kNeeded; }
  TracingStatus LUB(const TracingStatus& other) const {
    return status_ > other.status_ ? status_ : other.status_;
  }
  bool operator==(const TracingStatus& other) const {
    return status_ == other.status_;
  }
 private:
  enum Status { kIllegal, kUnneeded, kUnknown, kNeeded };
  TracingStatus(Status status) : status_(status) {}
  Status status_;
};

#endif // TOOLS_BLINK_GC_PLUGIN_TRACING_STATUS_H_
