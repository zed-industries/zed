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

#ifndef MEDIAPIPE_UTIL_TRACKING_BOX_TRACKER_H_
#define MEDIAPIPE_UTIL_TRACKING_BOX_TRACKER_H_

#include <inttypes.h>

#include <map>
#include <unordered_map>
#include <vector>

#include "absl/strings/str_format.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/port/threadpool.h"
#include "mediapipe/util/tracking/box_tracker.pb.h"
#include "mediapipe/util/tracking/flow_packager.pb.h"
#include "mediapipe/util/tracking/tracking.h"
#include "mediapipe/util/tracking/tracking.pb.h"

namespace mediapipe {

// Describes a rectangle box at an instance of time.
struct TimedBox {
  static const int kNumQuadVertices = 4;
  // Dimensions are in normalized coordinates [0, 1].
  float top = 0;
  float left = 0;
  float bottom = 0;
  float right = 0;
  // Rotation of box w.r.t. center in radians.
  float rotation = 0;
  int64_t time_msec = 0;
  // Confidence of the tracked box in range [0, 1].
  float confidence = 0;
  std::vector<Vector2_f> quad_vertices;
  // Aspect ratio (width / height) for the tracked rectangle in physical space.
  float aspect_ratio = -1.0;
  // Whether we want this box to be potentially grouped with other boxes
  // to track together. This is useful for tracking small boxes that lie
  // on a plane. For example, when we detect a plane,
  // track the plane, then all boxes within the plane can share the same
  // homography transform.
  bool request_grouping = false;

  bool operator<(const TimedBox& rhs) const {
    return time_msec < rhs.time_msec;
  }

  // Returns (1.0 - alpha) * lhs + alpha * rhs;
  static TimedBox Blend(const TimedBox& lhs, const TimedBox& rhs, double alpha);

  // Returns lhs * alpha + rhs * beta;
  static TimedBox Blend(const TimedBox& lhs, const TimedBox& rhs, double alpha,
                        double beta);

  std::string ToString() const {
    return absl::StrFormat(
        "top: %.3f left: %.3f bottom: %.3f right: %.3f "
        "rot: %.3f t: %d",
        top, left, bottom, right, rotation, static_cast<int64_t>(time_msec));
  }

  // Returns corners of TimedBox in the requested domain.
  std::array<Vector2_f, 4> Corners(float width, float height) const;

  static TimedBox FromProto(const TimedBoxProto& proto);
  TimedBoxProto ToProto() const;
};

// TimedBox augment with internal states.
struct InternalTimedBox : public TimedBox {
  InternalTimedBox() = default;
  // Convenience constructor.
  InternalTimedBox(const TimedBox& box, const MotionBoxState* state_)
      : TimedBox(box), state(state_) {}

  // Corresponding MotionBoxState a TimedBox.
  std::shared_ptr<const MotionBoxState> state;
};

// Initializes beginning tracking state from a TimedBox.
void MotionBoxStateFromTimedBox(const TimedBox& box, MotionBoxState* state);

// Retrieves box position and time from a tracking state.
void TimedBoxFromMotionBoxState(const MotionBoxState& state, TimedBox* box);

// Downgrade tracking degrees of TrackStepOptions to
// TRACKING_DEGREE_OBJECT_ROTATION_SCALE if originally
// specified TRACKING_DEGREE_OBJECT_PERSPECTIVE, but start pos
// does not contain quad or contains invalid quad.
template <typename T>
void ChangeTrackingDegreesBasedOnStartPos(
    const T& start_pos, TrackStepOptions* track_step_options) {
  if (track_step_options->tracking_degrees() ==
          TrackStepOptions::TRACKING_DEGREE_OBJECT_PERSPECTIVE &&
      (!start_pos.has_quad() || start_pos.quad().vertices_size() != 8)) {
    track_step_options->set_tracking_degrees(
        TrackStepOptions::TRACKING_DEGREE_OBJECT_ROTATION_SCALE);
    VLOG(1) << "Originally specified TRACKING_DEGREE_OBJECT_PERSPECTIVE, but "
               "changed to TRACKING_DEGREE_OBJECT_ROTATION_SCALE";
  }
}

// Set of boxes for a particular checkpoint.
typedef std::deque<InternalTimedBox> PathSegment;

// Stores the PathSegment for each checkpoint time.
typedef std::map<int, PathSegment> Path;

// Returns box at specified time for a specific path segment.
// Returns true on success, otherwise box is left untouched.
// Optionally returns closest known MotionBoxState if state is not null.
bool TimedBoxAtTime(const PathSegment& segment, int64_t time_msec,
                    TimedBox* box, MotionBoxState* state = nullptr);

// Tracks timed boxes from cached TrackingDataChunks created by
// FlowPackagerCalculator. For usage see accompanying test.
class BoxTracker {
 public:
  // Initializes a new BoxTracker to work on cached TrackingData from a chunk
  // directory.
  BoxTracker(const std::string& cache_dir, const BoxTrackerOptions& options);

