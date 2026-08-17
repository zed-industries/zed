/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VIDEO_ENCODER_H_
#define API_VIDEO_CODECS_VIDEO_ENCODER_H_

#include <cstddef>
#include <cstdint>
#include <limits>
#include <optional>
#include <string>
#include <vector>

#include "absl/container/inlined_vector.h"
#include "api/fec_controller_override.h"
#include "api/units/data_rate.h"
#include "api/video/encoded_image.h"
#include "api/video/video_bitrate_allocation.h"
#include "api/video/video_codec_constants.h"
#include "api/video/video_frame.h"
#include "api/video/video_frame_buffer.h"
#include "api/video/video_frame_type.h"
#include "api/video_codecs/video_codec.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// TODO(pbos): Expose these through a public (root) header or change these APIs.
struct CodecSpecificInfo;

constexpr int kDefaultMinPixelsPerFrame = 320 * 180;

class RTC_EXPORT EncodedImageCallback {
 public:
  virtual ~EncodedImageCallback() {}

  struct Result {
    enum Error {
      OK,

      // Failed to send the packet.
      ERROR_SEND_FAILED,
    };

    explicit Result(Error error) : error(error) {}
    Result(Error error, uint32_t frame_id) : error(error), frame_id(frame_id) {}

    Error error;

    // Frame ID assigned to the frame. The frame ID should be the same as the ID
    // seen by the receiver for this frame. RTP timestamp of the frame is used
    // as frame ID when RTP is used to send video. Must be used only when
    // error=OK.
    uint32_t frame_id = 0;

    // Tells the encoder that the next frame is should be dropped.
    bool drop_next_frame = false;
  };

  // Used to signal the encoder about reason a frame is dropped.
  // kDroppedByMediaOptimizations - dropped by MediaOptimizations (for rate
  // limiting purposes).
  // kDroppedByEncoder - dropped by encoder's internal rate limiter.
  // TODO(bugs.webrtc.org/10164): Delete this enum? It duplicates the more
  // general VideoStreamEncoderObserver::DropReason. Also,
  // kDroppedByMediaOptimizations is not produced by any encoder, but by
  // VideoStreamEncoder.
  enum class DropReason : uint8_t {
    kDroppedByMediaOptimizations,
    kDroppedByEncoder
  };

  // Callback function which is called when an image has been encoded.
  virtual Result OnEncodedImage(
      const EncodedImage& encoded_image,
      const CodecSpecificInfo* codec_specific_info) = 0;

  virtual void OnDroppedFrame(DropReason /* reason */) {}
};

class RTC_EXPORT VideoEncoder {
 public:
  struct QpThresholds {
    QpThresholds(int l, int h) : low(l), high(h) {}
    QpThresholds() : low(-1), high(-1) {}
    int low;
    int high;
  };

  // Quality scaling is enabled if thresholds are provided.
  struct RTC_EXPORT ScalingSettings {
   private:
    // Private magic type for kOff, implicitly convertible to
    // ScalingSettings.
    struct KOff {};

   public:
    // TODO(bugs.webrtc.org/9078): Since std::optional should be trivially copy
    // constructible, this magic value can likely be replaced by a constexpr
    // ScalingSettings value.
    static constexpr KOff kOff = {};

    ScalingSettings(int low, int high);
    ScalingSettings(int low, int high, int min_pixels);
    ScalingSettings(const ScalingSettings&);
    ScalingSettings(KOff);  // NOLINT(runtime/explicit)
    ~ScalingSettings();

    std::optional<QpThresholds> thresholds;

    // We will never ask for a resolution lower than this.
    // TODO(kthelgason): Lower this limit when better testing
    // on MediaCodec and fallback implementations are in place.
    // See https://bugs.chromium.org/p/webrtc/issues/detail?id=7206
    int min_pixels_per_frame = kDefaultMinPixelsPerFrame;

   private:
    // Private constructor; to get an object without thresholds, use
    // the magic constant ScalingSettings::kOff.
    ScalingSettings();
  };

