// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_UTIL_AUDIO_DECODER_H_
#define MEDIAPIPE_UTIL_AUDIO_DECODER_H_

#include <cstdint>  // required by avutil.h
#include <deque>
#include <string>
#include <vector>

#include "absl/flags/flag.h"
#include "absl/time/time.h"
#include "mediapipe/framework/formats/time_series_header.pb.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/timestamp.h"
#include "mediapipe/util/audio_decoder.pb.h"

extern "C" {
#include "libavcodec/avcodec.h"
#include "libavformat/avformat.h"
#include "libavutil/avutil.h"
#include "libavutil/dict.h"
#include "mediapipe/util/audio_decoder.pb.h"
}

namespace mediapipe {

using mediapipe::AudioStreamOptions;
using mediapipe::TimeSeriesHeader;

// The base helper class for a processor which handles decoding of a single
// stream.
class BasePacketProcessor {
 public:
  BasePacketProcessor();
  virtual ~BasePacketProcessor();

  // Opens the codec.
  virtual absl::Status Open(int id, AVStream* stream) = 0;

  // Processes a packet of data.  Caller retains ownership of packet.
  virtual absl::Status ProcessPacket(AVPacket* packet) = 0;

  // Returns true if the processor has data immediately available
  // (without providing more data with ProcessPacket()).
  bool HasData();

  // Fills packet with the next frame of data.  Returns an empty packet
  // if there is nothing to return.
  absl::Status GetData(Packet* packet);

  // Once no more AVPackets are available in the file, each stream must
  // be flushed to get any remaining frames which the codec is buffering.
  absl::Status Flush();

  // Closes the Processor, this does not close the file.  You may not
  // call ProcessPacket() after calling Close().  Close() may be called
  // repeatedly.
  void Close();

 protected:
  // Decodes frames in a packet.
  virtual absl::Status Decode(const AVPacket& packet,
                              bool ignore_decode_failures);

  // Processes a decoded frame.
  virtual absl::Status ProcessDecodedFrame(const AVPacket& packet) = 0;

  // Corrects the given PTS for MPEG PTS rollover. Assumed to be called with
  // the PTS of each frame in decode order. We detect a rollover whenever the
  // PTS timestamp changes by more than 2^33/2 (half the timestamp space). For
  // video this means every 26.5h with 1 PTS tick = 1/90000 of a second.
  // Example timeline:
  // CorrectPtsForRollover(0) -> 0
  // CorrectPtsForRollover(42) -> 42
  // CorrectPtsForRollover(2^33 - 1) -> 2^33 - 1
  // CorrectPtsForRollover(0) -> 2^33  // PTS in media rolls over, corrected.
  // CorrectPtsForRollover(1) -> 2^33 + 1
  int64_t CorrectPtsForRollover(int64_t media_pts);

  AVCodecContext* avcodec_ctx_ = nullptr;
  const AVCodec* avcodec_ = nullptr;
  AVDictionary* avcodec_opts_ = nullptr;
  AVFrame* decoded_frame_ = nullptr;

  // Stream ID this object processes.
  int id_ = -1;

  // Set to true if the stream has been flushed and no more AVPackets
  // will be processed with it.
  bool flushed_ = false;

  // The source time base.
  AVRational source_time_base_;
  // The output time base.
  const AVRational output_time_base_;

  // The source frame rate (estimated from header information).
  AVRational source_frame_rate_;

  // The number of frames that were successfully processed.
  int64_t num_frames_processed_ = 0;

  int bytes_per_sample_ = 0;

  // boolean flag to show if time regression has been detected for last frame;
  bool last_frame_time_regression_detected_ = false;

  // The last rollover corrected PTS returned by CorrectPtsForRollover.
  int64_t rollover_corrected_last_pts_ = AV_NOPTS_VALUE;

  // The buffer of current frames.
  std::deque<Packet> buffer_;
};

// Class which decodes packets from a single audio stream.
class AudioPacketProcessor : public BasePacketProcessor {
 public:
  explicit AudioPacketProcessor(const AudioStreamOptions& options);

  absl::Status Open(int id, AVStream* stream) override;

  absl::Status ProcessPacket(AVPacket* packet) override;

  absl::Status FillHeader(TimeSeriesHeader* header) const;

 private:
  // Appends audio in buffer(s) to the output buffer (buffer_).
  absl::Status AddAudioDataToBuffer(const Timestamp output_timestamp,
                                    uint8_t* const* raw_audio,
                                    int buf_size_bytes);

  // Converts a number of samples into an approximate stream timestamp value.
  int64_t SampleNumberToTimestamp(const int64_t sample_number);
  int64_t TimestampToSampleNumber(const int64_t timestamp);

  // Converts a timestamp/sample number to microseconds.
  int64_t TimestampToMicroseconds(const int64_t timestamp);
  int64_t SampleNumberToMicroseconds(const int64_t sample_number);

  // Returns an error if the sample format in avformat_ctx_.sample_format
  // is not supported.
  absl::Status ValidateSampleFormat();

  // Processes a decoded audio frame.  audio_frame_ must have been filled
  // with the frame before calling this function.
  absl::Status ProcessDecodedFrame(const AVPacket& packet) override;

  // Corrects PTS for rollover if correction is enabled.
  int64_t MaybeCorrectPtsForRollover(int64_t media_pts);

  // Number of channels to output. This value might be different from
  // the actual number of channels for the current AVPacket, found in
  // avcodec_ctx_->channels.
  int num_channels_ = -1;

  // Sample rate of the data to output. This value might be different
  // from the actual sample rate for the current AVPacket, found in
  // avcodec_ctx_->sample_rate.
  int64_t sample_rate_ = -1;

  // The time base of audio samples (i.e. the reciprocal of the sample rate).
  AVRational sample_time_base_;

  // The timestamp of the last packet added to the buffer.
  Timestamp last_timestamp_;

  // The expected sample number based on counting samples.
  int64_t expected_sample_number_ = 0;

  // Options for the processor.
  AudioStreamOptions options_;
};

// Decode the audio streams of a media file.  The AudioDecoder is responsible
// for demuxing the audio streams in the container format, whereas decoding of
// the content is delegated to AudioPacketProcessor.
class AudioDecoder {
 public:
  AudioDecoder();
  ~AudioDecoder();

  absl::Status Initialize(const std::string& input_file,
                          const mediapipe::AudioDecoderOptions options);

  absl::Status GetData(int* options_index, Packet* data);

  absl::Status Close();

  absl::Status FillAudioHeader(const AudioStreamOptions& stream_option,
                               TimeSeriesHeader* header) const;

 private:
  absl::Status ProcessPacket();
  absl::Status Flush();

  std::map<int, int> stream_id_to_audio_options_index_;
  std::map<int, int> stream_index_to_stream_id_;
  std::map<int, std::unique_ptr<AudioPacketProcessor>> audio_processor_;

  // Indexed by container stream index, true if the stream has not seen
  // a packet (whether returned or not), and false otherwise.
  std::vector<bool> is_first_packet_;
  bool flushed_ = false;

  Timestamp start_time_ = Timestamp::Unset();
  Timestamp end_time_ = Timestamp::Unset();

  AVFormatContext* avformat_ctx_ = nullptr;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_AUDIO_DECODER_H_
