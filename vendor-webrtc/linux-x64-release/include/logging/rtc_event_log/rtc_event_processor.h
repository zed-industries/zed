/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_RTC_EVENT_PROCESSOR_H_
#define LOGGING_RTC_EVENT_LOG_RTC_EVENT_PROCESSOR_H_

#include <stdint.h>

#include <functional>
#include <memory>
#include <optional>
#include <type_traits>
#include <vector>

#include "logging/rtc_event_log/rtc_event_log_parser.h"
#include "logging/rtc_event_log/rtc_event_processor_order.h"
#include "rtc_base/checks.h"

namespace webrtc {

// This file contains helper class used to process the elements of two or more
// sorted lists in timestamp order. The effect is the same as doing a merge step
// in the merge-sort algorithm but without copying the elements or modifying the
// lists.

namespace event_processor_impl {
// Interface to allow "merging" lists of different types. ProcessNext()
// processes the next unprocessed element in the list. IsEmpty() checks if all
// elements have been processed. GetNextTime returns the timestamp of the next
// unprocessed element.
class ProcessableEventListInterface {
 public:
  virtual ~ProcessableEventListInterface() = default;
  virtual void ProcessNext() = 0;
  virtual bool IsEmpty() const = 0;
  virtual int64_t GetNextTime() const = 0;
  virtual int GetTypeOrder() const = 0;
  virtual std::optional<uint16_t> GetTransportSeqNum() const = 0;
  virtual int GetInsertionOrder() const = 0;
};

// ProcessableEventList encapsulates a list of events and a function that will
// be applied to each element of the list.
template <typename Iterator, typename T>
class ProcessableEventList : public ProcessableEventListInterface {
 public:
  ProcessableEventList(Iterator begin,
                       Iterator end,
                       std::function<void(const T&)> f,
                       int type_order,
                       std::function<std::optional<uint16_t>(const T&)>
                           transport_seq_num_accessor,
                       int insertion_order)
      : begin_(begin),
        end_(end),
        f_(f),
        type_order_(type_order),
        transport_seq_num_accessor_(transport_seq_num_accessor),
        insertion_order_(insertion_order) {}

  void ProcessNext() override {
    RTC_DCHECK(!IsEmpty());
    f_(*begin_);
    ++begin_;
  }

  bool IsEmpty() const override { return begin_ == end_; }

  int64_t GetNextTime() const override {
    RTC_DCHECK(!IsEmpty());
    return begin_->log_time_us();
  }

  int GetTypeOrder() const override { return type_order_; }

  std::optional<uint16_t> GetTransportSeqNum() const override {
    RTC_DCHECK(!IsEmpty());
    return transport_seq_num_accessor_(*begin_);
  }

  int GetInsertionOrder() const override { return insertion_order_; }

 private:
  Iterator begin_;
  Iterator end_;
  std::function<void(const T&)> f_;
  int type_order_;
  std::function<std::optional<uint16_t>(const T&)> transport_seq_num_accessor_;
  int insertion_order_;
};

}  // namespace event_processor_impl

// Helper class used to "merge" two or more lists of ordered RtcEventLog events
// so that they can be treated as a single ordered list. Since the individual
// lists may have different types, we need to access the lists via pointers to
// the common base class.
//
// Usage example:
// ParsedRtcEventLogNew log;
// auto incoming_handler = [] (LoggedRtcpPacketIncoming elem) { ... };
// auto outgoing_handler = [] (LoggedRtcpPacketOutgoing elem) { ... };
//
// RtcEventProcessor processor;
// processor.AddEvents(log.incoming_rtcp_packets(),
//                     incoming_handler);
// processor.AddEvents(log.outgoing_rtcp_packets(),
//                     outgoing_handler);
// processor.ProcessEventsInOrder();
class RtcEventProcessor {
 public:
  RtcEventProcessor();
  ~RtcEventProcessor();
  // The elements of each list is processed in the index order. To process all
  // elements in all lists in timestamp order, each list needs to be sorted in
  // timestamp order prior to insertion.
  // N.B. `iterable` is not owned by RtcEventProcessor. The caller must ensure
  // that the iterable outlives RtcEventProcessor and it must not be modified
  // until processing has finished.
  template <typename Iterable>
  void AddEvents(
      const Iterable& iterable,
      std::function<void(const typename Iterable::value_type&)> handler) {
    using ValueType =
        typename std::remove_const<typename Iterable::value_type>::type;
    AddEvents(iterable, handler, TieBreaker<ValueType>::type_order,
              TieBreaker<ValueType>::transport_seq_num_accessor,
              num_insertions_);
  }

  template <typename Iterable>
  void AddEvents(
      const Iterable& iterable,
      std::function<void(const typename Iterable::value_type&)> handler,
      PacketDirection direction) {
    using ValueType =
        typename std::remove_const<typename Iterable::value_type>::type;
    AddEvents(iterable, handler, TieBreaker<ValueType>::type_order(direction),
              TieBreaker<ValueType>::transport_seq_num_accessor,
              num_insertions_);
  }

  template <typename Iterable>
  void AddEvents(
      const Iterable& iterable,
      std::function<void(const typename Iterable::value_type&)> handler,
      int type_order,
      std::function<std::optional<uint16_t>(
          const typename Iterable::value_type&)> transport_seq_num_accessor,
      int insertion_order) {
    if (iterable.begin() == iterable.end())
      return;
    num_insertions_++;
    event_lists_.push_back(
        std::make_unique<event_processor_impl::ProcessableEventList<
            typename Iterable::const_iterator, typename Iterable::value_type>>(
            iterable.begin(), iterable.end(), handler, type_order,
            transport_seq_num_accessor, insertion_order));
  }

  void ProcessEventsInOrder();

 private:
  using ListPtrType =
      std::unique_ptr<event_processor_impl::ProcessableEventListInterface>;
  // Comparison function to make `event_lists_` into a min heap.
  static bool Cmp(const ListPtrType& a, const ListPtrType& b);

  std::vector<ListPtrType> event_lists_;
  int num_insertions_ = 0;
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_RTC_EVENT_PROCESSOR_H_