  // Bitrate limits for resolution.
  struct ResolutionBitrateLimits {
    ResolutionBitrateLimits(int frame_size_pixels,
                            int min_start_bitrate_bps,
                            int min_bitrate_bps,
                            int max_bitrate_bps)
        : frame_size_pixels(frame_size_pixels),
          min_start_bitrate_bps(min_start_bitrate_bps),
          min_bitrate_bps(min_bitrate_bps),
          max_bitrate_bps(max_bitrate_bps) {}
    // Size of video frame, in pixels, the bitrate thresholds are intended for.
    int frame_size_pixels = 0;
    // Recommended minimum bitrate to start encoding.
    int min_start_bitrate_bps = 0;
    // Recommended minimum bitrate.
    int min_bitrate_bps = 0;
    // Recommended maximum bitrate.
    int max_bitrate_bps = 0;

    bool operator==(const ResolutionBitrateLimits& rhs) const;
    bool operator!=(const ResolutionBitrateLimits& rhs) const {
      return !(*this == rhs);
    }
  };

  // Struct containing metadata about the encoder implementing this interface.
  struct RTC_EXPORT EncoderInfo {
    static constexpr uint8_t kMaxFramerateFraction =
        std::numeric_limits<uint8_t>::max();

    EncoderInfo();
    EncoderInfo(const EncoderInfo&);

    ~EncoderInfo();

    std::string ToString() const;
    bool operator==(const EncoderInfo& rhs) const;
    bool operator!=(const EncoderInfo& rhs) const { return !(*this == rhs); }

    // Any encoder implementation wishing to use the WebRTC provided
    // quality scaler must populate this field.
    ScalingSettings scaling_settings;

    // The width and height of the incoming video frames should be divisible
    // by `requested_resolution_alignment`. If they are not, the encoder may
    // drop the incoming frame.
    // For example: With I420, this value would be a multiple of 2.
    // Note that this field is unrelated to any horizontal or vertical stride
    // requirements the encoder has on the incoming video frame buffers.
    uint32_t requested_resolution_alignment;

    // Same as above but if true, each simulcast layer should also be divisible
    // by `requested_resolution_alignment`.
    // Note that scale factors `scale_resolution_down_by` may be adjusted so a
    // common multiple is not too large to avoid largely cropped frames and
    // possibly with an aspect ratio far from the original.
    // Warning: large values of scale_resolution_down_by could be changed
    // considerably, especially if `requested_resolution_alignment` is large.
    bool apply_alignment_to_all_simulcast_layers;

    // If true, encoder supports working with a native handle (e.g. texture
    // handle for hw codecs) rather than requiring a raw I420 buffer.
    bool supports_native_handle;

    // The name of this particular encoder implementation, e.g. "libvpx".
    std::string implementation_name;

    // If this field is true, the encoder rate controller must perform
    // well even in difficult situations, and produce close to the specified
    // target bitrate seen over a reasonable time window, drop frames if
    // necessary in order to keep the rate correct, and react quickly to
    // changing bitrate targets. If this method returns true, we disable the
    // frame dropper in the media optimization module and rely entirely on the
    // encoder to produce media at a bitrate that closely matches the target.
    // Any overshooting may result in delay buildup. If this method returns
    // false (default behavior), the media opt frame dropper will drop input
    // frames if it suspect encoder misbehavior. Misbehavior is common,
    // especially in hardware codecs. Disable media opt at your own risk.
    bool has_trusted_rate_controller;

    // If this field is true, the encoder uses hardware support and different
    // thresholds will be used in CPU adaptation.
    bool is_hardware_accelerated;

    // For each spatial layer (simulcast stream or SVC layer), represented as an
    // element in `fps_allocation` a vector indicates how many temporal layers
    // the encoder is using for that spatial layer.
    // For each spatial/temporal layer pair, the frame rate fraction is given as
    // an 8bit unsigned integer where 0 = 0% and 255 = 100%.
    //
    // If the vector is empty for a given spatial layer, it indicates that frame
    // rates are not defined and we can't count on any specific frame rate to be
    // generated. Likely this indicates Vp8TemporalLayersType::kBitrateDynamic.
    //
    // The encoder may update this on a per-frame basis in response to both
    // internal and external signals.
    //
    // Spatial layers are treated independently, but temporal layers are
    // cumulative. For instance, if:
    //   fps_allocation[0][0] = kMaxFramerateFraction / 2;
    //   fps_allocation[0][1] = kMaxFramerateFraction;
    // Then half of the frames are in the base layer and half is in TL1, but
    // since TL1 is assumed to depend on the base layer, the frame rate is
    // indicated as the full 100% for the top layer.
    //
    // Defaults to a single spatial layer containing a single temporal layer
    // with a 100% frame rate fraction.
    absl::InlinedVector<uint8_t, kMaxTemporalStreams>
        fps_allocation[kMaxSpatialLayers];

