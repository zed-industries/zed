// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MOCK_CLIPBOARD_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MOCK_CLIPBOARD_HOST_H_

#include "build/build_config.h"
#include "mojo/public/cpp/bindings/receiver_set.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/clipboard/clipboard.mojom-blink.h"
#include "third_party/skia/include/core/SkBitmap.h"

namespace blink {

class MockClipboardHost : public mojom::blink::ClipboardHost {
 public:
  MockClipboardHost();
  MockClipboardHost(const MockClipboardHost&) = delete;
  MockClipboardHost& operator=(const MockClipboardHost&) = delete;
  ~MockClipboardHost() override;

  void Bind(mojo::PendingReceiver<mojom::blink::ClipboardHost> receiver);
  // Clears all clipboard data.
  void Reset();

  // These write methods exist only in the mock class because
  // mojom::ClipboardHost does not provide equivalent methods.  These are here
  // to simplify testing of the system clipboard.
  void WriteRtf(const String& rtf_text);
  void WriteFiles(mojom::blink::ClipboardFilesPtr files);

  // Method to simulate clipboard data change only for testing.
  void OnClipboardDataChanged();

 private:
  // mojom::ClipboardHost
  void GetSequenceNumber(mojom::ClipboardBuffer clipboard_buffer,
                         GetSequenceNumberCallback callback) override;
  void IsFormatAvailable(mojom::ClipboardFormat format,
                         mojom::ClipboardBuffer clipboard_buffer,
                         IsFormatAvailableCallback callback) override;
  void ReadAvailableTypes(mojom::ClipboardBuffer clipboard_buffer,
                          ReadAvailableTypesCallback callback) override;
  void ReadText(mojom::ClipboardBuffer clipboard_buffer,
                ReadTextCallback callback) override;
  void ReadHtml(mojom::ClipboardBuffer clipboard_buffer,
                ReadHtmlCallback callback) override;
  void ReadSvg(mojom::ClipboardBuffer clipboard_buffer,
               ReadSvgCallback callback) override;
  void ReadRtf(mojom::ClipboardBuffer clipboard_buffer,
               ReadRtfCallback callback) override;
  void ReadPng(mojom::ClipboardBuffer clipboard_buffer,
               ReadPngCallback callback) override;
  void ReadFiles(mojom::ClipboardBuffer clipboard_buffer,
                 ReadFilesCallback callback) override;
  void ReadDataTransferCustomData(
      mojom::ClipboardBuffer clipboard_buffer,
      const String& type,
      ReadDataTransferCustomDataCallback callback) override;
  void WriteText(const String& text) override;
  void WriteHtml(const String& markup, const KURL& url) override;
  void WriteSvg(const String& markup) override;
  void WriteSmartPasteMarker() override;
  void WriteDataTransferCustomData(
      const HashMap<String, String>& data) override;
  void WriteBookmark(const String& url, const String& title) override;
  void WriteImage(const SkBitmap& bitmap) override;
  void CommitWrite() override;
  void ReadAvailableCustomAndStandardFormats(
      ReadAvailableCustomAndStandardFormatsCallback callback) override;
  void ReadUnsanitizedCustomFormat(
      const String& format,
      ReadUnsanitizedCustomFormatCallback callback) override;
  void WriteUnsanitizedCustomFormat(const String& format,
                                    mojo_base::BigBuffer data) override;
  void RegisterClipboardListener(
      mojo::PendingRemote<mojom::blink::ClipboardListener> listener) override;
#if BUILDFLAG(IS_MAC)
  void WriteStringToFindPboard(const String& text) override;
#endif
  Vector<String> ReadStandardFormatNames();

  mojo::ReceiverSet<mojom::blink::ClipboardHost> receivers_;
  mojo::Remote<mojom::blink::ClipboardListener> clipboard_listener_;
  ClipboardSequenceNumberToken sequence_number_;
  String plain_text_ = g_empty_string;
  String html_text_ = g_empty_string;
  String svg_text_ = g_empty_string;
  String rtf_text_ = g_empty_string;
  mojom::blink::ClipboardFilesPtr files_ = mojom::blink::ClipboardFiles::New();
  KURL url_;
  Vector<uint8_t> png_;
  // TODO(asully): Remove `image_` once ReadImage() path is removed.
  SkBitmap image_;
  HashMap<String, String> custom_data_;
  bool write_smart_paste_ = false;
  bool needs_reset_ = false;
  HashMap<String, Vector<uint8_t>> unsanitized_custom_data_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MOCK_CLIPBOARD_HOST_H_
