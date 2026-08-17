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

#ifndef AUDIO_DEVICE_ALSASYMBOLTABLE_LINUX_H_
#define AUDIO_DEVICE_ALSASYMBOLTABLE_LINUX_H_

#include "modules/audio_device/linux/latebindingsymboltable_linux.h"

namespace webrtc {
namespace adm_linux_alsa {

// The ALSA symbols we need, as an X-Macro list.
// This list must contain precisely every libasound function that is used in
// alsasoundsystem.cc.
#define ALSA_SYMBOLS_LIST                      \
  X(snd_device_name_free_hint)                 \
  X(snd_device_name_get_hint)                  \
  X(snd_device_name_hint)                      \
  X(snd_pcm_avail_update)                      \
  X(snd_pcm_close)                             \
  X(snd_pcm_delay)                             \
  X(snd_pcm_drop)                              \
  X(snd_pcm_open)                              \
  X(snd_pcm_prepare)                           \
  X(snd_pcm_readi)                             \
  X(snd_pcm_recover)                           \
  X(snd_pcm_resume)                            \
  X(snd_pcm_reset)                             \
  X(snd_pcm_state)                             \
  X(snd_pcm_set_params)                        \
  X(snd_pcm_get_params)                        \
  X(snd_pcm_start)                             \
  X(snd_pcm_stream)                            \
  X(snd_pcm_frames_to_bytes)                   \
  X(snd_pcm_bytes_to_frames)                   \
  X(snd_pcm_wait)                              \
  X(snd_pcm_writei)                            \
  X(snd_pcm_info_get_class)                    \
  X(snd_pcm_info_get_subdevices_avail)         \
  X(snd_pcm_info_get_subdevice_name)           \
  X(snd_pcm_info_set_subdevice)                \
  X(snd_pcm_info_get_id)                       \
  X(snd_pcm_info_set_device)                   \
  X(snd_pcm_info_set_stream)                   \
  X(snd_pcm_info_get_name)                     \
  X(snd_pcm_info_get_subdevices_count)         \
  X(snd_pcm_info_sizeof)                       \
  X(snd_pcm_hw_params)                         \
  X(snd_pcm_hw_params_malloc)                  \
  X(snd_pcm_hw_params_free)                    \
  X(snd_pcm_hw_params_any)                     \
  X(snd_pcm_hw_params_set_access)              \
  X(snd_pcm_hw_params_set_format)              \
  X(snd_pcm_hw_params_set_channels)            \
  X(snd_pcm_hw_params_set_rate_near)           \
  X(snd_pcm_hw_params_set_buffer_size_near)    \
  X(snd_card_next)                             \
  X(snd_card_get_name)                         \
  X(snd_config_update)                         \
  X(snd_config_copy)                           \
  X(snd_config_get_id)                         \
  X(snd_ctl_open)                              \
  X(snd_ctl_close)                             \
  X(snd_ctl_card_info)                         \
  X(snd_ctl_card_info_sizeof)                  \
  X(snd_ctl_card_info_get_id)                  \
  X(snd_ctl_card_info_get_name)                \
  X(snd_ctl_pcm_next_device)                   \
  X(snd_ctl_pcm_info)                          \
  X(snd_mixer_load)                            \
  X(snd_mixer_free)                            \
  X(snd_mixer_detach)                          \
  X(snd_mixer_close)                           \
  X(snd_mixer_open)                            \
  X(snd_mixer_attach)                          \
  X(snd_mixer_first_elem)                      \
  X(snd_mixer_elem_next)                       \
  X(snd_mixer_selem_get_name)                  \
  X(snd_mixer_selem_is_active)                 \
  X(snd_mixer_selem_register)                  \
  X(snd_mixer_selem_set_playback_volume_all)   \
  X(snd_mixer_selem_get_playback_volume)       \
  X(snd_mixer_selem_has_playback_volume)       \
  X(snd_mixer_selem_get_playback_volume_range) \
  X(snd_mixer_selem_has_playback_switch)       \
  X(snd_mixer_selem_get_playback_switch)       \
  X(snd_mixer_selem_set_playback_switch_all)   \
  X(snd_mixer_selem_has_capture_switch)        \
  X(snd_mixer_selem_get_capture_switch)        \
  X(snd_mixer_selem_set_capture_switch_all)    \
  X(snd_mixer_selem_has_capture_volume)        \
  X(snd_mixer_selem_set_capture_volume_all)    \
  X(snd_mixer_selem_get_capture_volume)        \
  X(snd_mixer_selem_get_capture_volume_range)  \
  X(snd_dlopen)                                \
  X(snd_dlclose)                               \
  X(snd_config)                                \
  X(snd_config_search)                         \
  X(snd_config_get_string)                     \
  X(snd_config_search_definition)              \
  X(snd_config_get_type)                       \
  X(snd_config_delete)                         \
  X(snd_config_iterator_entry)                 \
  X(snd_config_iterator_first)                 \
  X(snd_config_iterator_next)                  \
  X(snd_config_iterator_end)                   \
  X(snd_config_delete_compound_members)        \
  X(snd_config_get_integer)                    \
  X(snd_config_get_bool)                       \
  X(snd_dlsym)                                 \
  X(snd_strerror)                              \
  X(snd_lib_error)                             \
  X(snd_lib_error_set_handler)

LATE_BINDING_SYMBOL_TABLE_DECLARE_BEGIN(AlsaSymbolTable)
#define X(sym) LATE_BINDING_SYMBOL_TABLE_DECLARE_ENTRY(AlsaSymbolTable, sym)
ALSA_SYMBOLS_LIST
#undef X
LATE_BINDING_SYMBOL_TABLE_DECLARE_END(AlsaSymbolTable)

}  // namespace adm_linux_alsa
}  // namespace webrtc

#endif  // AUDIO_DEVICE_ALSASYMBOLTABLE_LINUX_H_