    // Recommended bitrate limits for different resolutions.
    std::vector<ResolutionBitrateLimits> resolution_bitrate_limits;

    // Obtains the limits from `resolution_bitrate_limits` that best matches the
    // `frame_size_pixels`.
    std::optional<ResolutionBitrateLimits> GetEncoderBitrateLimitsForResolution(
        int frame_size_pixels) const;

    // If true, this encoder has internal support for generating simulcast
    // streams. Otherwise, an adapter class will be needed.
    // Even if true, the config provided to InitEncode() might not be supported,
    // in such case the encoder should return
    // WEBRTC_VIDEO_CODEC_ERR_SIMULCAST_PARAMETERS_NOT_SUPPORTED.
    bool supports_simulcast;

    // The list of pixel formats preferred by the encoder. It is assumed that if
    // the list is empty and supports_native_handle is false, then {I420} is the
    // preferred pixel format. The order of the formats does not matter.
    absl::InlinedVector<VideoFrameBuffer::Type, kMaxPreferredPixelFormats>
        preferred_pixel_formats;

    // Indicates whether or not QP value encoder writes into frame/slice/tile
    // header can be interpreted as average frame/slice/tile QP.
    std::optional<bool> is_qp_trusted;

    // The minimum QP that the encoder is expected to use with the current
    // configuration. This may be used to determine if the encoder has reached
    // its target video quality for static screenshare content.
    std::optional<int> min_qp;
  };

  struct RTC_EXPORT RateControlParameters {
    RateControlParameters();
    RateControlParameters(const VideoBitrateAllocation& bitrate,
                          double framerate_fps);
    RateControlParameters(const VideoBitrateAllocation& bitrate,
                          double framerate_fps,
                          DataRate bandwidth_allocation);
    virtual ~RateControlParameters();

    // Target bitrate, per spatial/temporal layer.
    // A target bitrate of 0bps indicates a layer should not be encoded at all.
    VideoBitrateAllocation target_bitrate;
    // Adjusted target bitrate, per spatial/temporal layer. May be lower or
    // higher than the target depending on encoder behaviour.
    VideoBitrateAllocation bitrate;
    // Target framerate, in fps. A value <= 0.0 is invalid and should be
    // interpreted as framerate target not available. In this case the encoder
    // should fall back to the max framerate specified in `codec_settings` of
    // the last InitEncode() call.
    double framerate_fps;
    // The network bandwidth available for video. This is at least
    // `bitrate.get_sum_bps()`, but may be higher if the application is not
    // network constrained.
    DataRate bandwidth_allocation;

    bool operator==(const RateControlParameters& rhs) const;
    bool operator!=(const RateControlParameters& rhs) const;
  };

  struct LossNotification {
    // The timestamp of the last decodable frame *prior* to the last received.
    // (The last received - described below - might itself be decodable or not.)
    uint32_t timestamp_of_last_decodable;
    // The timestamp of the last received frame.
    uint32_t timestamp_of_last_received;
    // Describes whether the dependencies of the last received frame were
    // all decodable.
    // `false` if some dependencies were undecodable, `true` if all dependencies
    // were decodable, and `nullopt` if the dependencies are unknown.
    std::optional<bool> dependencies_of_last_received_decodable;
    // Describes whether the received frame was decodable.
    // `false` if some dependency was undecodable or if some packet belonging
    // to the last received frame was missed.
    // `true` if all dependencies were decodable and all packets belonging
    // to the last received frame were received.
    // `nullopt` if no packet belonging to the last frame was missed, but the
    // last packet in the frame was not yet received.
    std::optional<bool> last_received_decodable;
  };

  // Negotiated capabilities which the VideoEncoder may expect the other
  // side to use.
  struct Capabilities {
    explicit Capabilities(bool loss_notification)
        : loss_notification(loss_notification) {}
    bool loss_notification;
  };

  struct Settings {
    Settings(const Capabilities& capabilities,
             int number_of_cores,
             size_t max_payload_size)
        : capabilities(capabilities),
          number_of_cores(number_of_cores),
          max_payload_size(max_payload_size) {}

