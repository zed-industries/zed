/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_RTC_STATS_COLLECTOR_H_
#define PC_RTC_STATS_COLLECTOR_H_

#include <stdint.h>

#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/audio/audio_device.h"
#include "api/data_channel_interface.h"
#include "api/environment/environment.h"
#include "api/media_types.h"
#include "api/ref_count.h"
#include "api/rtp_transceiver_direction.h"
#include "api/scoped_refptr.h"
#include "api/stats/rtc_stats_collector_callback.h"
#include "api/stats/rtc_stats_report.h"
#include "api/units/timestamp.h"
#include "call/call.h"
#include "pc/peer_connection_internal.h"
#include "pc/rtp_receiver.h"
#include "pc/rtp_sender.h"
#include "pc/rtp_transceiver.h"
#include "pc/track_media_info_map.h"
#include "pc/transport_stats.h"
#include "rtc_base/checks.h"
#include "rtc_base/containers/flat_set.h"
#include "rtc_base/event.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/time_utils.h"

namespace webrtc {

class RtpSenderInternal;
class RtpReceiverInternal;

// All public methods of the collector are to be called on the signaling thread.
// Stats are gathered on the signaling, worker and network threads
// asynchronously. The callback is invoked on the signaling thread. Resulting
// reports are cached for `cache_lifetime_` ms.
class RTCStatsCollector : public RefCountInterface {
 public:
  static scoped_refptr<RTCStatsCollector> Create(
      PeerConnectionInternal* pc,
      const Environment& env,
      int64_t cache_lifetime_us = 50 * kNumMicrosecsPerMillisec);

  // Gets a recent stats report. If there is a report cached that is still fresh
  // it is returned, otherwise new stats are gathered and returned. A report is
  // considered fresh for `cache_lifetime_` ms. const RTCStatsReports are safe
  // to use across multiple threads and may be destructed on any thread.
  // If the optional selector argument is used, stats are filtered according to
  // stats selection algorithm before delivery.
  // https://w3c.github.io/webrtc-pc/#dfn-stats-selection-algorithm
  void GetStatsReport(scoped_refptr<RTCStatsCollectorCallback> callback);
  // If `selector` is null the selection algorithm is still applied (interpreted
  // as: no RTP streams are sent by selector). The result is empty.
  void GetStatsReport(scoped_refptr<RtpSenderInternal> selector,
                      scoped_refptr<RTCStatsCollectorCallback> callback);
  // If `selector` is null the selection algorithm is still applied (interpreted
  // as: no RTP streams are received by selector). The result is empty.
  void GetStatsReport(scoped_refptr<RtpReceiverInternal> selector,
                      scoped_refptr<RTCStatsCollectorCallback> callback);
  // Clears the cache's reference to the most recent stats report. Subsequently
  // calling `GetStatsReport` guarantees fresh stats. This method must be called
  // any time the PeerConnection visibly changes as a result of an API call as
  // per
  // https://w3c.github.io/webrtc-stats/#guidelines-for-getstats-results-caching-throttling
  // and it must be called any time negotiation happens.
  void ClearCachedStatsReport();

  // If there is a `GetStatsReport` requests in-flight, waits until it has been
  // completed. Must be called on the signaling thread.
  void WaitForPendingRequest();

  // Called by the PeerConnection instance when data channel states change.
  void OnSctpDataChannelStateChanged(int channel_id,
                                     DataChannelInterface::DataState state);

 protected:
  RTCStatsCollector(PeerConnectionInternal* pc,
                    const Environment& env,
                    int64_t cache_lifetime_us);
  ~RTCStatsCollector();

  struct CertificateStatsPair {
    std::unique_ptr<SSLCertificateStats> local;
    std::unique_ptr<SSLCertificateStats> remote;

    CertificateStatsPair Copy() const;
  };

  // Stats gathering on a particular thread. Virtual for the sake of testing.
  virtual void ProducePartialResultsOnSignalingThreadImpl(
      Timestamp timestamp,
      RTCStatsReport* partial_report);
  virtual void ProducePartialResultsOnNetworkThreadImpl(
      Timestamp timestamp,
      const std::map<std::string, TransportStats>& transport_stats_by_name,
      const std::map<std::string, CertificateStatsPair>& transport_cert_stats,
      RTCStatsReport* partial_report);

 private:
  class RequestInfo {
   public:
    enum class FilterMode { kAll, kSenderSelector, kReceiverSelector };

