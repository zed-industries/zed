/*
 * Copyright 2025 LiveKit, Inc.
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
 */

#include "livekit/video_frame_buffer.h"

#include "api/make_ref_counted.h"

namespace livekit_ffi {

VideoFrameBuffer::VideoFrameBuffer(
    webrtc::scoped_refptr<webrtc::VideoFrameBuffer> buffer)
    : buffer_(std::move(buffer)) {}

VideoFrameBufferType VideoFrameBuffer::buffer_type() const {
  return static_cast<VideoFrameBufferType>(buffer_->type());
}

unsigned int VideoFrameBuffer::width() const {
  return buffer_->width();
}

unsigned int VideoFrameBuffer::height() const {
  return buffer_->height();
}

std::unique_ptr<I420Buffer> VideoFrameBuffer::to_i420() const {
  return std::make_unique<I420Buffer>(buffer_->ToI420());
}

// const_cast is valid here because we take the ownership on the rust side
std::unique_ptr<I420Buffer> VideoFrameBuffer::get_i420() {
  return std::make_unique<I420Buffer>(
      webrtc::scoped_refptr<webrtc::I420BufferInterface>(
          const_cast<webrtc::I420BufferInterface*>(buffer_->GetI420())));
}

std::unique_ptr<I420ABuffer> VideoFrameBuffer::get_i420a() {
  return std::make_unique<I420ABuffer>(
      webrtc::scoped_refptr<webrtc::I420ABufferInterface>(
          const_cast<webrtc::I420ABufferInterface*>(buffer_->GetI420A())));
}

std::unique_ptr<I422Buffer> VideoFrameBuffer::get_i422() {
  return std::make_unique<I422Buffer>(
      webrtc::scoped_refptr<webrtc::I422BufferInterface>(
          const_cast<webrtc::I422BufferInterface*>(buffer_->GetI422())));
}

std::unique_ptr<I444Buffer> VideoFrameBuffer::get_i444() {
  return std::make_unique<I444Buffer>(
      webrtc::scoped_refptr<webrtc::I444BufferInterface>(
          const_cast<webrtc::I444BufferInterface*>(buffer_->GetI444())));
}

std::unique_ptr<I010Buffer> VideoFrameBuffer::get_i010() {
  return std::make_unique<I010Buffer>(
      webrtc::scoped_refptr<webrtc::I010BufferInterface>(
          const_cast<webrtc::I010BufferInterface*>(buffer_->GetI010())));
}

std::unique_ptr<NV12Buffer> VideoFrameBuffer::get_nv12() {
  return std::make_unique<NV12Buffer>(
      webrtc::scoped_refptr<webrtc::NV12BufferInterface>(
          const_cast<webrtc::NV12BufferInterface*>(buffer_->GetNV12())));
}

webrtc::scoped_refptr<webrtc::VideoFrameBuffer> VideoFrameBuffer::get() const {
  return buffer_;
}

PlanarYuvBuffer::PlanarYuvBuffer(
    webrtc::scoped_refptr<webrtc::PlanarYuvBuffer> buffer)
    : VideoFrameBuffer(buffer) {}

unsigned int PlanarYuvBuffer::chroma_width() const {
  return buffer()->ChromaWidth();
}

unsigned int PlanarYuvBuffer::chroma_height() const {
  return buffer()->ChromaHeight();
}

unsigned int PlanarYuvBuffer::stride_y() const {
  return buffer()->StrideY();
}

unsigned int PlanarYuvBuffer::stride_u() const {
  return buffer()->StrideU();
}

unsigned int PlanarYuvBuffer::stride_v() const {
  return buffer()->StrideV();
}

webrtc::PlanarYuvBuffer* PlanarYuvBuffer::buffer() const {
  return static_cast<webrtc::PlanarYuvBuffer*>(buffer_.get());
}

PlanarYuv8Buffer::PlanarYuv8Buffer(
    webrtc::scoped_refptr<webrtc::PlanarYuv8Buffer> buffer)
    : PlanarYuvBuffer(buffer) {}

const uint8_t* PlanarYuv8Buffer::data_y() const {
  return buffer()->DataY();
}

const uint8_t* PlanarYuv8Buffer::data_u() const {
  return buffer()->DataU();
}

const uint8_t* PlanarYuv8Buffer::data_v() const {
  return buffer()->DataV();
}

webrtc::PlanarYuv8Buffer* PlanarYuv8Buffer::buffer() const {
  return static_cast<webrtc::PlanarYuv8Buffer*>(buffer_.get());
}

PlanarYuv16BBuffer::PlanarYuv16BBuffer(
    webrtc::scoped_refptr<webrtc::PlanarYuv16BBuffer> buffer)
    : PlanarYuvBuffer(buffer) {}

const uint16_t* PlanarYuv16BBuffer::data_y() const {
  return buffer()->DataY();
}

const uint16_t* PlanarYuv16BBuffer::data_u() const {
  return buffer()->DataU();
}

const uint16_t* PlanarYuv16BBuffer::data_v() const {
  return buffer()->DataV();
}

webrtc::PlanarYuv16BBuffer* PlanarYuv16BBuffer::buffer() const {
  return static_cast<webrtc::PlanarYuv16BBuffer*>(buffer_.get());
}

BiplanarYuvBuffer::BiplanarYuvBuffer(
    webrtc::scoped_refptr<webrtc::BiplanarYuvBuffer> buffer)
    : VideoFrameBuffer(buffer) {}

unsigned int BiplanarYuvBuffer::chroma_width() const {
  return buffer()->ChromaWidth();
}

unsigned int BiplanarYuvBuffer::chroma_height() const {
  return buffer()->ChromaHeight();
}

unsigned int BiplanarYuvBuffer::stride_y() const {
  return buffer()->StrideY();
}

unsigned int BiplanarYuvBuffer::stride_uv() const {
  return buffer()->StrideUV();
}

webrtc::BiplanarYuvBuffer* BiplanarYuvBuffer::buffer() const {
  return static_cast<webrtc::BiplanarYuvBuffer*>(buffer_.get());
}

BiplanarYuv8Buffer::BiplanarYuv8Buffer(
    webrtc::scoped_refptr<webrtc::BiplanarYuv8Buffer> buffer)
    : BiplanarYuvBuffer(buffer) {}

const uint8_t* BiplanarYuv8Buffer::data_y() const {
  return buffer()->DataY();
}

const uint8_t* BiplanarYuv8Buffer::data_uv() const {
  return buffer()->DataUV();
}

webrtc::BiplanarYuv8Buffer* BiplanarYuv8Buffer::buffer() const {
  return static_cast<webrtc::BiplanarYuv8Buffer*>(buffer_.get());
}

I420Buffer::I420Buffer(webrtc::scoped_refptr<webrtc::I420BufferInterface> buffer)
    : PlanarYuv8Buffer(buffer) {}

webrtc::I420BufferInterface* I420Buffer::buffer() const {
  return static_cast<webrtc::I420BufferInterface*>(buffer_.get());
}

std::unique_ptr<I420Buffer> I420Buffer::scale(int scaled_width,
                                              int scaled_height) const {
  rtc::scoped_refptr<webrtc::VideoFrameBuffer> result =
      buffer()->Scale(scaled_width, scaled_height);
  return std::make_unique<I420Buffer>(
      rtc::scoped_refptr<webrtc::I420BufferInterface>(
          const_cast<webrtc::I420BufferInterface*>(result->GetI420())));
}

I420ABuffer::I420ABuffer(
    webrtc::scoped_refptr<webrtc::I420ABufferInterface> buffer)
    : I420Buffer(buffer) {}

unsigned int I420ABuffer::stride_a() const {
  return buffer()->StrideA();
}

const uint8_t* I420ABuffer::data_a() const {
  return buffer()->DataA();
}

webrtc::I420ABufferInterface* I420ABuffer::buffer() const {
  return static_cast<webrtc::I420ABufferInterface*>(buffer_.get());
}

std::unique_ptr<I420ABuffer> I420ABuffer::scale(int scaled_width,
                                                int scaled_height) const {
  rtc::scoped_refptr<webrtc::VideoFrameBuffer> result =
      buffer()->Scale(scaled_width, scaled_height);
  return std::make_unique<I420ABuffer>(
      rtc::scoped_refptr<webrtc::I420ABufferInterface>(
          const_cast<webrtc::I420ABufferInterface*>(result->GetI420A())));
}

I422Buffer::I422Buffer(rtc::scoped_refptr<webrtc::I422BufferInterface> buffer)
    : PlanarYuv8Buffer(buffer) {}

webrtc::I422BufferInterface* I422Buffer::buffer() const {
  return static_cast<webrtc::I422BufferInterface*>(buffer_.get());
}

std::unique_ptr<I422Buffer> I422Buffer::scale(int scaled_width,
                                              int scaled_height) const {
  rtc::scoped_refptr<webrtc::VideoFrameBuffer> result =
      buffer()->Scale(scaled_width, scaled_height);
  return std::make_unique<I422Buffer>(
      rtc::scoped_refptr<webrtc::I422BufferInterface>(
          const_cast<webrtc::I422BufferInterface*>(result->GetI422())));
}

I444Buffer::I444Buffer(rtc::scoped_refptr<webrtc::I444BufferInterface> buffer)
    : PlanarYuv8Buffer(buffer) {}

webrtc::I444BufferInterface* I444Buffer::buffer() const {
  return static_cast<webrtc::I444BufferInterface*>(buffer_.get());
}

std::unique_ptr<I444Buffer> I444Buffer::scale(int scaled_width,
                                              int scaled_height) const {
  rtc::scoped_refptr<webrtc::VideoFrameBuffer> result =
      buffer()->Scale(scaled_width, scaled_height);
  return std::make_unique<I444Buffer>(
      rtc::scoped_refptr<webrtc::I444BufferInterface>(
          const_cast<webrtc::I444BufferInterface*>(result->GetI444())));
}

I010Buffer::I010Buffer(rtc::scoped_refptr<webrtc::I010BufferInterface> buffer)
    : PlanarYuv16BBuffer(buffer) {}

webrtc::I010BufferInterface* I010Buffer::buffer() const {
  return static_cast<webrtc::I010BufferInterface*>(buffer_.get());
}

std::unique_ptr<I010Buffer> I010Buffer::scale(int scaled_width,
                                              int scaled_height) const {
  rtc::scoped_refptr<webrtc::VideoFrameBuffer> result =
      buffer()->Scale(scaled_width, scaled_height);
  return std::make_unique<I010Buffer>(
      rtc::scoped_refptr<webrtc::I010BufferInterface>(
          const_cast<webrtc::I010BufferInterface*>(result->GetI010())));
}

NV12Buffer::NV12Buffer(rtc::scoped_refptr<webrtc::NV12BufferInterface> buffer)
    : BiplanarYuv8Buffer(buffer) {}

webrtc::NV12BufferInterface* NV12Buffer::buffer() const {
  return static_cast<webrtc::NV12BufferInterface*>(buffer_.get());
}

std::unique_ptr<NV12Buffer> NV12Buffer::scale(int scaled_width,
                                              int scaled_height) const {
  rtc::scoped_refptr<webrtc::VideoFrameBuffer> result =
      buffer()->Scale(scaled_width, scaled_height);
  return std::make_unique<NV12Buffer>(
      rtc::scoped_refptr<webrtc::NV12BufferInterface>(
          const_cast<webrtc::NV12BufferInterface*>(result->GetNV12())));
}

std::unique_ptr<I420Buffer> copy_i420_buffer(
    const std::unique_ptr<I420Buffer>& i420) {
  return std::make_unique<I420Buffer>(webrtc::I420Buffer::Copy(*i420->get()));
}

std::unique_ptr<I420Buffer> new_i420_buffer(int width,
                                            int height,
                                            int stride_y,
                                            int stride_u,
                                            int stride_v) {
  return std::make_unique<I420Buffer>(
      webrtc::I420Buffer::Create(width, height, stride_y, stride_u, stride_v));
}

std::unique_ptr<I422Buffer> new_i422_buffer(int width,
                                            int height,
                                            int stride_y,
                                            int stride_u,
                                            int stride_v) {
  return std::make_unique<I422Buffer>(
      webrtc::I422Buffer::Create(width, height, stride_y, stride_u, stride_v));
}

std::unique_ptr<I444Buffer> new_i444_buffer(int width,
                                            int height,
                                            int stride_y,
                                            int stride_u,
                                            int stride_v) {
  return std::make_unique<I444Buffer>(
      webrtc::I444Buffer::Create(width, height, stride_y, stride_u, stride_v));
}

std::unique_ptr<I010Buffer> new_i010_buffer(int width,
                                            int height,
                                            int stride_y,
                                            int stride_u,
                                            int stride_v) {
  return std::make_unique<I010Buffer>(webrtc::make_ref_counted<webrtc::I010Buffer>(
      width, height, stride_y, stride_u, stride_v));
}

std::unique_ptr<NV12Buffer> new_nv12_buffer(int width,
                                            int height,
                                            int stride_y,
                                            int stride_uv) {
  return std::make_unique<NV12Buffer>(
      webrtc::NV12Buffer::Create(width, height, stride_y, stride_uv));
}

#ifndef __APPLE__

std::unique_ptr<VideoFrameBuffer> new_native_buffer_from_platform_image_buffer(
    PlatformImageBuffer *buffer
) {
  return nullptr;
}

PlatformImageBuffer* native_buffer_to_platform_image_buffer(
    const std::unique_ptr<VideoFrameBuffer> &buffer
) {
  return nullptr;
}

#endif

}  // namespace livekit_ffi