    Capabilities capabilities;
    int number_of_cores;
    size_t max_payload_size;
    // Experimental API - currently only supported by LibvpxVp8Encoder and
    // the OpenH264 encoder. If set, limits the number of encoder threads.
    std::optional<int> encoder_thread_limit;
  };

  static VideoCodecVP8 GetDefaultVp8Settings();
  static VideoCodecVP9 GetDefaultVp9Settings();
  static VideoCodecH264 GetDefaultH264Settings();

  virtual ~VideoEncoder() {}

  // Set a FecControllerOverride, through which the encoder may override
  // decisions made by FecController.
  // TODO(bugs.webrtc.org/10769): Update downstream, then make pure-virtual.
  virtual void SetFecControllerOverride(
      FecControllerOverride* fec_controller_override);

  // Initialize the encoder with the information from the codecSettings
  //
  // Input:
  //          - codec_settings    : Codec settings
  //          - settings          : Settings affecting the encoding itself.
  // Input for deprecated version:
  //          - number_of_cores   : Number of cores available for the encoder
  //          - max_payload_size  : The maximum size each payload is allowed
  //                                to have. Usually MTU - overhead.
  //
  // Return value                  : Set bit rate if OK
  //                                 <0 - Errors:
  //                                  WEBRTC_VIDEO_CODEC_ERR_PARAMETER
  //                                  WEBRTC_VIDEO_CODEC_ERR_SIZE
  //                                  WEBRTC_VIDEO_CODEC_MEMORY
  //                                  WEBRTC_VIDEO_CODEC_ERROR
  // TODO(bugs.webrtc.org/10720): After updating downstream projects and posting
  // an announcement to discuss-webrtc, remove the three-parameters variant
  // and make the two-parameters variant pure-virtual.
  /* ABSL_DEPRECATED("bugs.webrtc.org/10720") */ virtual int32_t InitEncode(
      const VideoCodec* codec_settings,
      int32_t number_of_cores,
      size_t max_payload_size);
  virtual int InitEncode(const VideoCodec* codec_settings,
                         const VideoEncoder::Settings& settings);

  // Register an encode complete callback object.
  //
  // Input:
  //          - callback         : Callback object which handles encoded images.
  //
  // Return value                : WEBRTC_VIDEO_CODEC_OK if OK, < 0 otherwise.
  virtual int32_t RegisterEncodeCompleteCallback(
      EncodedImageCallback* callback) = 0;

  // Free encoder memory.
  // Return value                : WEBRTC_VIDEO_CODEC_OK if OK, < 0 otherwise.
  virtual int32_t Release() = 0;

  // Encode an image (as a part of a video stream). The encoded image
  // will be returned to the user through the encode complete callback.
  //
  // Input:
  //          - frame             : Image to be encoded
  //          - frame_types       : Frame type to be generated by the encoder.
  //
  // Return value                 : WEBRTC_VIDEO_CODEC_OK if OK
  //                                <0 - Errors:
  //                                  WEBRTC_VIDEO_CODEC_ERR_PARAMETER
  //                                  WEBRTC_VIDEO_CODEC_MEMORY
  //                                  WEBRTC_VIDEO_CODEC_ERROR
  virtual int32_t Encode(const VideoFrame& frame,
                         const std::vector<VideoFrameType>* frame_types) = 0;

  // Sets rate control parameters: bitrate, framerate, etc. These settings are
  // instantaneous (i.e. not moving averages) and should apply from now until
  // the next call to SetRates().
  virtual void SetRates(const RateControlParameters& parameters) = 0;

  // Inform the encoder when the packet loss rate changes.
  //
  // Input:   - packet_loss_rate  : The packet loss rate (0.0 to 1.0).
  virtual void OnPacketLossRateUpdate(float packet_loss_rate);

  // Inform the encoder when the round trip time changes.
  //
  // Input:   - rtt_ms            : The new RTT, in milliseconds.
  virtual void OnRttUpdate(int64_t rtt_ms);

  // Called when a loss notification is received.
  virtual void OnLossNotification(const LossNotification& loss_notification);

  // Returns meta-data about the encoder, such as implementation name.
  // The output of this method may change during runtime. For instance if a
  // hardware encoder fails, it may fall back to doing software encoding using
  // an implementation with different characteristics.
  virtual EncoderInfo GetEncoderInfo() const = 0;
};
}  // namespace webrtc
#endif  // API_VIDEO_CODECS_VIDEO_ENCODER_H_