    // Constructs with FilterMode::kAll.
    explicit RequestInfo(scoped_refptr<RTCStatsCollectorCallback> callback);
    // Constructs with FilterMode::kSenderSelector. The selection algorithm is
    // applied even if `selector` is null, resulting in an empty report.
    RequestInfo(scoped_refptr<RtpSenderInternal> selector,
                scoped_refptr<RTCStatsCollectorCallback> callback);
    // Constructs with FilterMode::kReceiverSelector. The selection algorithm is
    // applied even if `selector` is null, resulting in an empty report.
    RequestInfo(scoped_refptr<RtpReceiverInternal> selector,
                scoped_refptr<RTCStatsCollectorCallback> callback);

    FilterMode filter_mode() const { return filter_mode_; }
    scoped_refptr<RTCStatsCollectorCallback> callback() const {
      return callback_;
    }
    scoped_refptr<RtpSenderInternal> sender_selector() const {
      RTC_DCHECK(filter_mode_ == FilterMode::kSenderSelector);
      return sender_selector_;
    }
    scoped_refptr<RtpReceiverInternal> receiver_selector() const {
      RTC_DCHECK(filter_mode_ == FilterMode::kReceiverSelector);
      return receiver_selector_;
    }

   private:
    RequestInfo(FilterMode filter_mode,
                scoped_refptr<RTCStatsCollectorCallback> callback,
                scoped_refptr<RtpSenderInternal> sender_selector,
                scoped_refptr<RtpReceiverInternal> receiver_selector);

    FilterMode filter_mode_;
    scoped_refptr<RTCStatsCollectorCallback> callback_;
    scoped_refptr<RtpSenderInternal> sender_selector_;
    scoped_refptr<RtpReceiverInternal> receiver_selector_;
  };

  void GetStatsReportInternal(RequestInfo request);

  // Structure for tracking stats about each RtpTransceiver managed by the
  // PeerConnection. This can either by a Plan B style or Unified Plan style
  // transceiver (i.e., can have 0 or many senders and receivers).
  // Some fields are copied from the RtpTransceiver/BaseChannel object so that
  // they can be accessed safely on threads other than the signaling thread.
  // If a BaseChannel is not available (e.g., if signaling has not started),
  // then `mid` and `transport_name` will be null.
  struct RtpTransceiverStatsInfo {
    scoped_refptr<RtpTransceiver> transceiver;
    webrtc::MediaType media_type;
    std::optional<std::string> mid;
    std::optional<std::string> transport_name;
    TrackMediaInfoMap track_media_info_map;
    std::optional<RtpTransceiverDirection> current_direction;
  };

  void DeliverCachedReport(scoped_refptr<const RTCStatsReport> cached_report,
                           std::vector<RequestInfo> requests);

  // Produces `RTCCertificateStats`.
  void ProduceCertificateStats_n(
      Timestamp timestamp,
      const std::map<std::string, CertificateStatsPair>& transport_cert_stats,
      RTCStatsReport* report) const;
  // Produces `RTCDataChannelStats`.
  void ProduceDataChannelStats_n(Timestamp timestamp,
                                 RTCStatsReport* report) const;
  // Produces `RTCIceCandidatePairStats` and `RTCIceCandidateStats`.
  void ProduceIceCandidateAndPairStats_n(
      Timestamp timestamp,
      const std::map<std::string, TransportStats>& transport_stats_by_name,
      const Call::Stats& call_stats,
      RTCStatsReport* report) const;
  // Produces RTCMediaSourceStats, including RTCAudioSourceStats and
  // RTCVideoSourceStats.
  void ProduceMediaSourceStats_s(Timestamp timestamp,
                                 RTCStatsReport* report) const;
  // Produces `RTCPeerConnectionStats`.
  void ProducePeerConnectionStats_s(Timestamp timestamp,
                                    RTCStatsReport* report) const;
  // Produces `RTCAudioPlayoutStats`.
  void ProduceAudioPlayoutStats_s(Timestamp timestamp,
                                  RTCStatsReport* report) const;
  // Produces `RTCInboundRtpStreamStats`, `RTCOutboundRtpStreamStats`,
  // `RTCRemoteInboundRtpStreamStats`, `RTCRemoteOutboundRtpStreamStats` and any
  // referenced `RTCCodecStats`. This has to be invoked after transport stats
  // have been created because some metrics are calculated through lookup of
  // other metrics.
  void ProduceRTPStreamStats_n(
      Timestamp timestamp,
      const std::vector<RtpTransceiverStatsInfo>& transceiver_stats_infos,
      RTCStatsReport* report) const;
  void ProduceAudioRTPStreamStats_n(Timestamp timestamp,
                                    const RtpTransceiverStatsInfo& stats,
                                    RTCStatsReport* report) const;
  void ProduceVideoRTPStreamStats_n(Timestamp timestamp,
                                    const RtpTransceiverStatsInfo& stats,
                                    RTCStatsReport* report) const;
  // Produces `RTCTransportStats`.
  void ProduceTransportStats_n(
      Timestamp timestamp,
      const std::map<std::string, TransportStats>& transport_stats_by_name,
      const std::map<std::string, CertificateStatsPair>& transport_cert_stats,
      RTCStatsReport* report) const;

