/*
    Copyright (C) 2008 Nokia Corporation and/or its subsidiary(-ies)

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_NAVIGATOR_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_NAVIGATOR_BASE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/frame/navigator_concurrent_hardware.h"
#include "third_party/blink/renderer/core/frame/navigator_device_memory.h"
#include "third_party/blink/renderer/core/frame/navigator_id.h"
#include "third_party/blink/renderer/core/frame/navigator_language.h"
#include "third_party/blink/renderer/core/frame/navigator_on_line.h"
#include "third_party/blink/renderer/core/frame/navigator_ua.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

// NavigatorBase is a helper for shared logic between Navigator and
// WorkerNavigator. It is also a Supplementable, and can therefore be used for
// classes that need to Supplement both Navigator and WorkerNavigator.
class CORE_EXPORT NavigatorBase : public ScriptWrappable,
                                  public NavigatorConcurrentHardware,
                                  public NavigatorDeviceMemory,
                                  public NavigatorID,
                                  public NavigatorLanguage,
                                  public NavigatorOnLine,
                                  public NavigatorUA,
                                  public ExecutionContextClient,
                                  public Supplementable<NavigatorBase> {
 public:
  explicit NavigatorBase(ExecutionContext* context);

  // NavigatorID override
  String userAgent() const override;
  String platform() const override;
  void Trace(Visitor* visitor) const override;

  unsigned int hardwareConcurrency() const override;

 protected:
  ExecutionContext* GetUAExecutionContext() const override;
  UserAgentMetadata GetUserAgentMetadata() const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_NAVIGATOR_BASE_H_
