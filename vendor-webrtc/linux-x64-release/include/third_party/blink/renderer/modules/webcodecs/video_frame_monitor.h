// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_MONITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_MONITOR_H_

#include <map>
#include <optional>
#include <string>

#include "base/synchronization/lock.h"
#include "media/base/video_frame.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"

namespace blink {

// This class helps monitor usage of media::VideoFrame objects. This class does
// not directly use VideoFrames, but references them using the ID returned by
// media::VideoFrame::unique_id.
// This class is an alternative to using media::VideoFrame destruction observers
// for cases that require monitoring to stop well before the media::VideoFrame
// is destroyed. For example, monitoring blink::VideoFrames backed by
// media::VideoFrames, where the JS-exposed blink::VideoFrames can be closed
// but the backing media::VideoFrames can continue to live outside the
// blink::VideoFrames.
// This class is a per-renderer-process singleton and relies on the fact
// that VideoFrame IDs are unique per process. That means that the same ID
// identifies the same frame regardless of the execution context.
// The motivating use case for this class is keeping track of frames coming
// from sources that have global limits in the number of in-flight frames
// allowed. Thus, frames are monitored per source. Sources are identified with
// a nonempty std::string, so any way to group frames can be used as long as a
// source ID is given. std::string is chosen over the Blink-standard WTF::String
// because:
//   1. source IDs often come from outside Blink (e.g., camera and screen
//      device IDs)
//   2. std::string is easier to use in a multithreaded environment.
// All the methods of this class can be called from any thread.
class MODULES_EXPORT VideoFrameMonitor {
 public:
  // Returns the singleton VideoFrameMonitor.
  static VideoFrameMonitor& Instance();
  VideoFrameMonitor(const VideoFrameMonitor&) = delete;
  VideoFrameMonitor& operator=(const VideoFrameMonitor&) = delete;

  // Report that a new frame with ID |frame_id| associated with the source with
  // ID |source_id| is being monitored.
  void OnOpenFrame(const std::string& source_id,
                   media::VideoFrame::ID frame_id);
  // Report that a new frame with ID |frame_id| associated with the source with
  // ID |source_id| is being monitored.
  void OnCloseFrame(const std::string& source_id,
                    media::VideoFrame::ID frame_id);
  // Reports the number of distinct monitored frames associated with
  // |source_id|.
  wtf_size_t NumFrames(const std::string& source_id);
  // Reports the reference count for the frame with ID |frame_id| associated
  // with the source with ID |source_id|.
  int NumRefs(const std::string& source_id, media::VideoFrame::ID frame_id);

  // Reports true if nothing is being monitored, false otherwise.
  bool IsEmpty();

  // This function returns a lock that can be used to lock the
  // VideoFrameMonitor so that multiple invocations to the methods below
  // (suffixed with "Locked" can be done atomically, as a single update to
  // the monitor. Locking the VideoFrameMonitor using GetLock() must be done
  // very carefully, as any invocation to any non-Locked method while the lock
  // is acquired will result in deadlock.
  // For example, blink::VideoFrame objects may be automatically monitored, so
  // they should not be created, cloned or closed while the lock is acquired.
  base::Lock& GetLock() { return lock_; }

  // The methods below can be called only when the mutex returned by GetLock()
  // has been acquired. Other than that, they are equivalent to their
  // corresponding non-locked version.
  void OnOpenFrameLocked(const std::string& source_id,
                         media::VideoFrame::ID frame_id)
      EXCLUSIVE_LOCKS_REQUIRED(GetLock());
  void OnCloseFrameLocked(const std::string& source_id,
                          media::VideoFrame::ID frame_id)
      EXCLUSIVE_LOCKS_REQUIRED(GetLock());
  wtf_size_t NumFramesLocked(const std::string& source_id)
      EXCLUSIVE_LOCKS_REQUIRED(GetLock());
  int NumRefsLocked(const std::string& source_id,
                    media::VideoFrame::ID frame_id)
      EXCLUSIVE_LOCKS_REQUIRED(GetLock());

 private:
  VideoFrameMonitor() = default;

  // key: unique ID of a frame.
  // value: reference count for the frame (among objects explicitly tracking
  //        the frame with VideoFrameMonitor).
  struct VideoFrameIDHashTraits
      : WTF::GenericHashTraits<media::VideoFrame::ID> {
    static unsigned GetHash(media::VideoFrame::ID key) {
      static_assert(std::is_same_v<decltype(key.GetUnsafeValue()), uint64_t>);
      return WTF::HashInt(key.GetUnsafeValue());
    }

    static const bool kEmptyValueIsZero = false;

    static media::VideoFrame::ID EmptyValue() {
      return media::VideoFrame::ID();
    }
    static media::VideoFrame::ID DeletedValue() {
      return media::VideoFrame::ID::FromUnsafeValue(
          std::numeric_limits<media::VideoFrame::ID::underlying_type>::max());
    }
  };
  using FrameMap = HashMap<media::VideoFrame::ID, int, VideoFrameIDHashTraits>;

  // key: ID of the source of the frames.
  // value: References to frames associated to that source.
  // Using std::map because HashMap does not directly support std::string.
  using SourceMap ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/1404327)") =
      std::map<std::string, FrameMap>;

  base::Lock lock_;
  // Contains all data for VideoFrameMonitor.
  SourceMap map_ GUARDED_BY(GetLock());
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_MONITOR_H_