  // Initializes a new BoxTracker to work on the passed TrackingDataChunks.
  // If copy_data is true, BoxTracker will retain its own copy of the data;
  // otherwise the passed pointer need to be valid for the lifetime of the
  // BoxTracker.
  BoxTracker(const std::vector<const TrackingDataChunk*>& tracking_data,
             bool copy_data, const BoxTrackerOptions& options);

  // Add single TrackingDataChunk. This chunk must be correctly aligned with
  // existing chunks. If chunk starting timestamp is larger than next valid
  // chunk timestamp, empty chunks will be added to fill the gap. If copy_data
  // is true, BoxTracker will retain its own copy of the data; otherwise the
  // passed pointers need to be valid for the lifetime of the BoxTracker.
  void AddTrackingDataChunk(const TrackingDataChunk* chunk, bool copy_data);
  // Add new TrackingDataChunks. This method allows to streamline BoxTracker
  // usage. Instead of collecting all tracking chunks and then calling the
  // constructor, it makes possible to pass only minimally required chunks to
  // constructor, and then append new chunks when available.
  // Caller is responsible to guarantee that all chunks that are necessary for
  // tracking are at the disposal of BoxTracker before calling NewBoxTrack, i.e.
  // min_msec and max_msec parameters must be in the range of chunks passed to
  // BoxTracker.
  // Chunks will be added in the order they are specified in tracking_data. Each
  // chunk must be correctly aligned with existing chunks. If chunk starting
  // timestamp is larger than next valid chunk timestamp, empty chunks will be
  // added to fill the gap. If copy_data is true, BoxTracker will retain its own
  // copy of the data; otherwise the passed pointers need to be valid for the
  // lifetime of the BoxTracker.
  void AddTrackingDataChunks(
      const std::vector<const TrackingDataChunk*>& tracking_data,
      bool copy_data);

  // Starts a new track for the specified box until tracking terminates or
  // until track length achieves max_length.
  // Does not block caller, returns immediately.
  // Note: Use positive integers for id, we reserve negative ones for debugging
  // and visualization purposes.
  void NewBoxTrack(const TimedBox& initial_pos, int id, int64_t min_msec = 0,
                   int64_t max_msec = std::numeric_limits<int64_t>::max());

  // Returns interval for which the state of the specified box is known.
  // (Returns -1, -1 if id is missing or no tracking has been done).
  std::pair<int64_t, int64_t> TrackInterval(int id);

  // Returns box position for requested box at specified time.
  // Returns false if no such box exists.
  // Optional, if record_path_states is enabled in options outputs the
  // corresponding states for the resulting TimedBox. Note, as TimedBoxes are
  // generally interpolated from the underlying data and blended across
  // checkpoints, the returned states are the closest (snapped) known tracking
  // states for the left and right checkpoint path. Therefore the size of states
  // is either two or one (if only one checkpoint exists).
  bool GetTimedPosition(int id, int64_t time_msec, TimedBox* result,
                        std::vector<MotionBoxState>* states = nullptr);

  // Returns chunk index for specified time.
  int ChunkIdxFromTime(int64_t msec) const {
    return msec / options_.caching_chunk_size_msec();
  }

  // Returns true if any tracking is ongoing for the specified id.
  bool IsTrackingOngoingForId(int id) ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Returns true if any tracking is ongoing.
  bool IsTrackingOngoing() ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Cancels all ongoing tracks. To avoid race conditions all NewBoxTrack's in
  // flight will also be canceled. Future NewBoxTrack's will be canceled.
  // NOTE: To resume execution, you have to call ResumeTracking() before
  //       issuing more NewBoxTrack calls.
  void CancelAllOngoingTracks() ABSL_LOCKS_EXCLUDED(status_mutex_);
  void ResumeTracking() ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Waits for all ongoing tracks to complete.
  // Optionally accepts a timeout in microseconds (== 0 for infinite wait).
  // Returns true on success, false if timeout is reached.
  // NOTE: If WaitForAllOngoingTracks timed out, CancelAllOngoingTracks() must
  // be called before destructing the BoxTracker object or dangeling running
  // threads might try to access invalid data.
  bool WaitForAllOngoingTracks(int timeout_us = 0)
      ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Debug function to obtain raw TrackingData closest to the specified
  // timestamp. This call will read from disk on every invocation so it is
  // expensive.
  // To not interfere with other tracking requests it is recommended that you
  // use a unique id here.
  // Returns true on success.
  bool GetTrackingData(int id, int64_t request_time_msec,
                       TrackingData* tracking_data,
                       int* tracking_data_msec = nullptr);

 private:
  // Asynchronous implementation function for box tracking. Schedules forward
  // and backward tracking.
  void NewBoxTrackAsync(const TimedBox& initial_pos, int id, int64_t min_msec,
                        int64_t max_msec);

  typedef std::pair<const TrackingDataChunk*, bool> AugmentedChunkPtr;
  // Attempts to read chunk at chunk_idx if it exists. Reads from cache
  // directory or from in memory cache.
  // Important: 2nd part of return value indicates if returned tracking data
  // will be owned by the caller (if true). In that case caller is responsible
  // for releasing the returned chunk.
  AugmentedChunkPtr ReadChunk(int id, int checkpoint, int chunk_idx);

