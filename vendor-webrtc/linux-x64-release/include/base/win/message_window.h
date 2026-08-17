// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_MESSAGE_WINDOW_H_
#define BASE_WIN_MESSAGE_WINDOW_H_

#include <string>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/functional/callback.h"
#include "base/threading/thread_checker.h"
#include "base/win/windows_types.h"

// Protect against windows.h being included before this header.
#undef FindWindow

namespace base {
namespace win {

// Implements a message-only window.
class BASE_EXPORT MessageWindow {
 public:
  // Used to register a process-wide message window class.
  class WindowClass;

  // Implement this callback to handle messages received by the message window.
  // If the callback returns |false|, the first four parameters are passed to
  // DefWindowProc(). Otherwise, |*result| is returned by the window procedure.
  using MessageCallback =
      base::RepeatingCallback<bool(UINT message,
                                   WPARAM wparam,
                                   LPARAM lparam,
                                   LRESULT* result)>;  // NOLINT

  MessageWindow();

  MessageWindow(const MessageWindow&) = delete;
  MessageWindow& operator=(const MessageWindow&) = delete;

  ~MessageWindow();

  // Creates a message-only window. The incoming messages will be passed by
  // |message_callback|. |message_callback| must outlive |this|.
  bool Create(MessageCallback message_callback);

  // Same as Create() but assigns the name to the created window.
  bool CreateNamed(MessageCallback message_callback,
                   const std::wstring& window_name);

  HWND hwnd() const { return window_; }

  // Retrieves a handle of the first message-only window with matching
  // |window_name|.
  static HWND FindWindow(const std::wstring& window_name);

 private:
  // Give |WindowClass| access to WindowProc().
  friend class WindowClass;

  // Contains the actual window creation code.
  bool DoCreate(MessageCallback message_callback, const wchar_t* window_name);

  // Invoked by the OS to process incoming window messages.
  static LRESULT CALLBACK WindowProc(HWND hwnd,
                                     UINT message,
                                     WPARAM wparam,
                                     LPARAM lparam);

  // Invoked to handle messages received by the window.
  MessageCallback message_callback_;

  // Handle of the input window.
  HWND window_ = nullptr;

  THREAD_CHECKER(thread_checker_);
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_MESSAGE_WINDOW_H_
