/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#ifndef GRPC_INTERNAL_COMPILER_GENERATOR_HELPERS_H
#define GRPC_INTERNAL_COMPILER_GENERATOR_HELPERS_H

#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <vector>

#include "src/compiler/config.h"
#include "src/compiler/proto_parser_helper.h"

namespace grpc_generator {

inline bool StripSuffix(std::string* filename, const std::string& suffix) {
  if (filename->length() >= suffix.length()) {
    size_t suffix_pos = filename->length() - suffix.length();
    if (filename->compare(suffix_pos, std::string::npos, suffix) == 0) {
      filename->resize(filename->size() - suffix.size());
      return true;
    }
  }

  return false;
}

inline bool StripPrefix(std::string* name, const std::string& prefix) {
  if (name->length() >= prefix.length()) {
    if (name->substr(0, prefix.size()) == prefix) {
      *name = name->substr(prefix.size());
      return true;
    }
  }
  return false;
}

inline std::string StripProto(std::string filename) {
  if (!StripSuffix(&filename, ".protodevel")) {
    StripSuffix(&filename, ".proto");
  }
  return filename;
}

inline std::string StringReplace(std::string str, const std::string& from,
                                 const std::string& to, bool replace_all) {
  size_t pos = 0;

  do {
    pos = str.find(from, pos);
    if (pos == std::string::npos) {
      break;
    }
    str.replace(pos, from.length(), to);
    pos += to.length();
  } while (replace_all);

  return str;
}

inline std::string StringReplace(std::string str, const std::string& from,
                                 const std::string& to) {
  return StringReplace(str, from, to, true);
}

inline std::vector<std::string> tokenize(const std::string& input,
                                         const std::string& delimiters) {
  std::vector<std::string> tokens;
  size_t pos, last_pos = 0;

  for (;;) {
    bool done = false;
    pos = input.find_first_of(delimiters, last_pos);
    if (pos == std::string::npos) {
      done = true;
      pos = input.length();
    }

    tokens.push_back(input.substr(last_pos, pos - last_pos));
    if (done) return tokens;

    last_pos = pos + 1;
  }
}

inline std::string CapitalizeFirstLetter(std::string s) {
  if (s.empty()) {
    return s;
  }
  s[0] = ::toupper(s[0]);
  return s;
}

inline std::string LowercaseFirstLetter(std::string s) {
  if (s.empty()) {
    return s;
  }
  s[0] = ::tolower(s[0]);
  return s;
}

inline std::string LowerUnderscoreToUpperCamel(std::string str) {
  std::vector<std::string> tokens = tokenize(str, "_");
  std::string result = "";
  for (unsigned int i = 0; i < tokens.size(); i++) {
    result += CapitalizeFirstLetter(tokens[i]);
  }
  return result;
}

inline std::string FileNameInUpperCamel(
    const grpc::protobuf::FileDescriptor* file, bool include_package_path) {
  std::vector<std::string> tokens =
      tokenize(StripProto(std::string(file->name())), "/");
  std::string result = "";
  if (include_package_path) {
    for (unsigned int i = 0; i < tokens.size() - 1; i++) {
      result += tokens[i] + "/";
    }
  }
  result += LowerUnderscoreToUpperCamel(tokens.back());
  return result;
}

inline std::string FileNameInUpperCamel(
    const grpc::protobuf::FileDescriptor* file) {
  return FileNameInUpperCamel(file, true);
}

enum MethodType {
  METHODTYPE_NO_STREAMING,
  METHODTYPE_CLIENT_STREAMING,
  METHODTYPE_SERVER_STREAMING,
  METHODTYPE_BIDI_STREAMING
};

inline MethodType GetMethodType(
    const grpc::protobuf::MethodDescriptor* method) {
  if (method->client_streaming()) {
    if (method->server_streaming()) {
      return METHODTYPE_BIDI_STREAMING;
    } else {
      return METHODTYPE_CLIENT_STREAMING;
    }
  } else {
    if (method->server_streaming()) {
      return METHODTYPE_SERVER_STREAMING;
    } else {
      return METHODTYPE_NO_STREAMING;
    }
  }
}

inline void Split(const std::string& s, char /*delim*/,
                  std::vector<std::string>* append_to) {
  std::istringstream iss(s);
  std::string piece;
  while (std::getline(iss, piece)) {
    append_to->push_back(piece);
  }
}

enum CommentType {
  COMMENTTYPE_LEADING,
  COMMENTTYPE_TRAILING,
  COMMENTTYPE_LEADING_DETACHED
};

// Get all the raw comments and append each line without newline to out.
template <typename DescriptorType>
inline void GetComment(const DescriptorType* desc, CommentType type,
                       std::vector<std::string>* out) {
  grpc::protobuf::SourceLocation location;
  if (!desc->GetSourceLocation(&location)) {
    return;
  }
  if (type == COMMENTTYPE_LEADING || type == COMMENTTYPE_TRAILING) {
    const std::string& comments = type == COMMENTTYPE_LEADING
                                      ? location.leading_comments
                                      : location.trailing_comments;
    Split(comments, '\n', out);
  } else if (type == COMMENTTYPE_LEADING_DETACHED) {
    for (unsigned int i = 0; i < location.leading_detached_comments.size();
         i++) {
      Split(location.leading_detached_comments[i], '\n', out);
      out->push_back("");
    }
  } else {
    std::cerr << "Unknown comment type " << type << std::endl;
    abort();
  }
}

// Each raw comment line without newline is appended to out.
// For file level leading and detached leading comments, we return comments
// above syntax line. Return nothing for trailing comments.
template <>
inline void GetComment(const grpc::protobuf::FileDescriptor* desc,
                       CommentType type, std::vector<std::string>* out) {
  if (type == COMMENTTYPE_TRAILING) {
    return;
  }
  grpc::protobuf::SourceLocation location;
  std::vector<int> path;
  path.push_back(grpc::protobuf::FileDescriptorProto::kSyntaxFieldNumber);
  if (!desc->GetSourceLocation(path, &location)) {
    return;
  }
  if (type == COMMENTTYPE_LEADING) {
    Split(location.leading_comments, '\n', out);
  } else if (type == COMMENTTYPE_LEADING_DETACHED) {
    for (unsigned int i = 0; i < location.leading_detached_comments.size();
         i++) {
      Split(location.leading_detached_comments[i], '\n', out);
      out->push_back("");
    }
  } else {
    std::cerr << "Unknown comment type " << type << std::endl;
    abort();
  }
}

// Add prefix and newline to each comment line and concatenate them together.
// Make sure there is a space after the prefix unless the line is empty.
inline std::string GenerateCommentsWithPrefix(
    const std::vector<std::string>& in, const std::string& prefix) {
  std::ostringstream oss;
  for (auto it = in.begin(); it != in.end(); it++) {
    const std::string& elem = *it;
    if (elem.empty()) {
      oss << prefix << "\n";
    } else if (elem[0] == ' ') {
      oss << prefix << EscapeVariableDelimiters(elem) << "\n";
    } else {
      oss << prefix << " " << EscapeVariableDelimiters(elem) << "\n";
    }
  }
  return oss.str();
}

template <typename DescriptorType>
inline std::string GetPrefixedComments(const DescriptorType* desc, bool leading,
                                       const std::string& prefix) {
  std::vector<std::string> out;
  if (leading) {
    grpc_generator::GetComment(
        desc, grpc_generator::COMMENTTYPE_LEADING_DETACHED, &out);
    std::vector<std::string> leading;
    grpc_generator::GetComment(desc, grpc_generator::COMMENTTYPE_LEADING,
                               &leading);
    out.insert(out.end(), leading.begin(), leading.end());
  } else {
    grpc_generator::GetComment(desc, grpc_generator::COMMENTTYPE_TRAILING,
                               &out);
  }
  return GenerateCommentsWithPrefix(out, prefix);
}

}  // namespace grpc_generator

#endif  // GRPC_INTERNAL_COMPILER_GENERATOR_HELPERS_H
