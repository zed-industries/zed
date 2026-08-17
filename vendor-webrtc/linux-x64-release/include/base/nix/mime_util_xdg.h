// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_NIX_MIME_UTIL_XDG_H_
#define BASE_NIX_MIME_UTIL_XDG_H_

#include <stdint.h>

#include <map>
#include <string>

#include "base/base_export.h"
#include "build/build_config.h"

namespace base {

class FilePath;

namespace nix {

// Mime type with weight.
struct WeightedMime {
  std::string mime_type;
  uint8_t weight;
};

// Map of file extension to weighted mime type.
using MimeTypeMap = std::map<std::string, WeightedMime, std::less<>>;

// Parses a file at `file_path` which should be in the same format as the
// /usr/share/mime/mime.cache file on Linux.
// https://specifications.freedesktop.org/shared-mime-info-spec/shared-mime-info-spec-0.21.html#idm46058238280640
// `out_mime_types` will be populated with keys that are a file extension and a
// value that is a MIME type if a higher weighted mime is found that currently
// exists. Returns true if there was a valid list parsed from the file and false
// otherwise.
BASE_EXPORT bool ParseMimeTypes(const FilePath& file_path,
                                MimeTypeMap& out_mime_types);

// Gets the mime type for a file at |filepath|.
//
// The mime type is calculated based only on the file name of |filepath|.  In
// particular |filepath| will not be touched on disk and |filepath| doesn't even
// have to exist.  This means that the function does not work for directories
// (i.e. |filepath| is assumed to be a path to a file).
//
// Note that this function might need to read from disk the mime-types data
// provided by the OS.  Therefore this function should not be called from
// threads that disallow blocking.
//
// If the mime type is unknown, this will return application/octet-stream.
BASE_EXPORT std::string GetFileMimeType(const FilePath& filepath);

}  // namespace nix
}  // namespace base

#endif  // BASE_NIX_MIME_UTIL_XDG_H_
