/*
 * Copyright (C) 2019 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACE_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACE_PARSER_H_

#include <cstdint>
#include <string>

namespace perfetto::trace_processor {
namespace perf_importer {
struct Record;
}
namespace instruments_importer {
struct Row;
}
namespace gecko_importer {
struct GeckoEvent;
}
namespace art_method {
struct ArtMethodEvent;
}
namespace perf_text_importer {
struct PerfTextEvent;
}

struct AndroidDumpstateEvent;
struct AndroidLogEvent;
class PacketSequenceStateGeneration;
class TraceBlobView;
struct InlineSchedSwitch;
class FuchsiaRecord;
struct SystraceLine;
struct InlineSchedWaking;
struct TracePacketData;
struct TrackEventData;
struct LegacyV8CpuProfileEvent;

class ProtoTraceParser {
 public:
  virtual ~ProtoTraceParser();
  virtual void ParseTracePacket(int64_t, TracePacketData) = 0;
  virtual void ParseTrackEvent(int64_t, TrackEventData) = 0;
  virtual void ParseEtwEvent(uint32_t, int64_t, TracePacketData) = 0;
  virtual void ParseFtraceEvent(uint32_t, int64_t, TracePacketData) = 0;
  virtual void ParseInlineSchedSwitch(uint32_t, int64_t, InlineSchedSwitch) = 0;
  virtual void ParseInlineSchedWaking(uint32_t, int64_t, InlineSchedWaking) = 0;
};

class JsonTraceParser {
 public:
  virtual ~JsonTraceParser();
  virtual void ParseJsonPacket(int64_t, std::string) = 0;
  virtual void ParseSystraceLine(int64_t, SystraceLine) = 0;
  virtual void ParseLegacyV8ProfileEvent(int64_t, LegacyV8CpuProfileEvent) = 0;
};

class FuchsiaRecordParser {
 public:
  virtual ~FuchsiaRecordParser();
  virtual void ParseFuchsiaRecord(int64_t, FuchsiaRecord) = 0;
};

class PerfRecordParser {
 public:
  virtual ~PerfRecordParser();
  virtual void ParsePerfRecord(int64_t, perf_importer::Record) = 0;
};

class SpeRecordParser {
 public:
  virtual ~SpeRecordParser();
  virtual void ParseSpeRecord(int64_t, TraceBlobView) = 0;
};

class InstrumentsRowParser {
 public:
  virtual ~InstrumentsRowParser();
  virtual void ParseInstrumentsRow(int64_t, instruments_importer::Row) = 0;
};

class AndroidDumpstateEventParser {
 public:
  virtual ~AndroidDumpstateEventParser();
  virtual void ParseAndroidDumpstateEvent(int64_t, AndroidDumpstateEvent) = 0;
};

class AndroidLogEventParser {
 public:
  virtual ~AndroidLogEventParser();
  virtual void ParseAndroidLogEvent(int64_t, AndroidLogEvent) = 0;
};

class GeckoTraceParser {
 public:
  virtual ~GeckoTraceParser();
  virtual void ParseGeckoEvent(int64_t, gecko_importer::GeckoEvent) = 0;
};

class ArtMethodParser {
 public:
  virtual ~ArtMethodParser();
  virtual void ParseArtMethodEvent(int64_t, art_method::ArtMethodEvent) = 0;
};

class PerfTextTraceParser {
 public:
  virtual ~PerfTextTraceParser();
  virtual void ParsePerfTextEvent(int64_t,
                                  perf_text_importer::PerfTextEvent) = 0;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACE_PARSER_H_
