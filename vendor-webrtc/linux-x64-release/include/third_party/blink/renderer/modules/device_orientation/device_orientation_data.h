/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_DATA_H_

#include <optional>

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DeviceOrientationEventInit;

class MODULES_EXPORT DeviceOrientationData final
    : public GarbageCollected<DeviceOrientationData> {
 public:
  static DeviceOrientationData* Create();
  static DeviceOrientationData* Create(const std::optional<double>& alpha,
                                       const std::optional<double>& beta,
                                       const std::optional<double>& gamma,
                                       bool absolute);
  static DeviceOrientationData* Create(const DeviceOrientationEventInit*);

  DeviceOrientationData();
  DeviceOrientationData(const std::optional<double>& alpha,
                        const std::optional<double>& beta,
                        const std::optional<double>& gamma,
                        bool absolute);

  void Trace(Visitor* visitor) const {}

  double Alpha() const;
  double Beta() const;
  double Gamma() const;
  bool Absolute() const;
  bool CanProvideAlpha() const;
  bool CanProvideBeta() const;
  bool CanProvideGamma() const;

  bool CanProvideEventData() const;

 private:
  std::optional<double> alpha_;
  std::optional<double> beta_;
  std::optional<double> gamma_;
  bool absolute_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_DATA_H_
