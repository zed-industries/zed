// Copyright 2016 gRPC authors.
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

#include <string>
#include <utility>
#include <vector>

namespace grpc_tools {
// We declare `protoc_main` here since we want access to it from Cython as an
// extern but *without* triggering a dllimport declspec when on Windows.
int protoc_main(int argc, char* argv[], char* version);

struct ProtocError {
  std::string filename;
  int line;
  int column;
  std::string message;

  ProtocError() {}
  ProtocError(std::string filename, int line, int column, std::string message)
      : filename(filename), line(line), column(column), message(message) {}
};

typedef ProtocError ProtocWarning;

int protoc_get_protos(
    char* protobuf_path, const std::vector<std::string>* include_paths,
    std::vector<std::pair<std::string, std::string>>* files_out,
    std::vector<ProtocError>* errors, std::vector<ProtocWarning>* warnings);

int protoc_get_services(
    char* protobuf_path, char* version,
    const std::vector<std::string>* include_paths,
    std::vector<std::pair<std::string, std::string>>* files_out,
    std::vector<ProtocError>* errors, std::vector<ProtocWarning>* warnings);
}  // end namespace grpc_tools
