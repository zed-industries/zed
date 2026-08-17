#ifndef MEDIAPIPE_CALCULATORS_CORE_PACKET_RESAMPLER_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_CORE_PACKET_RESAMPLER_CALCULATOR_H_

#include <cstdint>
#include <cstdlib>
#include <memory>
#include <string>

#include "absl/strings/str_cat.h"
#include "mediapipe/calculators/core/packet_resampler_calculator.pb.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/collection_item_id.h"
#include "mediapipe/framework/deps/mathutil.h"
#include "mediapipe/framework/deps/random_base.h"
#include "mediapipe/framework/formats/video_stream_header.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/port/status_macros.h"
#include "mediapipe/framework/tool/options_util.h"

namespace mediapipe {

class PacketReservoir {
 public:
  PacketReservoir(RandomBase* rng) : rng_(rng) {}
  // Replace candidate with current packet with 1/count_ probability.
  void AddSample(Packet sample) {
    if (rng_->UnbiasedUniform(++count_) == 0) {
      reservoir_ = sample;
    }
  }
  bool IsEnabled() { return rng_ && enabled_; }
  void Disable() {
    if (enabled_) enabled_ = false;
  }
  void Clear() { count_ = 0; }
  bool IsEmpty() { return count_ == 0; }
  Packet GetSample() { return reservoir_; }

 private:
  RandomBase* rng_;
  bool enabled_ = true;
  int32_t count_ = 0;
  Packet reservoir_;
};

// This calculator is used to normalize the frequency of the packets
// out of a stream. Given a desired frame rate, packets are going to be
// removed or added to achieve it.
//
// If jitter_ is specified:
//   - The first packet is chosen randomly (uniform distribution) among frames
//     that correspond to timestamps [0, 1/frame_rate).  Let the chosen packet
//     correspond to timestamp t.
//   - The next packet is chosen randomly (uniform distribution) among frames
//     that correspond to [t+(1-jitter)/frame_rate, t+(1+jitter)/frame_rate].
//     - if jitter_with_reflection is true, the timestamp will be reflected
//       against the boundaries of [t_0 + (k-1)/frame_rate, t_0 + k/frame_rate)
//       so that its marginal distribution is uniform within this interval.
//       In the formula, t_0 is the timestamp of the first sampled
//       packet, and the k is the packet index.
//       See paper (https://arxiv.org/abs/2002.01147) for details.
//   - t is updated and the process is repeated.
//   - Note that seed is specified as input side packet for reproducibility of
//     the resampling.  For Cloud ML Video Intelligence API, the hash of the
//     input video should serve this purpose.  For YouTube, either video ID or
//     content hex ID of the input video should do.
//   - If reproducible_samping is true, care is taken to allow reproducible
//     "mid-stream" sampling.  The calculator can be executed on a stream that
//     doesn't start at the first period.  For instance, if the calculator
//     is run on a 10 second stream it will produce the same set of samples
//     as two runs of the calculator, the first with 3 seconds of input starting
//     at time 0 and the second with 7 seconds of input starting at time +3s.
//     - In order to guarantee the exact same samples, 1) the inputs must be
//       aligned with the sampling period.  For instance, if the sampling rate
//       is 2 frames per second, streams should be aligned on 0.5 second
//       boundaries, and 2) the stream must include at least one extra packet
//       before and after the second aligned sampling period.
//
// If jitter_ is not specified:
//   - The first packet defines the first_timestamp of the output stream,
//     so it is always emitted.
//   - If more packets are emitted, they will have timestamp equal to
//     round(first_timestamp + k * period) , where k is a positive
//     integer and the period is defined by the frame rate.
//     Example: first_timestamp=0, fps=30, then the output stream
//              will have timestamps: 0, 33333, 66667, 100000, etc...
//   - The packets selected for the output stream are the ones closer
//     to the exact middle point (33333.33, 66666.67 in our previous
//     example). In case of ties, later packets are chosen.
//   - 'Empty' periods happen when there are no packets for a long time
//     (greater than a period). In this case, we send a copy of the last
//     packet received before the empty period.
// The jitter feature is disabled by default. To enable it, you need to
// implement CreateSecureRandom(const std::string&).
//
// The data stream may be either specified as the only stream (by index)
// or as the stream with tag "DATA".
//
// The input and output streams may be accompanied by a VIDEO_HEADER
// stream.  This stream includes a VideoHeader at Timestamp::PreStream().
// The input VideoHeader on the VIDEO_HEADER stream will always be updated
// with the resampler frame rate no matter what the options value for
// output_header is before being output on the output VIDEO_HEADER stream.
// If the input VideoHeader is not available, then only the frame rate
// value will be set in the output.
//
// Related:
//   packet_downsampler_calculator.cc: skips packets regardless of timestamps.
class PacketResamplerCalculator : public CalculatorBase {
 public:
  static absl::Status GetContract(CalculatorContract* cc);

  absl::Status Open(CalculatorContext* cc) override;
  absl::Status Close(CalculatorContext* cc) override;
  absl::Status Process(CalculatorContext* cc) override;

