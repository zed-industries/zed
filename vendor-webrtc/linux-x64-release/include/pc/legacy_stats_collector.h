/*
 *  Copyright 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains a class used for gathering statistics from an ongoing
// libjingle PeerConnection.

#ifndef PC_LEGACY_STATS_COLLECTOR_H_
#define PC_LEGACY_STATS_COLLECTOR_H_

#include <stdint.h>

#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "api/candidate.h"
#include "api/legacy_stats_types.h"
#include "api/media_stream_interface.h"
#include "api/peer_connection_interface.h"
#include "api/scoped_refptr.h"
#include "p2p/base/connection_info.h"
#include "p2p/base/port.h"
#include "pc/legacy_stats_collector_interface.h"
#include "pc/peer_connection_internal.h"
#include "pc/rtp_transceiver.h"
#include "pc/transport_stats.h"
#include "rtc_base/network_constants.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// Conversion function to convert candidate type string to the corresponding one
// from  enum RTCStatsIceCandidateType.
const char* IceCandidateTypeToStatsType(const Candidate& candidate);

// Conversion function to convert adapter type to report string which are more
// fitting to the general style of http://w3c.github.io/webrtc-stats. This is
// only used by stats collector.
const char* AdapterTypeToStatsType(AdapterType type);

// A mapping between track ids and their StatsReport.
typedef std::map<std::string, StatsReport*> TrackIdMap;

class LegacyStatsCollector : public LegacyStatsCollectorInterface {
 public:
  // The caller is responsible for ensuring that the pc outlives the
  // LegacyStatsCollector instance.
  explicit LegacyStatsCollector(PeerConnectionInternal* pc);
  virtual ~LegacyStatsCollector();

  // Adds a MediaStream with tracks that can be used as a `selector` in a call
  // to GetStats.
  void AddStream(MediaStreamInterface* stream);
  void AddTrack(MediaStreamTrackInterface* track);

  // Adds a local audio track that is used for getting some voice statistics.
  void AddLocalAudioTrack(AudioTrackInterface* audio_track,
                          uint32_t ssrc) override;

  // Removes a local audio tracks that is used for getting some voice
  // statistics.
  void RemoveLocalAudioTrack(AudioTrackInterface* audio_track,
                             uint32_t ssrc) override;

  // Gather statistics from the session and store them for future use.
  void UpdateStats(PeerConnectionInterface::StatsOutputLevel level);

  // Gets a StatsReports of the last collected stats. Note that UpdateStats must
  // be called before this function to get the most recent stats. `selector` is
  // a track label or empty string. The most recent reports are stored in
  // `reports`.
  // TODO(tommi): Change this contract to accept a callback object instead
  // of filling in `reports`.  As is, there's a requirement that the caller
  // uses `reports` immediately without allowing any async activity on
  // the thread (message handling etc) and then discard the results.
  void GetStats(MediaStreamTrackInterface* track,
                StatsReports* reports) override;

  // Prepare a local or remote SSRC report for the given ssrc. Used internally
  // in the ExtractStatsFromList template.
  StatsReport* PrepareReport(bool local,
                             uint32_t ssrc,
                             const std::string& track_id,
                             const StatsReport::Id& transport_id,
                             StatsReport::Direction direction);

  StatsReport* PrepareADMReport();

  // A track is invalid if there is no report data for it.
  bool IsValidTrack(const std::string& track_id);

  // Reset the internal cache timestamp to force an update of the stats next
  // time UpdateStats() is called. This call needs to be made on the signaling
  // thread and should be made every time configuration changes that affect
  // stats have been made.
  void InvalidateCache();

  bool UseStandardBytesStats() const { return use_standard_bytes_stats_; }

 private:
  friend class LegacyStatsCollectorTest;

  // Struct that's populated on the network thread and carries the values to
  // the signaling thread where the stats are added to the stats reports.
  struct TransportStats {
    TransportStats() = default;
    TransportStats(std::string transport_name,
                   ::webrtc::TransportStats transport_stats)
        : name(std::move(transport_name)), stats(std::move(transport_stats)) {}
    TransportStats(TransportStats&&) = default;
    TransportStats(const TransportStats&) = delete;

    std::string name;
    ::webrtc::TransportStats stats;
    std::unique_ptr<SSLCertificateStats> local_cert_stats;
    std::unique_ptr<SSLCertificateStats> remote_cert_stats;
  };

  struct SessionStats {
    SessionStats() = default;
    SessionStats(SessionStats&&) = default;
    SessionStats(const SessionStats&) = delete;

    SessionStats& operator=(SessionStats&&) = default;
    SessionStats& operator=(SessionStats&) = delete;

    CandidateStatsList candidate_stats;
    std::vector<TransportStats> transport_stats;
    std::map<std::string, std::string> transport_names_by_mid;
  };

  // Overridden in unit tests to fake timing.
  virtual double GetTimeNow();

  bool CopySelectedReports(const std::string& selector, StatsReports* reports);

  // Helper method for creating IceCandidate report. `is_local` indicates
  // whether this candidate is local or remote.
  StatsReport* AddCandidateReport(const CandidateStats& candidate_stats,
                                  bool local);

  // Adds a report for this certificate and every certificate in its chain, and
  // returns the leaf certificate's report (`cert_stats`'s report).
  StatsReport* AddCertificateReports(
      std::unique_ptr<SSLCertificateStats> cert_stats);

  StatsReport* AddConnectionInfoReport(const std::string& content_name,
                                       int component,
                                       int connection_id,
                                       const StatsReport::Id& channel_report_id,
                                       const ConnectionInfo& info);

  void ExtractDataInfo_n(StatsCollection* reports);

  // Returns the `transport_names_by_mid` member from the SessionStats as
  // gathered and used to populate the stats. Contains one synchronous hop
  // to the network thread to get this information along with querying data
  // channel stats at the same time and populating `reports_`.
  std::map<std::string, std::string> ExtractSessionAndDataInfo();

  void ExtractBweInfo();
  void ExtractMediaInfo(
      const std::map<std::string, std::string>& transport_names_by_mid);
  void ExtractSenderInfo();
  StatsReport* GetReport(const StatsReport::StatsType& type,
                         const std::string& id,
                         StatsReport::Direction direction);

  // Helper method to get stats from the local audio tracks.
  void UpdateStatsFromExistingLocalAudioTracks(bool has_remote_tracks);
  void UpdateReportFromAudioTrack(AudioTrackInterface* track,
                                  StatsReport* report,
                                  bool has_remote_tracks);

  // Helper method to update the timestamp of track records.
  void UpdateTrackReports();

  SessionStats ExtractSessionInfo_n(
      const std::vector<scoped_refptr<
          RtpTransceiverProxyWithInternal<RtpTransceiver>>>& transceivers,
      std::optional<std::string> sctp_transport_name,
      std::optional<std::string> sctp_mid);
  void ExtractSessionInfo_s(SessionStats& session_stats);

  // A collection for all of our stats reports.
  StatsCollection reports_;
  TrackIdMap track_ids_;
  // Raw pointer to the peer connection the statistics are gathered from.
  PeerConnectionInternal* const pc_;
  int64_t cache_timestamp_ms_ RTC_GUARDED_BY(pc_->signaling_thread()) = 0;
  double stats_gathering_started_;
  const bool use_standard_bytes_stats_;

  // TODO(tommi): We appear to be holding on to raw pointers to reference
  // counted objects?  We should be using scoped_refptr here.
  typedef std::vector<std::pair<AudioTrackInterface*, uint32_t>>
      LocalAudioTrackVector;
  LocalAudioTrackVector local_audio_tracks_;
};

}  // namespace webrtc

#endif  // PC_LEGACY_STATS_COLLECTOR_H_
