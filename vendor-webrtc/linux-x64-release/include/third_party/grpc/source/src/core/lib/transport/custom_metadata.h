// Copyright 2023 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_LIB_TRANSPORT_CUSTOM_METADATA_H
#define GRPC_SRC_CORE_LIB_TRANSPORT_CUSTOM_METADATA_H

// This file defines two macros: GRPC_CUSTOM_CLIENT_METADATA and
// GRPC_CUSTOM_SERVER_METADATA.
// Each of these is a comma-prefixed and comma-separated list of metadata types
// that will be added to ClientMetadata and ServerMetadata respectively.
// (note: for now both are added to both).
// Different sites with internal grpc-core extensions can substitute this file
// and define their own versions of these macros to extend the metadata system
// with fast metadata types of their own.

#define GRPC_CUSTOM_CLIENT_METADATA
#define GRPC_CUSTOM_SERVER_METADATA

#endif  // GRPC_SRC_CORE_LIB_TRANSPORT_CUSTOM_METADATA_H