  // Given the current count of periods that have passed, this returns
  // the next valid timestamp of the middle point of the next period:
  //    if count is 0, it returns the first_timestamp_.
  //    if count is 1, it returns the first_timestamp_ + period (corresponding
  //       to the first tick using exact fps)
  // e.g. for frame_rate=30 and first_timestamp_=0:
  //    0: 0
  //    1: 33333
  //    2: 66667
  //    3: 100000
  //
  // Can only be used if jitter_ equals zero.
  Timestamp PeriodIndexToTimestamp(int64_t index) const;

  // Given a Timestamp, finds the closest sync Timestamp based on
  // first_timestamp_ and the desired fps.
  //
  // Can only be used if jitter_ equals zero.
  int64_t TimestampToPeriodIndex(Timestamp timestamp) const;

  // Outputs a packet if it is in range (start_time_, end_time_).
  void OutputWithinLimits(CalculatorContext* cc, const Packet& packet) const;

 protected:
  // Returns Sampling Strategy to use.
  //
  // Virtual to allow injection of testing strategies.
  virtual std::unique_ptr<class PacketResamplerStrategy> GetSamplingStrategy(
      const mediapipe::PacketResamplerCalculatorOptions& options);

 private:
  // Updates the frame rate of the calculator.
  //
  // This updates the metadata of the frame rate of the calculator moving
  // forward. All already processed packets will be ignored.
  absl::Status UpdateFrameRate(
      const mediapipe::PacketResamplerCalculatorOptions& resampler_options,
      double frame_rate);

  std::unique_ptr<class PacketResamplerStrategy> strategy_;

  // The timestamp of the first packet received.
  Timestamp first_timestamp_;

  // Number of frames per second (desired output frequency).
  double frame_rate_;

  // Inverse of frame_rate_.
  int64_t frame_time_usec_;

  VideoHeader video_header_;
  // The "DATA" input stream.
  CollectionItemId input_data_id_;
  // The "DATA" output stream.
  CollectionItemId output_data_id_;

  // Indicator whether to flush last packet even if its timestamp is greater
  // than the final stream timestamp.
  bool flush_last_packet_;

  double jitter_ = 0.0;

  int64_t jitter_usec_;

  // The last packet that was received.
  Packet last_packet_;

  // If specified, only outputs at/after start_time are included.
  Timestamp start_time_;

  // If specified, only outputs before end_time are included.
  Timestamp end_time_;

  // If set, the output timestamps nearest to start_time and end_time
  // are included in the output, even if the nearest timestamp is not
  // between start_time and end_time.
  bool round_limits_;

  // Allow strategies access to all internal calculator state.
  //
  // The calculator and strategies are intimiately tied together so this should
  // not break encapsulation.
  friend class LegacyJitterWithReflectionStrategy;
  friend class ReproducibleJitterWithReflectionStrategy;
  friend class JitterWithoutReflectionStrategy;
  friend class NoJitterStrategy;
};

// Abstract class encapsulating sampling stategy.
//
// These are used solely by PacketResamplerCalculator, but are exposed here
// to facilitate tests.
class PacketResamplerStrategy {
 public:
  PacketResamplerStrategy(PacketResamplerCalculator* calculator)
      : calculator_(calculator) {}
  virtual ~PacketResamplerStrategy() = default;

  // Delegate for CalculatorBase::Open.  See CalculatorBase for relevant
  // implementation considerations.
  virtual absl::Status Open(CalculatorContext* cc) = 0;
  // Delegate for CalculatorBase::Close.  See CalculatorBase for relevant
  // implementation considerations.
  virtual absl::Status Close(CalculatorContext* cc) = 0;
  // Delegate for CalculatorBase::Process.  See CalculatorBase for relevant
  // implementation considerations.
  virtual absl::Status Process(CalculatorContext* cc) = 0;

 protected:
  // Calculator running strategy.
  PacketResamplerCalculator* calculator_;
};

// Strategy that applies Jitter with reflection based sampling.
//
// Used by PacketResamplerCalculator when both Jitter and reflection are
// enabled.
//
// This applies the legacy jitter with reflection which doesn't allow
// for reproducibility of sampling when starting mid-stream.  This is maintained
// for backward compatibility.
class LegacyJitterWithReflectionStrategy : public PacketResamplerStrategy {
 public:
  LegacyJitterWithReflectionStrategy(PacketResamplerCalculator* calculator)
      : PacketResamplerStrategy(calculator) {}

  absl::Status Open(CalculatorContext* cc) override;
  absl::Status Close(CalculatorContext* cc) override;
  absl::Status Process(CalculatorContext* cc) override;

 private:
  void InitializeNextOutputTimestampWithJitter();
  void UpdateNextOutputTimestampWithJitter();

  // Jitter-related variables.
  std::unique_ptr<RandomBase> random_;

  // The timestamp of the first packet received.
  Timestamp first_timestamp_;

  // Next packet to be emitted.  Since packets may not align perfectly with
  // next_output_timestamp_, the closest packet will be emitted.
  Timestamp next_output_timestamp_;

  // Lower bound for next timestamp.
  //
  // next_output_timestamp_ will be kept within the interval
  // [next_output_timestamp_min_, next_output_timestamp_min_ + frame_time_usec_)
  Timestamp next_output_timestamp_min_ = Timestamp::Unset();

