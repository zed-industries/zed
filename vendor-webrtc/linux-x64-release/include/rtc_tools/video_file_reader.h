/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_TOOLS_VIDEO_FILE_READER_H_
#define RTC_TOOLS_VIDEO_FILE_READER_H_

#include <stddef.h>

#include <cstdio>
#include <iterator>
#include <string>

#include "api/ref_count.h"
#include "api/scoped_refptr.h"
#include "api/video/video_frame_buffer.h"

namespace webrtc {
namespace test {

// Iterable class representing a sequence of I420 buffers. This class is not
// thread safe because it is expected to be backed by a file.
class Video : public RefCountInterface {
 public:
  class Iterator {
   public:
    typedef int value_type;
    typedef std::ptrdiff_t difference_type;
    typedef int* pointer;
    typedef int& reference;
    typedef std::input_iterator_tag iterator_category;

    Iterator(const scoped_refptr<const Video>& video, size_t index);
    Iterator(const Iterator& other);
    Iterator(Iterator&& other);
    Iterator& operator=(Iterator&&);
    Iterator& operator=(const Iterator&);
    ~Iterator();

    scoped_refptr<I420BufferInterface> operator*() const;
    bool operator==(const Iterator& other) const;
    bool operator!=(const Iterator& other) const;

    Iterator operator++(int);
    Iterator& operator++();

   private:
    scoped_refptr<const Video> video_;
    size_t index_;
  };

  Iterator begin() const;
  Iterator end() const;

  virtual int width() const = 0;
  virtual int height() const = 0;
  virtual size_t number_of_frames() const = 0;
  virtual scoped_refptr<I420BufferInterface> GetFrame(size_t index) const = 0;
};

scoped_refptr<Video> OpenY4mFile(const std::string& file_name);

scoped_refptr<Video> OpenYuvFile(const std::string& file_name,
                                 int width,
                                 int height);

// This is a helper function for the two functions above. It reads the file
// extension to determine whether it is a .yuv or a .y4m file.
scoped_refptr<Video> OpenYuvOrY4mFile(const std::string& file_name,
                                      int width,
                                      int height);

}  // namespace test
}  // namespace webrtc

#endif  // RTC_TOOLS_VIDEO_FILE_READER_H_
