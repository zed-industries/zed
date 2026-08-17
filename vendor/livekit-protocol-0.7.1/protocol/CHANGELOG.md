# github.com/livekit/protocol

## 1.44.0

### Minor Changes

- Add numbers field to SIPDispatchRuleInfo for filtering calls by called number - [#1351](https://github.com/livekit/protocol/pull/1351) ([@civilcoder55](https://github.com/civilcoder55))

### Patch Changes

- Add auto_subscribe_data_track to StartSession. - [#1366](https://github.com/livekit/protocol/pull/1366) ([@boks1971](https://github.com/boks1971))

- Add TextMessageRequest and TextMessageResponse - [#1349](https://github.com/livekit/protocol/pull/1349) ([@longcw](https://github.com/longcw))

- Add featureinfo nested field for reporting noise cancellation feature specific metadata - [#1367](https://github.com/livekit/protocol/pull/1367) ([@1egoman](https://github.com/1egoman))

- Option to control auto subscribe of data tracks. - [#1365](https://github.com/livekit/protocol/pull/1365) ([@boks1971](https://github.com/boks1971))

- Changing the wording on duplicate dispatch rule error message - [#1343](https://github.com/livekit/protocol/pull/1343) ([@alexlivekit](https://github.com/alexlivekit))

- Allow passing ClientParams to SIP RPC client. - [#1356](https://github.com/livekit/protocol/pull/1356) ([@dennwc](https://github.com/dennwc))

- Add project ID to internal SIPCall info. - [#1346](https://github.com/livekit/protocol/pull/1346) ([@dennwc](https://github.com/dennwc))

- Add helpers for tracing attributes. - [#1363](https://github.com/livekit/protocol/pull/1363) ([@dennwc](https://github.com/dennwc))

- Add helper for setting Jaeger tracing. - [#1355](https://github.com/livekit/protocol/pull/1355) ([@dennwc](https://github.com/dennwc))

- Store repair ssrc in TrackInfo for migration purposes. - [#1348](https://github.com/livekit/protocol/pull/1348) ([@boks1971](https://github.com/boks1971))

## 1.43.4

### Patch Changes

- add tags to agent recording proto - [#1328](https://github.com/livekit/protocol/pull/1328) ([@paulwe](https://github.com/paulwe))

## 1.43.3

### Patch Changes

- Data tracks initial protocol definition. - [#1327](https://github.com/livekit/protocol/pull/1327) ([@boks1971](https://github.com/boks1971))

- Cancellation counts for join/publish/subscribe. - [#1321](https://github.com/livekit/protocol/pull/1321) ([@boks1971](https://github.com/boks1971))

- Add ai coustics feature usage - [#1324](https://github.com/livekit/protocol/pull/1324) ([@lukasIO](https://github.com/lukasIO))

## 1.43.2

## 1.43.1

### Patch Changes

- add option to only use IPv4 for webhooks - [#1296](https://github.com/livekit/protocol/pull/1296) ([@davidzhao](https://github.com/davidzhao))

- Redact metadata, attribute, participant name from logs. - [#1309](https://github.com/livekit/protocol/pull/1309) ([@boks1971](https://github.com/boks1971))

- Adding more SIP return codes and updating SIP-to-grpc return code mapping - [#1305](https://github.com/livekit/protocol/pull/1305) ([@alexlivekit](https://github.com/alexlivekit))

- Switch to new header validation for SIP commands - [#1279](https://github.com/livekit/protocol/pull/1279) ([@alexlivekit](https://github.com/alexlivekit))

## 1.43.0

### Patch Changes

- Added warning prints to SIP headers - [#1238](https://github.com/livekit/protocol/pull/1238) ([@alexlivekit](https://github.com/alexlivekit))

- removed observability field - [#1294](https://github.com/livekit/protocol/pull/1294) ([@davidzhao](https://github.com/davidzhao))

## 1.42.2

### Patch Changes

- Include mid in SessionDescription - [#1257](https://github.com/livekit/protocol/pull/1257) ([@lukasIO](https://github.com/lukasIO))

## 1.42.1

## 1.42.0

### Minor Changes

- rename ListUpdate "del" field to "rename" - [#1203](https://github.com/livekit/protocol/pull/1203) ([@rektdeckard](https://github.com/rektdeckard))

- Utilities for phone number functionality - [#1206](https://github.com/livekit/protocol/pull/1206) ([@nishadmusthafa](https://github.com/nishadmusthafa))

- Adding a call_context field to SIPCallInfo - [#1188](https://github.com/livekit/protocol/pull/1188) ([@nishadmusthafa](https://github.com/nishadmusthafa))

- Added DisplayName field to CreateSIPParticipantRequest - [#1216](https://github.com/livekit/protocol/pull/1216) ([@alexlivekit](https://github.com/alexlivekit))

### Patch Changes

- Adding get and update apis for livekit phone numbers - [#1179](https://github.com/livekit/protocol/pull/1179) ([@nishadmusthafa](https://github.com/nishadmusthafa))

- allow credentials to be stored in SIP tokens - [#1213](https://github.com/livekit/protocol/pull/1213) ([@davidzhao](https://github.com/davidzhao))

- Unifying data types for phone numbers - [#1190](https://github.com/livekit/protocol/pull/1190) ([@nishadmusthafa](https://github.com/nishadmusthafa))

- Adding an rpc to record call context using SipCallInfo - [#1189](https://github.com/livekit/protocol/pull/1189) ([@nishadmusthafa](https://github.com/nishadmusthafa))

## 1.41.0

### Minor Changes

- Adding public apis for Livekit Phone Numbers feature - [#1146](https://github.com/livekit/protocol/pull/1146) ([@nishadmusthafa](https://github.com/nishadmusthafa))

- Add encryption metadata for data packets - [#1127](https://github.com/livekit/protocol/pull/1127) ([@lukasIO](https://github.com/lukasIO))

- Use OpenTelemetry types. Pass tracer options. - [#1177](https://github.com/livekit/protocol/pull/1177) ([@dennwc](https://github.com/dennwc))

- Add a Name helper for SIPTransport. - [#1175](https://github.com/livekit/protocol/pull/1175) ([@dennwc](https://github.com/dennwc))

## 1.40.0

### Minor Changes

- Adding SIP Protocol generated call id to SipCall - [#1119](https://github.com/livekit/protocol/pull/1119) ([@nishadmusthafa](https://github.com/nishadmusthafa))

### Patch Changes

- Store SDP cid in TrackInfo. - [#1164](https://github.com/livekit/protocol/pull/1164) ([@boks1971](https://github.com/boks1971))

- Support per simulcast codec layers. - [#1157](https://github.com/livekit/protocol/pull/1157) ([@boks1971](https://github.com/boks1971))

- Add webhook for aborted participant connection. - [#1166](https://github.com/livekit/protocol/pull/1166) ([@boks1971](https://github.com/boks1971))

- Go SDK support for chat messages. - [#1147](https://github.com/livekit/protocol/pull/1147) ([@dennwc](https://github.com/dennwc))

- Go SDK support for stream data messages. - [#1148](https://github.com/livekit/protocol/pull/1148) ([@dennwc](https://github.com/dennwc))

- Updating how ListSIPInboundTrunkRequest filtering occurs by normalising phone number that are being compared - [#1126](https://github.com/livekit/protocol/pull/1126) ([@nishadmusthafa](https://github.com/nishadmusthafa))

- add hash function to generate common guid - [#1163](https://github.com/livekit/protocol/pull/1163) ([@paulwe](https://github.com/paulwe))

- Allow adding, removing items and clearing list fields. - [#1142](https://github.com/livekit/protocol/pull/1142) ([@dennwc](https://github.com/dennwc))

- Allow clients to specify video layers mode. - [#1158](https://github.com/livekit/protocol/pull/1158) ([@boks1971](https://github.com/boks1971))

## 1.39.3

## 1.39.2

## 1.39.1

### Patch Changes

- adding detailed responses to the trunk match logic which can help with decisions on blocking - [#1101](https://github.com/livekit/protocol/pull/1101) ([@nishadmusthafa](https://github.com/nishadmusthafa))

- E2E reliability for data channel - [#1099](https://github.com/livekit/protocol/pull/1099) ([@cnderrauber](https://github.com/cnderrauber))

- ensure access token do not contain sensitive credentials - [#1097](https://github.com/livekit/protocol/pull/1097) ([@davidzhao](https://github.com/davidzhao))

- WHIP internal signalling to be able to support WHIP in OSS. - [#1091](https://github.com/livekit/protocol/pull/1091) ([@boks1971](https://github.com/boks1971))

## 1.39.0

### Patch Changes

- Add a MEDIA_FAILURE disconnect reason. - [#1085](https://github.com/livekit/protocol/pull/1085) ([@dennwc](https://github.com/dennwc))

## 1.38.0

### Minor Changes

- feat: MoveParticipant API - [#1065](https://github.com/livekit/protocol/pull/1065) ([@cnderrauber](https://github.com/cnderrauber))

## 1.37.1

### Patch Changes

- Deprecate explicit AddTrackRequest fields that can be read from AudioTrackFeatures - [#1058](https://github.com/livekit/protocol/pull/1058) ([@lukasIO](https://github.com/lukasIO))

- Add PreconnectBuffer to AudioTrackFeatures, add AudioTrackFeatures to AddTrackRequest - [#1057](https://github.com/livekit/protocol/pull/1057) ([@lukasIO](https://github.com/lukasIO))

## 1.37.0

### Minor Changes

- Allow updating SIP media encryption. - [#1047](https://github.com/livekit/protocol/pull/1047) ([@dennwc](https://github.com/dennwc))

- Add helper to get SIP call status from an error. - [#1028](https://github.com/livekit/protocol/pull/1028) ([@dennwc](https://github.com/dennwc))

- Add timeout parameter to SIP transfer API. - [#1036](https://github.com/livekit/protocol/pull/1036) ([@dennwc](https://github.com/dennwc))

- Fix SIP update when replacing array fields. - [#1038](https://github.com/livekit/protocol/pull/1038) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- Add CONNECTION_TIMEOUT disconnect reason - [#1037](https://github.com/livekit/protocol/pull/1037) ([@boks1971](https://github.com/boks1971))

- Add rate for memory use and total. - [#1030](https://github.com/livekit/protocol/pull/1030) ([@boks1971](https://github.com/boks1971))

- Revert cid changes. - [#1029](https://github.com/livekit/protocol/pull/1029) ([@boks1971](https://github.com/boks1971))

- Use cgroup for memory stats. - [#1031](https://github.com/livekit/protocol/pull/1031) ([@boks1971](https://github.com/boks1971))

- Allow '\_' in SIP headers. - [#1048](https://github.com/livekit/protocol/pull/1048) ([@dennwc](https://github.com/dennwc))

- Add more fields to CodecInfo. - [#1024](https://github.com/livekit/protocol/pull/1024) ([@boks1971](https://github.com/boks1971))

- Add client info enum value for unreal - [#1032](https://github.com/livekit/protocol/pull/1032) ([@bcherry](https://github.com/bcherry))

- Rework node stats to split out rate metrics into its own shape. - [#1023](https://github.com/livekit/protocol/pull/1023) ([@boks1971](https://github.com/boks1971))

## 1.36.1

### Patch Changes

- Remove unused kinds - [#1020](https://github.com/livekit/protocol/pull/1020) ([@cnderrauber](https://github.com/cnderrauber))

- Refine backup codec policy - [#1022](https://github.com/livekit/protocol/pull/1022) ([@cnderrauber](https://github.com/cnderrauber))

## 1.36.0

### Minor Changes

- add cloud agents - [#1010](https://github.com/livekit/protocol/pull/1010) ([@real-danm](https://github.com/real-danm))

- sdp.go: add helper functions for simulcast and trackId - [#1018](https://github.com/livekit/protocol/pull/1018) ([@anunaym14](https://github.com/anunaym14))

- Update API for SIP. - [#869](https://github.com/livekit/protocol/pull/869) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- Fix incremental SIP dispatch rule update. - [#1014](https://github.com/livekit/protocol/pull/1014) ([@dennwc](https://github.com/dennwc))

## 1.35.0

### Minor Changes

- Add ForwardParticipant to room service - [#1011](https://github.com/livekit/protocol/pull/1011) ([@cnderrauber](https://github.com/cnderrauber))

- Move SIP call dispatch info into s separate type. - [#989](https://github.com/livekit/protocol/pull/989) ([@dennwc](https://github.com/dennwc))

- Handle SIP hostname in address filters. Expose attribute. - [#987](https://github.com/livekit/protocol/pull/987) ([@dennwc](https://github.com/dennwc))

- Expose and improve SIP number normalization. - [#986](https://github.com/livekit/protocol/pull/986) ([@dennwc](https://github.com/dennwc))

- More analytics fields for SIPCallInfo. - [#984](https://github.com/livekit/protocol/pull/984) ([@dennwc](https://github.com/dennwc))

## 1.34.0

### Minor Changes

- Allow passing all SIP trunk options in CreateSIPParticipant. - [#961](https://github.com/livekit/protocol/pull/961) ([@dennwc](https://github.com/dennwc))

- Add egress audio mixing options - [#978](https://github.com/livekit/protocol/pull/978) ([@frostbyte73](https://github.com/frostbyte73))

- Expose hooks for SIP trunk and rule matching. - [#983](https://github.com/livekit/protocol/pull/983) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- Fix iteration over empty and unsorted records. - [#971](https://github.com/livekit/protocol/pull/971) ([@dennwc](https://github.com/dennwc))

- Remove SIP outbound number normalization for Telnyx. - [#969](https://github.com/livekit/protocol/pull/969) ([@dennwc](https://github.com/dennwc))

- Remove SIP outbound number normalization for Telnyx. - [#982](https://github.com/livekit/protocol/pull/982) ([@dennwc](https://github.com/dennwc))

## 1.33.0

### Minor Changes

- Use SIP statuses as Go and gRPC errors. - [#960](https://github.com/livekit/protocol/pull/960) ([@dennwc](https://github.com/dennwc))

- Add flag to make CreateSIPParticipant synchronous. - [#952](https://github.com/livekit/protocol/pull/952) ([@dennwc](https://github.com/dennwc))

- Add Twirp options to preserve error details and client timeouts. - [#963](https://github.com/livekit/protocol/pull/963) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- Add a workaround for invalid IP masks in SIP. - [#956](https://github.com/livekit/protocol/pull/956) ([@dennwc](https://github.com/dennwc))

- Add backup codec policy to AddTrackRequest - [#947](https://github.com/livekit/protocol/pull/947) ([@cnderrauber](https://github.com/cnderrauber))

## 1.32.1

### Patch Changes

- Add nonce to SendDataRequest #953 - [#954](https://github.com/livekit/protocol/pull/954) ([@lukasIO](https://github.com/lukasIO))

## 1.32.0

### Minor Changes

- Use iterators in SIP. - [#943](https://github.com/livekit/protocol/pull/943) ([@dennwc](https://github.com/dennwc))

## 1.31.0

### Minor Changes

- Add media encryption options for SIP. - [#892](https://github.com/livekit/protocol/pull/892) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- include room preset/config in SIP tokens - [#932](https://github.com/livekit/protocol/pull/932) ([@davidzhao](https://github.com/davidzhao))

## 1.30.0

### Minor Changes

- Add filters to SIP list APIs. - [#913](https://github.com/livekit/protocol/pull/913) ([@dennwc](https://github.com/dennwc))

- Add headers for SIP call transfer. - [#926](https://github.com/livekit/protocol/pull/926) ([@dennwc](https://github.com/dennwc))

- Add headers for CreateSipParticipant - [#911](https://github.com/livekit/protocol/pull/911) ([@s-hamdananwar](https://github.com/s-hamdananwar))

- Allow mapping all SIP headers to attributes. - [#920](https://github.com/livekit/protocol/pull/920) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- Allow SIP inbound to specify room configuration - [#923](https://github.com/livekit/protocol/pull/923) ([@davidzhao](https://github.com/davidzhao))

- Relax SIP header validation. - [#925](https://github.com/livekit/protocol/pull/925) ([@dennwc](https://github.com/dennwc))

## 1.29.4

## 1.29.3

## 1.29.2

### Patch Changes

- Revert back to Go 1.22 - [#905](https://github.com/livekit/protocol/pull/905) ([@davidzhao](https://github.com/davidzhao))

## 1.29.1

## 1.29.0

### Minor Changes

- Add DataStream support - [#871](https://github.com/livekit/protocol/pull/871) ([@lukasIO](https://github.com/lukasIO))

## 1.28.1

## 1.28.0

### Minor Changes

- Add disconnect reason for SIP failures. - [#845](https://github.com/livekit/protocol/pull/845) ([@dennwc](https://github.com/dennwc))

- Map participant attributes to SIP headers. - [#893](https://github.com/livekit/protocol/pull/893) ([@dennwc](https://github.com/dennwc))

- Allow setting number for SIP outbound. - [#891](https://github.com/livekit/protocol/pull/891) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- Annotate SIP errors with Twirp codes. - [#879](https://github.com/livekit/protocol/pull/879) ([@dennwc](https://github.com/dennwc))

- Fix port type for SIPUri. - [#882](https://github.com/livekit/protocol/pull/882) ([@dennwc](https://github.com/dennwc))

- Require number, CIDR or auth for SIP inbound. - [#890](https://github.com/livekit/protocol/pull/890) ([@dennwc](https://github.com/dennwc))

## 1.27.1

### Patch Changes

- Add extra fields to SIP analytics events - [#872](https://github.com/livekit/protocol/pull/872) ([@biglittlebigben](https://github.com/biglittlebigben))

## 1.27.0

### Minor Changes

- Support for room configuration and agent dispatches - [#864](https://github.com/livekit/protocol/pull/864) ([@davidzhao](https://github.com/davidzhao))

### Patch Changes

- added manifest details to egress info - [#862](https://github.com/livekit/protocol/pull/862) ([@frostbyte73](https://github.com/frostbyte73))

- Type safe IP checks for SIP Trunks. - [#857](https://github.com/livekit/protocol/pull/857) ([@dennwc](https://github.com/dennwc))

- enable krisp SIP setting - [#866](https://github.com/livekit/protocol/pull/866) ([@frostbyte73](https://github.com/frostbyte73))

## 1.26.0

### Minor Changes

- allow Agents to pass through initial attributes - [#852](https://github.com/livekit/protocol/pull/852) ([@davidzhao](https://github.com/davidzhao))

## 1.25.0

### Minor Changes

- Add ringing timeout and max call duration setting for SIP. - [#844](https://github.com/livekit/protocol/pull/844) ([@dennwc](https://github.com/dennwc))

## 1.24.0

### Minor Changes

- Add disconnect reasons for SIP. - [#828](https://github.com/livekit/protocol/pull/828) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- Add missing SIP status attribute. - [#833](https://github.com/livekit/protocol/pull/833) ([@dennwc](https://github.com/dennwc))

- Validate SipCallTo in CreateSIPParticipantRequest. - [#823](https://github.com/livekit/protocol/pull/823) ([@dennwc](https://github.com/dennwc))

## 1.23.0

### Patch Changes

- Add protocols for client metrics - [#821](https://github.com/livekit/protocol/pull/821) ([@davidliu](https://github.com/davidliu))

- Add other_sdks field to ClientInfo - [#804](https://github.com/livekit/protocol/pull/804) ([@bcherry](https://github.com/bcherry))

## 1.22.0

### Minor Changes

- Update SIP protocol. Pass headers, project ID and hostname. - [#805](https://github.com/livekit/protocol/pull/805) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- Add generated flag to ChatMessage - [#813](https://github.com/livekit/protocol/pull/813) ([@lukasIO](https://github.com/lukasIO))

- Add dedicated chat message definition - [#785](https://github.com/livekit/protocol/pull/785) ([@lukasIO](https://github.com/lukasIO))

- Update @bufbuild/protobuf dependency - [#800](https://github.com/livekit/protocol/pull/800) ([@lukasIO](https://github.com/lukasIO))

- Metrics server side timestamp - [#806](https://github.com/livekit/protocol/pull/806) ([@boks1971](https://github.com/boks1971))

## 1.21.0

### Minor Changes

- Add Callee dispatch rule type for SIP. - [#798](https://github.com/livekit/protocol/pull/798) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- Add SDK values for Unity-Web and NodeJS. - [#797](https://github.com/livekit/protocol/pull/797) ([@bcherry](https://github.com/bcherry))

## 1.20.1

### Patch Changes

- Add fast_publish option to JoinResponse - [#796](https://github.com/livekit/protocol/pull/796) ([@cnderrauber](https://github.com/cnderrauber))

## 1.20.0

### Minor Changes

- Send non-error responses also as part of RequestResponse - [#787](https://github.com/livekit/protocol/pull/787) ([@lukasIO](https://github.com/lukasIO))

### Patch Changes

- Add disconnect reason in participant info. - [#788](https://github.com/livekit/protocol/pull/788) ([@boks1971](https://github.com/boks1971))

- Use multiple webhook workers for each URL to improve parallelism - [#784](https://github.com/livekit/protocol/pull/784) ([@davidzhao](https://github.com/davidzhao))

## 1.19.3

## 1.19.2

### Patch Changes

- Add FilenamePrefix to ImagesInfo - [#751](https://github.com/livekit/protocol/pull/751) ([@frostbyte73](https://github.com/frostbyte73))

- Add experimental replay api - [#755](https://github.com/livekit/protocol/pull/755) ([@frostbyte73](https://github.com/frostbyte73))

- Implement a custom YAML unmarshaller for RoomConfiguration - [#765](https://github.com/livekit/protocol/pull/765) ([@biglittlebigben](https://github.com/biglittlebigben))

- Add AgentDispatchPrefix to guid - [#757](https://github.com/livekit/protocol/pull/757) ([@biglittlebigben](https://github.com/biglittlebigben))

- Ability to set attributes upon job acceptance - [#769](https://github.com/livekit/protocol/pull/769) ([@davidzhao](https://github.com/davidzhao))

- Remove unused fields from RegisterWorkerRequest - [#761](https://github.com/livekit/protocol/pull/761) ([@biglittlebigben](https://github.com/biglittlebigben))

- Integrate feedback on the agents protocol: - [#750](https://github.com/livekit/protocol/pull/750) ([@biglittlebigben](https://github.com/biglittlebigben))

  - Rename JobDescription to AgentDispatch
  - Remove participant_identity entry in the dispatch
  - Deprecate namespace
  - Add an agent_name field to specify what agent workers a job should be dispatched to

  Also allow setting a room configuration in the token.

- add srt output for egress - [#766](https://github.com/livekit/protocol/pull/766) ([@frostbyte73](https://github.com/frostbyte73))

- Internal agent protocol improvements - [#764](https://github.com/livekit/protocol/pull/764) ([@biglittlebigben](https://github.com/biglittlebigben))

- Implement MarshalYAML on RoomConfiguration and related types - [#771](https://github.com/livekit/protocol/pull/771) ([@biglittlebigben](https://github.com/biglittlebigben))

- Use consistent json field name for room configuration grant - [#762](https://github.com/livekit/protocol/pull/762) ([@biglittlebigben](https://github.com/biglittlebigben))

- Added RoomClosed as a DisconnectReason - [#778](https://github.com/livekit/protocol/pull/778) ([@davidzhao](https://github.com/davidzhao))

## 1.19.1

## 1.19.0

### Minor Changes

- Add SIP grants. - [#745](https://github.com/livekit/protocol/pull/745) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- Fix typo in SIP Trunk conversion. - [#744](https://github.com/livekit/protocol/pull/744) ([@dennwc](https://github.com/dennwc))

## 1.18.0

### Minor Changes

- Split SIP Trunk configuration to inbound and outbound parts. - [#738](https://github.com/livekit/protocol/pull/738) ([@dennwc](https://github.com/dennwc))

- add Participant attributes key/val storage - [#733](https://github.com/livekit/protocol/pull/733) ([@davidzhao](https://github.com/davidzhao))

### Patch Changes

- Simplify number matching rules for SIP. - [#737](https://github.com/livekit/protocol/pull/737) ([@dennwc](https://github.com/dennwc))

- Added session token option for s3 uploads - [#734](https://github.com/livekit/protocol/pull/734) ([@frostbyte73](https://github.com/frostbyte73))

- Include analytics event ids - [#739](https://github.com/livekit/protocol/pull/739) ([@davidzhao](https://github.com/davidzhao))

## 1.17.0

### Minor Changes

- Add support for additional SIP transports. - [#719](https://github.com/livekit/protocol/pull/719) ([@dennwc](https://github.com/dennwc))

- Add ICERestartWHIPResource to RPC - [#716](https://github.com/livekit/protocol/pull/716) ([@Sean-Der](https://github.com/Sean-Der))

### Patch Changes

- Added error code field to EgressInfo - [#714](https://github.com/livekit/protocol/pull/714) ([@frostbyte73](https://github.com/frostbyte73))

- Include node_id with analytics events - [#727](https://github.com/livekit/protocol/pull/727) ([@davidzhao](https://github.com/davidzhao))

- Match SIP Trunks by source IP or mask. - [#724](https://github.com/livekit/protocol/pull/724) ([@dennwc](https://github.com/dennwc))

- Add notice to use participant identity from DataPacket for transcriptions. - [#706](https://github.com/livekit/protocol/pull/706) ([@dennwc](https://github.com/dennwc))

## 1.16.0

### Minor Changes

- - Add RedactAutoEncodedOutput to support redacting auto egress types - [#712](https://github.com/livekit/protocol/pull/712) ([@biglittlebigben](https://github.com/biglittlebigben))
  - Redact image outputs
  - Support AliOSS uploads for auto egress

- Allow inbound number filtering on SIP Dispatch Rules - [#707](https://github.com/livekit/protocol/pull/707) ([@dennwc](https://github.com/dennwc))

- Move egress request redacting routines to protocol - [#711](https://github.com/livekit/protocol/pull/711) ([@biglittlebigben](https://github.com/biglittlebigben))

## 1.15.0

### Minor Changes

- Expose Logger constructor for Zap. - [#701](https://github.com/livekit/protocol/pull/701) ([@dennwc](https://github.com/dennwc))

- Add metadata to SIP trunks, dispatch rules and participants. Change SIP participant identity prefix to `sip_`. - [#696](https://github.com/livekit/protocol/pull/696) ([@dennwc](https://github.com/dennwc))

- Move language into TranscriptionSegment #703 - [#704](https://github.com/livekit/protocol/pull/704) ([@lukasIO](https://github.com/lukasIO))

## 1.14.0

### Minor Changes

- Added real-time Transcription protocol - [#686](https://github.com/livekit/protocol/pull/686) ([@davidzhao](https://github.com/davidzhao))

- WHIP protocol change - [#680](https://github.com/livekit/protocol/pull/680) ([@biglittlebigben](https://github.com/biglittlebigben))

  Deprecate the bypass_transcoding property in all ingress APIs and introduce the optional enable_transcoding property. This property will default to false for WHIP and to true for all other ingress types.

### Patch Changes

- Add SIP participant name. - [#687](https://github.com/livekit/protocol/pull/687) ([@dennwc](https://github.com/dennwc))

## 1.13.0

### Minor Changes

- Add and option to play ringtone for SIP outbound calls. - [#671](https://github.com/livekit/protocol/pull/671) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- Add initial support for slog. - [#668](https://github.com/livekit/protocol/pull/668) ([@dennwc](https://github.com/dennwc))

## 1.12.0

### Minor Changes

- Moved CPU stats to a separate hwstats package, removing cgo dependency. - [#660](https://github.com/livekit/protocol/pull/660) ([@dennwc](https://github.com/dennwc))

### Patch Changes

- Fixed bounds check when masking short SIP numbers. - [#650](https://github.com/livekit/protocol/pull/650) ([@dennwc](https://github.com/dennwc))

- Add signal requests for local track updates - [#651](https://github.com/livekit/protocol/pull/651) ([@lukasIO](https://github.com/lukasIO))

- Allow sending DTMF when creating SIP participant. - [#658](https://github.com/livekit/protocol/pull/658) ([@dennwc](https://github.com/dennwc))

## 1.11.0

### Minor Changes

- Align package version with manually tagged golang module - [#646](https://github.com/livekit/protocol/pull/646) ([@lukasIO](https://github.com/lukasIO))

## 1.10.4

## 1.10.3

## 1.10.2
