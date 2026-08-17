/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_CANDIDATE_PAIR_INTERFACE_H_
#define P2P_BASE_CANDIDATE_PAIR_INTERFACE_H_

#include "api/candidate.h"

namespace webrtc {

class CandidatePairInterface {
 public:
  virtual ~CandidatePairInterface() {}

  virtual const Candidate& local_candidate() const = 0;
  virtual const Candidate& remote_candidate() const = 0;
};

// Specific implementation of the interface, suitable for being a
// data member of other structs.
struct CandidatePair final : public CandidatePairInterface {
  ~CandidatePair() override = default;

  const Candidate& local_candidate() const override { return local; }
  const Candidate& remote_candidate() const override { return remote; }

  Candidate local;
  Candidate remote;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::CandidatePair;
using ::webrtc::CandidatePairInterface;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_CANDIDATE_PAIR_INTERFACE_H_
