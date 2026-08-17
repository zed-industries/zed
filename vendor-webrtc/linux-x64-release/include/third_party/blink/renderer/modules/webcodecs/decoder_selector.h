// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_DECODER_SELECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_DECODER_SELECTOR_H_

#include <memory>

#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "media/base/demuxer_stream.h"
#include "media/base/media_util.h"
#include "media/filters/decoder_selector.h"
#include "media/filters/decoder_stream_traits.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

template <media::DemuxerStream::Type StreamType>
class NullDemuxerStream;

template <media::DemuxerStream::Type StreamType>
class DecoderSelector {
 public:
  typedef media::DecoderStreamTraits<StreamType> StreamTraits;
  typedef typename StreamTraits::DecoderType Decoder;
  typedef typename StreamTraits::DecoderConfigType DecoderConfig;
  using DecoderOrError =
      typename media::DecoderSelector<StreamType>::DecoderOrError;

  // Callback to create a list of decoders to select from.
  using CreateDecodersCB =
      base::RepeatingCallback<std::vector<std::unique_ptr<Decoder>>()>;

  // Emits the result of a single call to SelectDecoder(). Parameter is
  // the initialized Decoder. nullptr if selection failed. The caller owns the
  // Decoder.
  using SelectDecoderCB = base::OnceCallback<void(std::unique_ptr<Decoder>)>;

  // Construction can happen on any thread, but all subsequent API calls
  // including destruction must use |task_runner| thread.
  // Provided callbacks will be called on |task_runner|. |output_cb| will always
  // be Post()'ed.
  DecoderSelector(scoped_refptr<base::SequencedTaskRunner> task_runner,
                  CreateDecodersCB create_decoders_cb,
                  typename Decoder::OutputCB output_cb);

  // Aborts any pending decoder selection.
  ~DecoderSelector();

  // Disallow copy and assign.
  DecoderSelector(const DecoderSelector&) = delete;
  DecoderSelector& operator=(const DecoderSelector&) = delete;

  // Selects and initializes a decoder using |config|. Decoder will
  // be returned via |select_decoder_cb| posted to |task_runner_|. Subsequent
  // calls will again select from the full list of decoders.
  void SelectDecoder(const DecoderConfig& config,
                     bool low_delay,
                     SelectDecoderCB select_decoder_cb);

 private:
  // Helper to create |stream_traits_|.
  std::unique_ptr<StreamTraits> CreateStreamTraits();

  // Proxy SelectDecoderCB from impl_ to our |select_decoder_cb|.
  void OnDecoderSelected(SelectDecoderCB select_decoder_cb,
                         DecoderOrError decoder_or_error,
                         std::unique_ptr<media::DecryptingDemuxerStream>);

  // Implements heavy lifting for decoder selection.
  media::DecoderSelector<StreamType> impl_;

  // Shim to satisfy dependencies of |impl_|. Provides DecoderConfig to |impl_|.
  std::unique_ptr<NullDemuxerStream<StreamType>> demuxer_stream_;

  // Helper to unify API for configuring audio/video decoders.
  std::unique_ptr<StreamTraits> stream_traits_;

  // Repeating callback for decoder outputs.
  typename Decoder::OutputCB output_cb_;

  // TODO(chcunningham): Route MEDIA_LOG for WebCodecs.
  media::NullMediaLog null_media_log_;

  base::WeakPtrFactory<DecoderSelector<StreamType>> weak_factory_{this};
};

typedef DecoderSelector<media::DemuxerStream::VIDEO>
    WebCodecsVideoDecoderSelector;
typedef DecoderSelector<media::DemuxerStream::AUDIO>
    WebCodecsAudioDecoderSelector;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_DECODER_SELECTOR_H_