  // Helper function to stats-producing functions.
  std::map<std::string, CertificateStatsPair>
  PrepareTransportCertificateStats_n(
      const std::map<std::string, TransportStats>& transport_stats_by_name);
  // The results are stored in `transceiver_stats_infos_` and `call_stats_`.
  void PrepareTransceiverStatsInfosAndCallStats_s_w_n();

  // Stats gathering on a particular thread.
  void ProducePartialResultsOnSignalingThread(Timestamp timestamp);
  void ProducePartialResultsOnNetworkThread(
      Timestamp timestamp,
      std::optional<std::string> sctp_transport_name);
  // Merges `network_report_` into `partial_report_` and completes the request.
  // This is a NO-OP if `network_report_` is null.
  void MergeNetworkReport_s();

  scoped_refptr<RTCStatsReport> CreateReportFilteredBySelector(
      bool filter_by_sender_selector,
      scoped_refptr<const RTCStatsReport> report,
      scoped_refptr<RtpSenderInternal> sender_selector,
      scoped_refptr<RtpReceiverInternal> receiver_selector);

  PeerConnectionInternal* const pc_;
  const Environment env_;
  const bool stats_timestamp_with_environment_clock_;
  Thread* const signaling_thread_;
  Thread* const worker_thread_;
  Thread* const network_thread_;

  int num_pending_partial_reports_;
  int64_t partial_report_timestamp_us_;
  // Reports that are produced on the signaling thread or the network thread are
  // merged into this report. It is only touched on the signaling thread. Once
  // all partial reports are merged this is the result of a request.
  scoped_refptr<RTCStatsReport> partial_report_;
  std::vector<RequestInfo> requests_;
  // Holds the result of ProducePartialResultsOnNetworkThread(). It is merged
  // into `partial_report_` on the signaling thread and then nulled by
  // MergeNetworkReport_s(). Thread-safety is ensured by using
  // `network_report_event_`.
  scoped_refptr<RTCStatsReport> network_report_;
  // If set, it is safe to touch the `network_report_` on the signaling thread.
  // This is reset before async-invoking ProducePartialResultsOnNetworkThread()
  // and set when ProducePartialResultsOnNetworkThread() is complete, after it
  // has updated the value of `network_report_`.
  Event network_report_event_;

  // Cleared and set in `PrepareTransceiverStatsInfosAndCallStats_s_w_n`,
  // starting out on the signaling thread, then network. Later read on the
  // network and signaling threads as part of collecting stats and finally
  // reset when the work is done. Initially this variable was added and not
  // passed around as an arguments to avoid copies. This is thread safe due to
  // how operations are sequenced and we don't start the stats collection
  // sequence if one is in progress. As a future improvement though, we could
  // now get rid of the variable and keep the data scoped within a stats
  // collection sequence.
  std::vector<RtpTransceiverStatsInfo> transceiver_stats_infos_;
  // This cache avoids having to call webrtc::SSLCertChain::GetStats(), which
  // can relatively expensive. ClearCachedStatsReport() needs to be called on
  // negotiation to ensure the cache is not obsolete.
  Mutex cached_certificates_mutex_;
  std::map<std::string, CertificateStatsPair> cached_certificates_by_transport_
      RTC_GUARDED_BY(cached_certificates_mutex_);

  Call::Stats call_stats_;

  std::optional<AudioDeviceModule::Stats> audio_device_stats_;

  // A timestamp, in microseconds, that is based on a timer that is
  // monotonically increasing. That is, even if the system clock is modified the
  // difference between the timer and this timestamp is how fresh the cached
  // report is.
  int64_t cache_timestamp_us_;
  int64_t cache_lifetime_us_;
  scoped_refptr<const RTCStatsReport> cached_report_;

  // Data recorded and maintained by the stats collector during its lifetime.
  // Some stats are produced from this record instead of other components.
  struct InternalRecord {
    InternalRecord() : data_channels_opened(0), data_channels_closed(0) {}

    // The opened count goes up when a channel is fully opened and the closed
    // count goes up if a previously opened channel has fully closed. The opened
    // count does not go down when a channel closes, meaning (opened - closed)
    // is the number of channels currently opened. A channel that is closed
    // before reaching the open state does not affect these counters.
    uint32_t data_channels_opened;
    uint32_t data_channels_closed;
    // Identifies channels that have been opened, whose internal id is stored in
    // the set until they have been fully closed.
    flat_set<int> opened_data_channels;
  };
  InternalRecord internal_record_;
};

}  // namespace webrtc

#endif  // PC_RTC_STATS_COLLECTOR_H_
