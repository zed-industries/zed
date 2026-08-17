// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_RESPONSE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_RESPONSE_H_

#include "services/network/public/mojom/fetch_api.mojom-blink.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_response.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/dictionary.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/fetch/body.h"
#include "third_party/blink/renderer/core/fetch/body_stream_buffer.h"
#include "third_party/blink/renderer/core/fetch/fetch_response_data.h"
#include "third_party/blink/renderer/core/fetch/headers.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExceptionState;
class ResponseInit;
class ScriptState;
class V8ResponseType;

class CORE_EXPORT Response final : public ScriptWrappable, public Body {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // These "create" function which takes a ScriptState* must be called with
  // entering an appropriate V8 context.
  // From Response.idl:
  static Response* Create(ScriptState*, ExceptionState&);
  static Response* Create(ScriptState*,
                          ScriptValue body,
                          const ResponseInit*,
                          ExceptionState&);

  static Response* Create(ScriptState*,
                          BodyStreamBuffer*,
                          const String& content_type,
                          const ResponseInit*,
                          ExceptionState&);
  static Response* Create(ExecutionContext*, FetchResponseData*);
  static Response* Create(ScriptState*, mojom::blink::FetchAPIResponse&);

  static Response* CreateClone(const Response&);

  static Response* error(ScriptState*);
  static Response* redirect(ScriptState*,
                            const String& url,
                            uint16_t status,
                            ExceptionState&);
  static Response* staticJson(ScriptState*,
                              ScriptValue data,
                              const ResponseInit*,
                              ExceptionState&);

  static FetchResponseData* CreateUnfilteredFetchResponseDataWithoutBody(
      ScriptState*,
      mojom::blink::FetchAPIResponse&);

  static FetchResponseData* FilterResponseData(
      network::mojom::FetchResponseType response_type,
      FetchResponseData* response,
      WTF::Vector<WTF::String>& headers);

  explicit Response(ExecutionContext*);
  Response(ExecutionContext*, FetchResponseData*);
  Response(ExecutionContext*, FetchResponseData*, Headers*);
  Response(const Response&) = delete;
  Response& operator=(const Response&) = delete;

  const FetchResponseData* GetResponse() const { return response_.Get(); }

  // From Response.idl:
  V8ResponseType type() const;
  String url() const;
  bool redirected() const;
  uint16_t status() const;
  bool ok() const;
  String statusText() const;
  Headers* headers() const;

  // From Response.idl:
  // This function must be called with entering an appropriate V8 context.
  Response* clone(ScriptState*, ExceptionState&);

  // Does not contain the blob response body or any side data blob.
  // |request_url| is the current request URL that resulted in the response. It
  // is needed to process some response headers (e.g. CSP).
  // TODO(lfg, kinuko): The FetchResponseData::url_list_ should include the
  // request URL per step 9 in Main Fetch
  // https://fetch.spec.whatwg.org/#main-fetch. Just fixing it might break the
  // logic in ResourceMultiBufferDataProvider, please see
  // https://chromium-review.googlesource.com/c/1366464 for more details.
  mojom::blink::FetchAPIResponsePtr PopulateFetchAPIResponse(
      const KURL& request_url);

  bool HasBody() const;
  BodyStreamBuffer* BodyBuffer() override { return response_->Buffer(); }
  // Returns the BodyStreamBuffer of |m_response|. This method doesn't check
  // the internal response of |m_response| even if |m_response| has it.
  const BodyStreamBuffer* BodyBuffer() const override {
    return response_->Buffer();
  }
  // Returns the BodyStreamBuffer of the internal response of |m_response| if
  // any. Otherwise, returns one of |m_response|.
  BodyStreamBuffer* InternalBodyBuffer() { return response_->InternalBuffer(); }
  const BodyStreamBuffer* InternalBodyBuffer() const {
    return response_->InternalBuffer();
  }

  bool IsBodyUsed() const override;

  String ContentType() const override;
  String MimeType() const override;
  String InternalMIMEType() const;

  const Vector<KURL>& InternalURLList() const;

  FetchHeaderList* InternalHeaderList() const;

  void Trace(Visitor*) const override;

 private:
  const Member<FetchResponseData> response_;
  const Member<Headers> headers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_RESPONSE_H_
