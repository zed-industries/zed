/*
 * libjingle
 * Copyright 2004--2010, Google Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *  1. Redistributions of source code must retain the above copyright notice,
 *     this list of conditions and the following disclaimer.
 *  2. Redistributions in binary form must reproduce the above copyright notice,
 *     this list of conditions and the following disclaimer in the documentation
 *     and/or other materials provided with the distribution.
 *  3. The name of the author may not be used to endorse or promote products
 *     derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO
 * EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
 * OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 * OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
 * ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef AUDIO_DEVICE_PULSEAUDIOSYMBOLTABLE_LINUX_H_
#define AUDIO_DEVICE_PULSEAUDIOSYMBOLTABLE_LINUX_H_

#include "modules/audio_device/linux/latebindingsymboltable_linux.h"

namespace webrtc {
namespace adm_linux_pulse {

// The PulseAudio symbols we need, as an X-Macro list.
// This list must contain precisely every libpulse function that is used in
// the ADM LINUX PULSE Device and Mixer classes
#define PULSE_AUDIO_SYMBOLS_LIST           \
  X(pa_bytes_per_second)                   \
  X(pa_context_connect)                    \
  X(pa_context_disconnect)                 \
  X(pa_context_errno)                      \
  X(pa_context_get_protocol_version)       \
  X(pa_context_get_server_info)            \
  X(pa_context_get_sink_info_list)         \
  X(pa_context_get_sink_info_by_index)     \
  X(pa_context_get_sink_info_by_name)      \
  X(pa_context_get_sink_input_info)        \
  X(pa_context_get_source_info_by_index)   \
  X(pa_context_get_source_info_by_name)    \
  X(pa_context_get_source_info_list)       \
  X(pa_context_get_state)                  \
  X(pa_context_new)                        \
  X(pa_context_set_sink_input_volume)      \
  X(pa_context_set_sink_input_mute)        \
  X(pa_context_set_source_volume_by_index) \
  X(pa_context_set_source_mute_by_index)   \
  X(pa_context_set_state_callback)         \
  X(pa_context_unref)                      \
  X(pa_cvolume_set)                        \
  X(pa_operation_get_state)                \
  X(pa_operation_unref)                    \
  X(pa_stream_connect_playback)            \
  X(pa_stream_connect_record)              \
  X(pa_stream_disconnect)                  \
  X(pa_stream_drop)                        \
  X(pa_stream_get_device_index)            \
  X(pa_stream_get_index)                   \
  X(pa_stream_get_latency)                 \
  X(pa_stream_get_sample_spec)             \
  X(pa_stream_get_state)                   \
  X(pa_stream_new)                         \
  X(pa_stream_peek)                        \
  X(pa_stream_readable_size)               \
  X(pa_stream_set_buffer_attr)             \
  X(pa_stream_set_overflow_callback)       \
  X(pa_stream_set_read_callback)           \
  X(pa_stream_set_state_callback)          \
  X(pa_stream_set_underflow_callback)      \
  X(pa_stream_set_write_callback)          \
  X(pa_stream_unref)                       \
  X(pa_stream_writable_size)               \
  X(pa_stream_write)                       \
  X(pa_strerror)                           \
  X(pa_threaded_mainloop_free)             \
  X(pa_threaded_mainloop_get_api)          \
  X(pa_threaded_mainloop_lock)             \
  X(pa_threaded_mainloop_new)              \
  X(pa_threaded_mainloop_signal)           \
  X(pa_threaded_mainloop_start)            \
  X(pa_threaded_mainloop_stop)             \
  X(pa_threaded_mainloop_unlock)           \
  X(pa_threaded_mainloop_wait)

LATE_BINDING_SYMBOL_TABLE_DECLARE_BEGIN(PulseAudioSymbolTable)
#define X(sym) \
  LATE_BINDING_SYMBOL_TABLE_DECLARE_ENTRY(PulseAudioSymbolTable, sym)
PULSE_AUDIO_SYMBOLS_LIST
#undef X
LATE_BINDING_SYMBOL_TABLE_DECLARE_END(PulseAudioSymbolTable)

}  // namespace adm_linux_pulse
}  // namespace webrtc

#endif  // AUDIO_DEVICE_PULSEAUDIOSYMBOLTABLE_LINUX_H_