  // Attempts to read specified chunk from caching directory. Blocks and waits
  // until chunk is available or internal time out is reached.
  // Returns nullptr if data could not be read.
  std::unique_ptr<TrackingDataChunk> ReadChunkFromCache(int id, int checkpoint,
                                                        int chunk_idx);

  // Waits with timeout for chunkfile to become available. Returns true on
  // success, false if waited till timeout or when canceled.
  bool WaitForChunkFile(int id, int checkpoint, const std::string& chunk_file)
      ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Determines closest index in passed TrackingDataChunk
  int ClosestFrameIndex(int64_t msec, const TrackingDataChunk& chunk) const;

  // Adds new TimedBox to specified checkpoint with state.
  void AddBoxResult(const TimedBox& box, int id, int checkpoint,
                    const MotionBoxState& state);

  // Callback can only handle 5 args max.
  // Set own_data to true for args to assume ownership.
  struct TrackingImplArgs {
    TrackingImplArgs(AugmentedChunkPtr chunk_ptr,
                     const MotionBoxState& start_state_, int start_frame_,
                     int chunk_idx_, int id_, int checkpoint_, bool forward_,
                     bool first_call_, int64_t min_msec_, int64_t max_msec_)
        : start_state(start_state_),
          start_frame(start_frame_),
          chunk_idx(chunk_idx_),
          id(id_),
          checkpoint(checkpoint_),
          forward(forward_),
          first_call(first_call_),
          min_msec(min_msec_),
          max_msec(max_msec_) {
      if (chunk_ptr.second) {
        chunk_data_buffer.reset(chunk_ptr.first);
      }

      chunk_data = chunk_ptr.first;
    }

    TrackingImplArgs(const TrackingImplArgs&) = default;

    // Storage for tracking data.
    std::shared_ptr<const TrackingDataChunk> chunk_data_buffer;

    // Pointer to the actual tracking data. Usually points to the buffer
    // but can also point to external data for performance reasons.
    const TrackingDataChunk* chunk_data;

    MotionBoxState start_state;
    int start_frame;
    int chunk_idx;
    int id;
    int checkpoint;
    bool forward = true;
    bool first_call = true;
    int64_t min_msec;  // minimum timestamp to track to
    int64_t max_msec;  // maximum timestamp to track to
  };

  // Actual tracking algorithm.
  void TrackingImpl(const TrackingImplArgs& args);

  // Ids are scheduled exclusively, run this method to acquire lock.
  // Returns false if id could not be scheduled (e.g. id got canceled during
  // waiting).
  bool WaitToScheduleId(int id) ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Signals end of scheduling phase. Requires status mutex to be held.
  void DoneSchedulingId(int id) ABSL_EXCLUSIVE_LOCKS_REQUIRED(status_mutex_);

  // Removes all checkpoints within vicinity of new checkpoint.
  void RemoveCloseCheckpoints(int id, int checkpoint)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(status_mutex_);

  // Removes specific checkpoint.
  void ClearCheckpoint(int id, int checkpoint)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(status_mutex_);

  // Terminates tracking for specific id and checkpoint.
  void CancelTracking(int id, int checkpoint)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(status_mutex_);

  // Implementation function for IsTrackingOngoing assuming mutex is already
  // held.
  bool IsTrackingOngoingMutexHeld()
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(status_mutex_);

  // Captures tracking status for each checkpoint
  struct TrackStatus {
    // Indicates that all current tracking processes should be canceled.
    bool canceled = false;

    // Number of tracking requests than are currently ongoing.
    int tracks_ongoing = 0;
  };

 private:
  // Stores computed tracking paths_ for all boxes.
  std::unordered_map<int, Path> paths_ ABSL_GUARDED_BY(path_mutex_);
  absl::Mutex path_mutex_;

  // For each id and each checkpoint stores current tracking status.
  std::unordered_map<int, std::map<int, TrackStatus>> track_status_
      ABSL_GUARDED_BY(status_mutex_);

  // Keeps track which ids are currently processing in NewBoxTrack.
  std::unordered_map<int, bool> new_box_track_ ABSL_GUARDED_BY(status_mutex_);
  absl::Mutex status_mutex_;

  bool canceling_ ABSL_GUARDED_BY(status_mutex_) = false;

  // Use to signal changes to status_condvar_;
  absl::CondVar status_condvar_ ABSL_GUARDED_BY(status_mutex_);

  BoxTrackerOptions options_;

  // Caching directory for TrackingData stored on disk.
  std::string cache_dir_;

  // Pointers to tracking data stored in memory.
  std::vector<const TrackingDataChunk*> tracking_data_;
  // Buffer for tracking data in case we retain a deep copy.
  std::vector<std::unique_ptr<TrackingDataChunk>> tracking_data_buffer_;

  // Workers that run the tracking algorithm.
  std::unique_ptr<ThreadPool> tracking_workers_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_BOX_TRACKER_H_
