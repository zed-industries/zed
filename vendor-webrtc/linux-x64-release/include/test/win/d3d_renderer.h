/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_WIN_D3D_RENDERER_H_
#define TEST_WIN_D3D_RENDERER_H_

#include <Windows.h>
#include <d3d9.h>
#pragma comment(lib, "d3d9.lib")  // located in DirectX SDK

#include "api/scoped_refptr.h"
#include "test/video_renderer.h"

namespace webrtc {
namespace test {

class D3dRenderer : public VideoRenderer {
 public:
  static D3dRenderer* Create(const char* window_title,
                             size_t width,
                             size_t height);
  virtual ~D3dRenderer();

  void OnFrame(const webrtc::VideoFrame& frame) override;

 private:
  D3dRenderer(size_t width, size_t height);

  static LRESULT WINAPI WindowProc(HWND hwnd,
                                   UINT msg,
                                   WPARAM wparam,
                                   LPARAM lparam);
  bool Init(const char* window_title);
  void Resize(size_t width, size_t height);
  void Destroy();

  size_t width_, height_;

  HWND hwnd_;
  webrtc::scoped_refptr<IDirect3D9> d3d_;
  webrtc::scoped_refptr<IDirect3DDevice9> d3d_device_;

  webrtc::scoped_refptr<IDirect3DTexture9> texture_;
  webrtc::scoped_refptr<IDirect3DVertexBuffer9> vertex_buffer_;
};
}  // namespace test
}  // namespace webrtc

#endif  // TEST_WIN_D3D_RENDERER_H_
