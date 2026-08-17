// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_OUTPUT_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_OUTPUT_MAP_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_sync_iterator_midi_output_map.h"
#include "third_party/blink/renderer/modules/webmidi/midi_output.h"
#include "third_party/blink/renderer/modules/webmidi/midi_port_map.h"

namespace blink {

class MIDIOutputMap : public MIDIPortMap<MIDIOutputMap, MIDIOutput> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit MIDIOutputMap(HeapVector<Member<MIDIOutput>>&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_OUTPUT_MAP_H_
