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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_ERROR_CFSTREAM_H
#define GRPC_SRC_CORE_LIB_IOMGR_ERROR_CFSTREAM_H

#ifdef GRPC_CFSTREAM
// Create an error from Apple Core Foundation CFError object
#define GRPC_ERROR_CREATE_FROM_CFERROR(error, desc)  \
  grpc_error_create_from_cferror(__FILE__, __LINE__, \
                                 static_cast<void*>((error)), (desc))
grpc_error_handle grpc_error_create_from_cferror(const char* file, int line,
                                                 void* arg, const char* desc);
#endif  // GRPC_CFSTREAM

#endif  // GRPC_SRC_CORE_LIB_IOMGR_ERROR_CFSTREAM_H
