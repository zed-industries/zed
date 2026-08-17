//
//
// Copyright 2018 gRPC authors.
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
//
//

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_ALTS_CHECK_GCP_ENVIRONMENT_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_ALTS_CHECK_GCP_ENVIRONMENT_H

namespace grpc_core {
namespace internal {

///
/// This method is a helper function that reads a file containing system bios
/// data. Exposed for testing only.
///
///- bios_file: a file containing BIOS data used to determine GCE tenancy
///  information.
///
/// It returns a buffer containing the data read from the file.
///
char* read_bios_file(const char* bios_file);

///
/// This method checks if system BIOS data contains Google-specific phrases.
/// Exposed for testing only.
///
///- bios_data: a buffer containing system BIOS data.
///
/// It returns true if the BIOS data contains Google-specific phrases, and false
/// otherwise.
///
bool check_bios_data(const char* bios_data);

}  // namespace internal
}  // namespace grpc_core

///
/// This method checks if a VM (Windows or Linux) is running within Google
/// compute Engine (GCE) or not. It returns true if the VM is running in GCE and
/// false otherwise.
///
bool grpc_alts_is_running_on_gcp();

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_ALTS_CHECK_GCP_ENVIRONMENT_H