  // packet reservior used for sampling random packet out of partial
  // period when jitter is enabled
  std::unique_ptr<PacketReservoir> packet_reservoir_;

  // random number generator used in packet_reservior_.
  std::unique_ptr<RandomBase> packet_reservoir_random_;
};

// Strategy that applies reproducible jitter with reflection based sampling.
//
// Used by PacketResamplerCalculator when both Jitter and reflection are
// enabled.
class ReproducibleJitterWithReflectionStrategy
    : public PacketResamplerStrategy {
 public:
  ReproducibleJitterWithReflectionStrategy(
      PacketResamplerCalculator* calculator)
      : PacketResamplerStrategy(calculator) {}

  absl::Status Open(CalculatorContext* cc) override;
  absl::Status Close(CalculatorContext* cc) override;
  absl::Status Process(CalculatorContext* cc) override;

 protected:
  // Returns next random in range (0,n].
  //
  // Exposed as virtual function for testing Jitter with reflection.
  // This is the only way random_ is accessed.
  virtual uint64_t GetNextRandom(uint64_t n) {
    return random_->UnbiasedUniform64(n);
  }

 private:
  // Initializes Jitter with reflection.
  //
  // This will fast-forward to the period containing current_timestamp.
  // next_output_timestamp_ is guarnateed to be current_timestamp's period
  // and packet_emitted_this_period_ will be set to false.
  void InitializeNextOutputTimestamp(Timestamp current_timestamp);

  // Potentially advances next_output_timestamp_ a single period.
  //
  // next_output_timestamp_ will only be advanced if packet_emitted_this_period_
  // is false.  next_output_timestamp_ will never be advanced beyond
  // current_timestamp's period.
  //
  // However, next_output_timestamp_ could fall before current_timestamp's
  // period since only a single period can be advanced at a time.
  void UpdateNextOutputTimestamp(Timestamp current_timestamp);

  // Jitter-related variables.
  std::unique_ptr<RandomBase> random_;

  // Next packet to be emitted.  Since packets may not align perfectly with
  // next_output_timestamp_, the closest packet will be emitted.
  Timestamp next_output_timestamp_;

  // Lower bound for next timestamp.
  //
  // next_output_timestamp_ will be kept within the interval
  // [next_output_timestamp_min_, next_output_timestamp_min_ + frame_time_usec_)
  Timestamp next_output_timestamp_min_ = Timestamp::Unset();

  // Indicates packet was emitted for current period (i.e. the period
  // next_output_timestamp_ falls in.
  bool packet_emitted_this_period_ = false;
};

// Strategy that applies Jitter without reflection based sampling.
//
// Used by PacketResamplerCalculator when Jitter is enabled and reflection is
// not enabled.
class JitterWithoutReflectionStrategy : public PacketResamplerStrategy {
 public:
  JitterWithoutReflectionStrategy(PacketResamplerCalculator* calculator)
      : PacketResamplerStrategy(calculator) {}

  absl::Status Open(CalculatorContext* cc) override;
  absl::Status Close(CalculatorContext* cc) override;
  absl::Status Process(CalculatorContext* cc) override;

 private:
  // Calculates the first sampled timestamp that incorporates a jittering
  // offset.
  void InitializeNextOutputTimestamp();

  // Calculates the next sampled timestamp that incorporates a jittering offset.
  void UpdateNextOutputTimestamp();

  // Jitter-related variables.
  std::unique_ptr<RandomBase> random_;

  // Next packet to be emitted.  Since packets may not align perfectly with
  // next_output_timestamp_, the closest packet will be emitted.
  Timestamp next_output_timestamp_;

  // Lower bound for next timestamp.
  //
  // next_output_timestamp_ will be kept within the interval
  // [next_output_timestamp_min_, next_output_timestamp_min_ + frame_time_usec_)
  Timestamp next_output_timestamp_min_ = Timestamp::Unset();

  // packet reservior used for sampling random packet out of partial period.
  std::unique_ptr<PacketReservoir> packet_reservoir_;

  // random number generator used in packet_reservior_.
  std::unique_ptr<RandomBase> packet_reservoir_random_;
};

// Strategy that applies sampling without any jitter.
//
// Used by PacketResamplerCalculator when jitter is not enabled.
class NoJitterStrategy : public PacketResamplerStrategy {
 public:
  NoJitterStrategy(PacketResamplerCalculator* calculator)
      : PacketResamplerStrategy(calculator) {}

  absl::Status Open(CalculatorContext* cc) override;
  absl::Status Close(CalculatorContext* cc) override;
  absl::Status Process(CalculatorContext* cc) override;

 private:
  // Number of periods that have passed (= #packets sent to the output).
  int64_t period_count_;

  // If specified, output timestamps are aligned with base_timestamp.
  // Otherwise, they are aligned with the first input timestamp.
  Timestamp base_timestamp_;
};

}  // namespace mediapipe
#endif  // MEDIAPIPE_CALCULATORS_CORE_PACKET_RESAMPLER_CALCULATOR_H_
