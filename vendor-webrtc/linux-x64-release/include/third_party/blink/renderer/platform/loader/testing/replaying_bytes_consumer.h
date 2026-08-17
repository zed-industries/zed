// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_REPLAYING_BYTES_CONSUMER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_REPLAYING_BYTES_CONSUMER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/bytes_consumer.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"

namespace base {
class SingleThreadTaskRunner;
}  // namespace base

namespace blink {

// ReplayingBytesConsumer stores commands via |add| and replays the stored
// commands when read.
class ReplayingBytesConsumer final : public BytesConsumer {
 public:
  class Command final {
    DISALLOW_NEW();

   public:
    enum Name {
      kData,
      kDone,
      kError,
      kWait,
      kDataAndDone,
    };

    explicit Command(Name name) : name_(name) {}
    Command(Name name, const Vector<char>& body) : name_(name), body_(body) {}
    Command(Name name, base::span<const char> body) : name_(name) {
      body_.AppendSpan(body);
    }
    template <size_t N>
    Command(Name name, const char (&body)[N])
        : Command(name, base::span(body).template first<N - 1>()) {}
    Name GetName() const { return name_; }
    const Vector<char>& Body() const { return body_; }

   private:
    const Name name_;
    Vector<char> body_;
  };

  explicit ReplayingBytesConsumer(scoped_refptr<base::SingleThreadTaskRunner>);
  ~ReplayingBytesConsumer() override;

  // Add a command to this handle. This function must be called BEFORE
  // any BytesConsumer methods are called.
  void Add(const Command& command) { commands_.push_back(command); }

  Result BeginRead(base::span<const char>& buffer) override;
  Result EndRead(size_t read_size) override;

  void SetClient(Client*) override;
  void ClearClient() override;
  void Cancel() override;
  PublicState GetPublicState() const override;
  Error GetError() const override;
  String DebugName() const override { return "ReplayingBytesConsumer"; }

  bool IsCancelled() const { return is_cancelled_; }
  bool IsCommandsEmpty() { return commands_.empty(); }
  void TriggerOnStateChange() { client_->OnStateChange(); }

  void Trace(Visitor*) const override;

 private:
  void NotifyAsReadable(int notification_token);
  void Close();
  void MakeErrored(const Error&);

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  Member<BytesConsumer::Client> client_;
  InternalState state_ = InternalState::kWaiting;
  Deque<Command> commands_;
  size_t offset_ = 0;
  BytesConsumer::Error error_;
  int notification_token_ = 0;
  bool is_cancelled_ = false;
  bool is_in_two_phase_read_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_REPLAYING_BYTES_CONSUMER_H_
