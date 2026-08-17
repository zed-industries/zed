// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_MOCK_FILE_CHOOSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_MOCK_FILE_CHOOSER_H_

#include <utility>

#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/receiver_set.h"
#include "mojo/public/cpp/system/message_pipe.h"
#include "third_party/blink/public/mojom/choosers/file_chooser.mojom-blink-forward.h"
#include "third_party/blink/public/platform/browser_interface_broker_proxy.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

class MockFileChooser : public mojom::blink::FileChooser {
  using FileChooser = mojom::blink::FileChooser;
  using FileChooserParamsPtr = mojom::blink::FileChooserParamsPtr;

 public:
  // |reached_callback| is called when OpenFileChooser() or
  // |EnumerateChosenDirectory() is called.
  MockFileChooser(const blink::BrowserInterfaceBrokerProxy& broker,
                  base::OnceClosure reached_callback)
      : broker_(broker), reached_callback_(std::move(reached_callback)) {
    broker.SetBinderForTesting(
        FileChooser::Name_,
        WTF::BindRepeating(&MockFileChooser::BindFileChooserReceiver,
                           WTF::Unretained(this)));
  }

  ~MockFileChooser() override {
    broker_.SetBinderForTesting(FileChooser::Name_, {});
  }

  void SetQuitClosure(base::OnceClosure reached_callback) {
    reached_callback_ = std::move(reached_callback);
  }

  void ResponseOnOpenFileChooser(FileChooserFileInfoList files) {
    DCHECK(callback_)
        << "OpenFileChooser() or EnumerateChosenDirectory() should "
           "be called beforehand.";
    std::move(callback_).Run(mojom::blink::FileChooserResult::New(
        std::move(files), base::FilePath()));
    receivers_.FlushForTesting();
  }

 private:
  void BindFileChooserReceiver(mojo::ScopedMessagePipeHandle handle) {
    receivers_.Add(this, mojo::PendingReceiver<FileChooser>(std::move(handle)));
  }

  void OpenFileChooser(FileChooserParamsPtr params,
                       OpenFileChooserCallback callback) override {
    DCHECK(!callback_);
    callback_ = std::move(callback);
    if (reached_callback_)
      std::move(reached_callback_).Run();
  }

  void EnumerateChosenDirectory(
      const base::FilePath& directory_path,
      EnumerateChosenDirectoryCallback callback) override {
    DCHECK(!callback_);
    callback_ = std::move(callback);
    if (reached_callback_)
      std::move(reached_callback_).Run();
  }

  const blink::BrowserInterfaceBrokerProxy& broker_;
  mojo::ReceiverSet<FileChooser> receivers_;
  OpenFileChooserCallback callback_;
  FileChooserParamsPtr params_;
  base::OnceClosure reached_callback_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_MOCK_FILE_CHOOSER_H_
