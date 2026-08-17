// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_MEDIA_INSPECTOR_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_MEDIA_INSPECTOR_H_

#include <vector>

#include "base/time/time.h"
#include "third_party/blink/public/platform/web_string.h"

namespace blink {

// These types should look exactly like the types defined in
// browser_protocol.pdl.
struct InspectorPlayerMessage {
  enum class Level { kError, kWarning, kInfo, kDebug };
  Level level;
  WebString message;
};
using InspectorPlayerMessages = std::vector<InspectorPlayerMessage>;

struct InspectorPlayerProperty {
  WebString name;
  WebString value;
};
using InspectorPlayerProperties = std::vector<InspectorPlayerProperty>;

struct InspectorPlayerEvent {
  base::TimeTicks timestamp;
  WebString value;
};
using InspectorPlayerEvents = std::vector<InspectorPlayerEvent>;

struct InspectorPlayerError {
  struct Data {
    WebString name;
    WebString value;
  };
  struct SourceLocation {
    WebString filename;
    int line_number;
  };
  WebString group;
  int code;
  WebString message;
  std::vector<SourceLocation> stack;
  std::vector<InspectorPlayerError> caused_by;
  std::vector<Data> data;
};
using InspectorPlayerErrors = std::vector<InspectorPlayerError>;

class MediaInspectorContext {
 public:
  virtual WebString CreatePlayer() = 0;

  virtual void DestroyPlayer(const WebString& playerId) = 0;

  virtual void NotifyPlayerEvents(WebString player_id,
                                  const InspectorPlayerEvents&) = 0;
  virtual void NotifyPlayerErrors(WebString player_id,
                                  const InspectorPlayerErrors&) = 0;
  virtual void NotifyPlayerMessages(WebString player_id,
                                    const InspectorPlayerMessages&) = 0;
  virtual void SetPlayerProperties(WebString player_id,
                                   const InspectorPlayerProperties&) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_MEDIA_INSPECTOR_H_
