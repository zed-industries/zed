// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ACCESSIBILITY_HISTOGRAM_MACROS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ACCESSIBILITY_HISTOGRAM_MACROS_H_

namespace blink {

#define UMA_HISTOGRAM_CUSTOM_EXACT_LINEAR(name, sample, min, max, buckets) \
  do {                                                                     \
    static_assert(!std::is_enum_v<std::decay_t<decltype(sample)>>,         \
                  "|sample| should not be an enum type!");                 \
    static_assert(!std::is_enum_v<std::decay_t<decltype(min)>>,            \
                  "|min| should not be an enum type!");                    \
    static_assert(!std::is_enum_v<std::decay_t<decltype(max)>>,            \
                  "|max| should not be an enum type!");                    \
    static_assert(!std::is_enum_v<std::decay_t<decltype(buckets)>>,        \
                  "|buckets| should not be an enum type!");                \
    STATIC_HISTOGRAM_POINTER_BLOCK(                                        \
        name, Add(sample),                                                 \
        base::LinearHistogram::FactoryGet(                                 \
            name, min, max, buckets,                                       \
            base::HistogramBase::kUmaTargetedHistogramFlag));              \
  } while (0)

#define STATIC_HISTOGRAM_POINTER_BLOCK(constant_histogram_name,             \
                                       histogram_add_method_invocation,     \
                                       histogram_factory_get_invocation)    \
  do {                                                                      \
    static std::atomic_uintptr_t atomic_histogram_pointer;                  \
    HISTOGRAM_POINTER_USE(                                                  \
        std::addressof(atomic_histogram_pointer), constant_histogram_name,  \
        histogram_add_method_invocation, histogram_factory_get_invocation); \
  } while (0)

#define HISTOGRAM_POINTER_USE(                                           \
    atomic_histogram_pointer, constant_histogram_name,                   \
    histogram_add_method_invocation, histogram_factory_get_invocation)   \
  do {                                                                   \
    base::HistogramBase* histogram_pointer(                              \
        reinterpret_cast<base::HistogramBase*>(                          \
            atomic_histogram_pointer->load(std::memory_order_acquire))); \
    if (!histogram_pointer) {                                            \
      histogram_pointer = histogram_factory_get_invocation;              \
      atomic_histogram_pointer->store(                                   \
          reinterpret_cast<uintptr_t>(histogram_pointer),                \
          std::memory_order_release);                                    \
    }                                                                    \
    if (DCHECK_IS_ON())                                                  \
      histogram_pointer->CheckName(constant_histogram_name);             \
    histogram_pointer->histogram_add_method_invocation;                  \
  } while (0)

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ACCESSIBILITY_HISTOGRAM_MACROS_H_
