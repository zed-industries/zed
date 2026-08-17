/*
 *  Copyright (C) 2012 Samsung Electronics
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Library General Public
 *  License as published by the Free Software Foundation; either
 *  version 2 of the License, or (at your option) any later version.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Library General Public License for more details.
 *
 *  You should have received a copy of the GNU Library General Public License
 *  along with this library; see the file COPYING.LIB.  If not, write to
 *  the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 *  Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_VIBRATION_VIBRATION_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_VIBRATION_VIBRATION_CONTROLLER_H_

#include "services/device/public/mojom/vibration_manager.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/page/page_visibility_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Navigator;
class V8UnionUnsignedLongOrUnsignedLongSequence;

class MODULES_EXPORT VibrationController final
    : public GarbageCollected<VibrationController>,
      public Supplement<Navigator>,
      public ExecutionContextLifecycleObserver,
      public PageVisibilityObserver {
 public:
  using VibrationPattern = Vector<unsigned>;

  static const char kSupplementName[];
  static VibrationController& From(Navigator&);

  static bool vibrate(Navigator&, unsigned time);
  static bool vibrate(Navigator&, const VibrationPattern&);

  explicit VibrationController(Navigator&);

  VibrationController(const VibrationController&) = delete;
  VibrationController& operator=(const VibrationController&) = delete;

  ~VibrationController() override;

  static VibrationPattern SanitizeVibrationPattern(
      const V8UnionUnsignedLongOrUnsignedLongSequence* input);

  void DoVibrate(TimerBase*);
  void DidVibrate();

  // Cancels the ongoing vibration if there is one.
  void Cancel();
  void DidCancel();

  // Whether a pattern is being processed. If this is true, the vibration
  // hardware may currently be active, but during a pause it may be inactive.
  bool IsRunning() const { return is_running_; }

  VibrationPattern Pattern() const { return pattern_; }

  void Trace(Visitor*) const override;

 private:
  // Inherited from ExecutionContextLifecycleObserver.
  void ContextDestroyed() override;

  // Inherited from PageVisibilityObserver.
  void PageVisibilityChanged() override;

  bool Vibrate(const VibrationPattern&);

  // Remote to VibrationManager mojo interface. This is reset in
  // |contextDestroyed| and must not be called or recreated after it is reset.
  //
  // TODO(crbug.com/1116948): Remove kForceWithoutContextObserver parameter
  // after hooking disconnect handler in js is implemented in
  // MojoInterfaceInterceptor.
  // See: third_party/blink/web_tests/vibration/vibration-iframe.html
  HeapMojoRemote<device::mojom::blink::VibrationManager,
                 HeapMojoWrapperMode::kForceWithoutContextObserver>
      vibration_manager_;

  // Timer for calling |doVibrate| after a delay. It is safe to call
  // |startOneshot| when the timer is already running: it may affect the time
  // at which it fires, but |doVibrate| will still be called only once.
  HeapTaskRunnerTimer<VibrationController> timer_do_vibrate_;

  // Whether a pattern is being processed. The vibration hardware may
  // currently be active, or during a pause it may be inactive.
  bool is_running_;

  // Whether an async mojo call to cancel is pending.
  bool is_calling_cancel_;

  // Whether an async mojo call to vibrate is pending.
  bool is_calling_vibrate_;

  VibrationPattern pattern_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_VIBRATION_VIBRATION_CONTROLLER_H_
