/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CAPTURE_MAIN_SOURCE_WINDOWS_SINK_FILTER_DS_H_
#define MODULES_VIDEO_CAPTURE_MAIN_SOURCE_WINDOWS_SINK_FILTER_DS_H_

#include <dshow.h>

#include <atomic>
#include <memory>
#include <vector>

#include "api/sequence_checker.h"
#include "modules/video_capture/video_capture_impl.h"
#include "modules/video_capture/windows/help_functions_ds.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {
namespace videocapturemodule {
// forward declarations
class CaptureSinkFilter;

// Input pin for camera input
// Implements IMemInputPin, IPin.
class CaptureInputPin : public IMemInputPin, public IPin {
 public:
  CaptureInputPin(CaptureSinkFilter* filter);

  HRESULT SetRequestedCapability(const VideoCaptureCapability& capability);

  // Notifications from the filter.
  void OnFilterActivated();
  void OnFilterDeactivated();

 protected:
  virtual ~CaptureInputPin();

 private:
  CaptureSinkFilter* Filter() const;

  HRESULT AttemptConnection(IPin* receive_pin, const AM_MEDIA_TYPE* media_type);
  std::vector<AM_MEDIA_TYPE*> DetermineCandidateFormats(
      IPin* receive_pin,
      const AM_MEDIA_TYPE* media_type);
  void ClearAllocator(bool decommit);
  HRESULT CheckDirection(IPin* pin) const;

  // IUnknown
  STDMETHOD(QueryInterface)(REFIID riid, void** ppv) override;

  // clang-format off
  // clang isn't sure what to do with the longer STDMETHOD() function
  // declarations.

  // IPin
  STDMETHOD(Connect)(IPin* receive_pin,
                     const AM_MEDIA_TYPE* media_type) override;
  STDMETHOD(ReceiveConnection)(IPin* connector,
                               const AM_MEDIA_TYPE* media_type) override;
  STDMETHOD(Disconnect)() override;
  STDMETHOD(ConnectedTo)(IPin** pin) override;
  STDMETHOD(ConnectionMediaType)(AM_MEDIA_TYPE* media_type) override;
  STDMETHOD(QueryPinInfo)(PIN_INFO* info) override;
  STDMETHOD(QueryDirection)(PIN_DIRECTION* pin_dir) override;
  STDMETHOD(QueryId)(LPWSTR* id) override;
  STDMETHOD(QueryAccept)(const AM_MEDIA_TYPE* media_type) override;
  STDMETHOD(EnumMediaTypes)(IEnumMediaTypes** types) override;
  STDMETHOD(QueryInternalConnections)(IPin** pins, ULONG* count) override;
  STDMETHOD(EndOfStream)() override;
  STDMETHOD(BeginFlush)() override;
  STDMETHOD(EndFlush)() override;
  STDMETHOD(NewSegment)(REFERENCE_TIME start, REFERENCE_TIME stop,
                        double rate) override;

  // IMemInputPin
  STDMETHOD(GetAllocator)(IMemAllocator** allocator) override;
  STDMETHOD(NotifyAllocator)(IMemAllocator* allocator, BOOL read_only) override;
  STDMETHOD(GetAllocatorRequirements)(ALLOCATOR_PROPERTIES* props) override;
  STDMETHOD(Receive)(IMediaSample* sample) override;
  STDMETHOD(ReceiveMultiple)(IMediaSample** samples, long count,
                             long* processed) override;
  STDMETHOD(ReceiveCanBlock)() override;
  // clang-format on

  SequenceChecker main_checker_;
  SequenceChecker capture_checker_;

  VideoCaptureCapability requested_capability_ RTC_GUARDED_BY(main_checker_);
  // Accessed on the main thread when Filter()->IsStopped() (capture thread not
  // running), otherwise accessed on the capture thread.
  VideoCaptureCapability resulting_capability_;
  DWORD capture_thread_id_ = 0;
  webrtc::scoped_refptr<IMemAllocator> allocator_ RTC_GUARDED_BY(main_checker_);
  webrtc::scoped_refptr<IPin> receive_pin_ RTC_GUARDED_BY(main_checker_);
  std::atomic_bool flushing_{false};
  std::atomic_bool runtime_error_{false};
  // Holds a referenceless pointer to the owning filter, the name and
  // direction of the pin. The filter pointer can be considered const.
  PIN_INFO info_ = {};
  AM_MEDIA_TYPE media_type_ RTC_GUARDED_BY(main_checker_) = {};
};

// Implement IBaseFilter (including IPersist and IMediaFilter).
class CaptureSinkFilter : public IBaseFilter {
 public:
  CaptureSinkFilter(VideoCaptureImpl* capture_observer);

  HRESULT SetRequestedCapability(const VideoCaptureCapability& capability);

  // Called on the capture thread.
  void ProcessCapturedFrame(unsigned char* buffer,
                            size_t length,
                            const VideoCaptureCapability& frame_info);

  void NotifyEvent(long code, LONG_PTR param1, LONG_PTR param2);
  bool IsStopped() const;

  //  IUnknown
  STDMETHOD(QueryInterface)(REFIID riid, void** ppv) override;

  // IPersist
  STDMETHOD(GetClassID)(CLSID* clsid) override;

  // IMediaFilter.
  STDMETHOD(GetState)(DWORD msecs, FILTER_STATE* state) override;
  STDMETHOD(SetSyncSource)(IReferenceClock* clock) override;
  STDMETHOD(GetSyncSource)(IReferenceClock** clock) override;
  STDMETHOD(Pause)() override;
  STDMETHOD(Run)(REFERENCE_TIME start) override;
  STDMETHOD(Stop)() override;

  // IBaseFilter
  STDMETHOD(EnumPins)(IEnumPins** pins) override;
  STDMETHOD(FindPin)(LPCWSTR id, IPin** pin) override;
  STDMETHOD(QueryFilterInfo)(FILTER_INFO* info) override;
  STDMETHOD(JoinFilterGraph)(IFilterGraph* graph, LPCWSTR name) override;
  STDMETHOD(QueryVendorInfo)(LPWSTR* vendor_info) override;

 protected:
  virtual ~CaptureSinkFilter();

 private:
  SequenceChecker main_checker_;
  const webrtc::scoped_refptr<ComRefCount<CaptureInputPin>> input_pin_;
  VideoCaptureImpl* const capture_observer_;
  FILTER_INFO info_ RTC_GUARDED_BY(main_checker_) = {};
  // Set/cleared in JoinFilterGraph. The filter must be stopped (no capture)
  // at that time, so no lock is required. While the state is not stopped,
  // the sink will be used from the capture thread.
  IMediaEventSink* sink_ = nullptr;
  FILTER_STATE state_ RTC_GUARDED_BY(main_checker_) = State_Stopped;
};
}  // namespace videocapturemodule
}  // namespace webrtc
#endif  // MODULES_VIDEO_CAPTURE_MAIN_SOURCE_WINDOWS_SINK_FILTER_DS_H_
