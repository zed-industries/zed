#!/bin/bash
# Copyright 2025 LiveKit, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# dependencies: cargo install protoc-gen-prost@0.3.1 protoc-gen-prost-serde@0.3.1


PROTOCOL=protocol/protobufs
OUT_RUST=src

protoc \
    -I=$PROTOCOL \
    --prost_out=$OUT_RUST \
    --prost_opt=compile_well_known_types \
    --prost_opt=extern_path=.google.protobuf=::pbjson_types \
    --prost-serde_out=$OUT_RUST \
    --prost-serde_opt=ignore_unknown_fields \
    $PROTOCOL/livekit_egress.proto \
    $PROTOCOL/livekit_rtc.proto \
    $PROTOCOL/livekit_room.proto \
    $PROTOCOL/livekit_webhook.proto \
    $PROTOCOL/livekit_sip.proto \
    $PROTOCOL/livekit_models.proto \
    $PROTOCOL/livekit_connector.proto \
    $PROTOCOL/livekit_connector_whatsapp.proto \
    $PROTOCOL/livekit_connector_twilio.proto
