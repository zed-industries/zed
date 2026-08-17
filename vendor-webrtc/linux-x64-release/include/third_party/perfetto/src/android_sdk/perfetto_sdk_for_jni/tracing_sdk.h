/*
 * Copyright (C) 2025 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_ANDROID_SDK_PERFETTO_SDK_FOR_JNI_TRACING_SDK_H_
#define SRC_ANDROID_SDK_PERFETTO_SDK_FOR_JNI_TRACING_SDK_H_

#include <stdint.h>

#include <sys/types.h>

#include <optional>
#include <string>
#include <vector>

#include "perfetto/public/tracing_session.h"
#include "perfetto/public/track_event.h"

// Macro copied from
// https://source.corp.google.com/h/googleplex-android/platform/superproject/main/+/main:system/libbase/include/android-base/macros.h;l=45;drc=bd641075ad60ed703baf59f63a9153d96d96b98e
// A macro to disallow the copy constructor and operator= functions
// This must be placed in the private: declarations for a class.
//
// For disallowing only assign or copy, delete the relevant operator or
// constructor, for example:
// void operator=(const TypeName&) = delete;
// Note, that most uses of DISALLOW_ASSIGN and DISALLOW_COPY are broken
// semantically, one should either use disallow both or neither. Try to
// avoid these in new code.
#define DISALLOW_COPY_AND_ASSIGN(TypeName) \
  TypeName(const TypeName&) = delete;      \
  void operator=(const TypeName&) = delete

/**
 * The objects declared here are intended to be managed by Java.
 * This means the Java Garbage Collector is responsible for freeing the
 * underlying native resources.
 *
 * The static methods prefixed with `delete_` are special. They are designed to
 * be invoked by Java through the `NativeAllocationRegistry` when the
 * corresponding Java object becomes unreachable.  These methods act as
 * callbacks to ensure proper deallocation of native resources.
 */
namespace perfetto {
namespace sdk_for_jni {
/**
 * @brief Initializes the global perfetto instance.
 * @param backend_in_process use in-process or system backend
 */
void register_perfetto(bool backend_in_process = false);

/**
 * @brief Represents extra data associated with a trace event.
 * This class manages a collection of PerfettoTeHlExtra pointers.
 */
class Extra;

/**
 * @brief Emits a trace event.
 * @param type The type of the event.
 * @param cat The category of the event.
 * @param name The name of the event.
 * @param extra Pointer to Extra data.
 */
void trace_event(int type,
                 const PerfettoTeCategory* cat,
                 const char* name,
                 Extra* extra);

/**
 * @brief Gets the process track UUID.
 */
uint64_t get_process_track_uuid();

/**
 * @brief Gets the thread track UUID for a given PID.
 */
uint64_t get_thread_track_uuid(pid_t tid);

/**
 * @brief Holder for all the other classes in the file.
 */
class Extra {
 public:
  Extra();
  void push_extra(PerfettoTeHlExtra* extra);
  void pop_extra();
  void clear_extras();
  static void delete_extra(Extra* extra);

  PerfettoTeHlExtra* const* get() const;

 private:
  DISALLOW_COPY_AND_ASSIGN(Extra);

  // These PerfettoTeHlExtra pointers are really pointers to all the other
  // types of extras: Category, DebugArg, Counter etc. Those objects are
  // individually managed by Java.
  std::vector<PerfettoTeHlExtra*> extras_;
};

/**
 * @brief Represents a trace event category.
 */
class Category {
 public:
  explicit Category(const std::string& name);

  Category(const std::string& name, const std::vector<std::string>& tags);

  ~Category();

  void register_category();

  void unregister_category();

  bool is_category_enabled();

  static void delete_category(Category* category);

  const PerfettoTeCategory* get() const;

 private:
  DISALLOW_COPY_AND_ASSIGN(Category);
  PerfettoTeCategory category_;
  const std::string name_;
  const std::vector<std::string> tags_;
  std::vector<const char*> tags_data_;
};

/**
 * @brief Represents one end of a flow between two events.
 */
class Flow {
 public:
  Flow();

  void set_process_flow(uint64_t id);
  void set_process_terminating_flow(uint64_t id);
  static void delete_flow(Flow* flow);

  const PerfettoTeHlExtraFlow* get() const;

 private:
  DISALLOW_COPY_AND_ASSIGN(Flow);
  PerfettoTeHlExtraFlow flow_;
};

/**
 * @brief Represents a named track.
 */
class NamedTrack {
 public:
  NamedTrack(uint64_t id, uint64_t parent_uuid, const std::string& name);

  static void delete_track(NamedTrack* track);

  const PerfettoTeHlExtraNamedTrack* get() const;

 private:
  DISALLOW_COPY_AND_ASSIGN(NamedTrack);
  const std::string name_;
  PerfettoTeHlExtraNamedTrack track_;
};

/**
 * @brief Represents a registered track.
 */
class RegisteredTrack {
 public:
  RegisteredTrack(uint64_t id,
                  uint64_t parent_uuid,
                  const std::string& name,
                  bool is_counter);
  ~RegisteredTrack();

  void register_track();
  void unregister_track();
  static void delete_track(RegisteredTrack* track);

  const PerfettoTeHlExtraRegisteredTrack* get() const;

 private:
  DISALLOW_COPY_AND_ASSIGN(RegisteredTrack);
  PerfettoTeRegisteredTrack registered_track_;
  PerfettoTeHlExtraRegisteredTrack track_;
  const std::string name_;
  const uint64_t id_;
  const uint64_t parent_uuid_;
  const bool is_counter_;
};

/**
 * @brief Represents a counter track event.
 * @tparam T The data type of the counter (int64_t or double).
 */
template <typename T>
class Counter {
 public:
  template <typename>
  struct always_false : std::false_type {};

  template <class I>
  struct type_identity {
    using type = I;
  };

  struct TypeMap {
    template <typename CounterT>
    static constexpr auto get_counter_type() {
      if constexpr (std::is_same_v<CounterT, int64_t>) {
        return type_identity<PerfettoTeHlExtraCounterInt64>{};
      } else if constexpr (std::is_same_v<CounterT, double>) {
        return type_identity<PerfettoTeHlExtraCounterDouble>{};
      } else {
        return type_identity<void>{};
      }
    }
    using type = typename decltype(get_counter_type<T>())::type;

    static constexpr int enum_value = []() {
      if constexpr (std::is_same_v<T, int64_t>) {
        return PERFETTO_TE_HL_EXTRA_TYPE_COUNTER_INT64;
      } else if constexpr (std::is_same_v<T, double>) {
        return PERFETTO_TE_HL_EXTRA_TYPE_COUNTER_DOUBLE;
      } else {
        static_assert(always_false<T>::value, "Unsupported type");
        return 0;  // Never reached, just to satisfy return type
      }
    }();
  };

  Counter() {
    static_assert(!std::is_same_v<typename TypeMap::type, void>,
                  "Unsupported type for Counter");

    typename TypeMap::type counter;
    counter.header = {TypeMap::enum_value};
    counter_ = std::move(counter);
  }

  void set_value(T value) {
    if constexpr (std::is_same_v<T, int64_t>) {
      counter_.value = value;
    } else if constexpr (std::is_same_v<T, double>) {
      counter_.value = value;
    }
  }

  static void delete_counter(Counter* counter) { delete counter; }

  const typename TypeMap::type* get() const { return &counter_; }

 private:
  DISALLOW_COPY_AND_ASSIGN(Counter);
  typename TypeMap::type counter_;
};

/**
 * @brief Represents a debug argument for a trace event.
 * @tparam T The data type of the argument (bool, int64_t, double, const char*).
 */
template <typename T>
class DebugArg {
 public:
  template <typename>
  struct always_false : std::false_type {};

  template <class I>
  struct type_identity {
    using type = I;
  };

  struct TypeMap {
    template <typename DebugArgT>
    static constexpr auto get_debug_arg_type() {
      if constexpr (std::is_same_v<DebugArgT, bool>) {
        return type_identity<PerfettoTeHlExtraDebugArgBool>{};
      } else if constexpr (std::is_same_v<DebugArgT, int64_t>) {
        return type_identity<PerfettoTeHlExtraDebugArgInt64>{};
      } else if constexpr (std::is_same_v<DebugArgT, double>) {
        return type_identity<PerfettoTeHlExtraDebugArgDouble>{};
      } else if constexpr (std::is_same_v<DebugArgT, const char*>) {
        return type_identity<PerfettoTeHlExtraDebugArgString>{};
      } else {
        return type_identity<void>{};
      }
    }
    using type = typename decltype(get_debug_arg_type<T>())::type;

    static constexpr int enum_value = []() {
      if constexpr (std::is_same_v<T, bool>) {
        return PERFETTO_TE_HL_EXTRA_TYPE_DEBUG_ARG_BOOL;
      } else if constexpr (std::is_same_v<T, int64_t>) {
        return PERFETTO_TE_HL_EXTRA_TYPE_DEBUG_ARG_INT64;
      } else if constexpr (std::is_same_v<T, double>) {
        return PERFETTO_TE_HL_EXTRA_TYPE_DEBUG_ARG_DOUBLE;
      } else if constexpr (std::is_same_v<T, const char*>) {
        return PERFETTO_TE_HL_EXTRA_TYPE_DEBUG_ARG_STRING;
      } else {
        static_assert(always_false<T>::value, "Unsupported type");
        return 0;  // Never reached, just to satisfy return type
      }
    }();
  };

  DebugArg(const std::string& name) : name_(name) {
    static_assert(!std::is_same_v<typename TypeMap::type, void>,
                  "Unsupported type for DebugArg");

    typename TypeMap::type arg;
    arg.header = {TypeMap::enum_value};
    arg.name = name_.c_str();
    arg_ = std::move(arg);
  }

  void set_value(T value) {
    if constexpr (std::is_same_v<T, const char*>) {
      arg_.value = value;
    } else if constexpr (std::is_same_v<T, int64_t>) {
      arg_.value = value;
    } else if constexpr (std::is_same_v<T, bool>) {
      arg_.value = value;
    } else if constexpr (std::is_same_v<T, double>) {
      arg_.value = value;
    }
  }

  static void delete_arg(DebugArg* arg) { delete arg; }

  const typename TypeMap::type* get() const { return &arg_; }

 private:
  DISALLOW_COPY_AND_ASSIGN(DebugArg);
  typename TypeMap::type arg_;
  const std::string name_;
};

template <typename T>
class ProtoField {
 public:
  template <typename>
  struct always_false : std::false_type {};

  template <class I>
  struct type_identity {
    using type = I;
  };

  struct TypeMap {
    template <typename ProtoT>
    static constexpr auto get_proto_type() {
      if constexpr (std::is_same_v<ProtoT, int64_t>) {
        return type_identity<PerfettoTeHlProtoFieldVarInt>{};
      } else if constexpr (std::is_same_v<ProtoT, double>) {
        return type_identity<PerfettoTeHlProtoFieldDouble>{};
      } else if constexpr (std::is_same_v<ProtoT, const char*>) {
        return type_identity<PerfettoTeHlProtoFieldCstr>{};
      } else {
        return type_identity<void>{};
      }
    }
    using type = typename decltype(get_proto_type<T>())::type;

    static constexpr PerfettoTeHlProtoFieldType enum_value = []() {
      if constexpr (std::is_same_v<T, int64_t>) {
        return PERFETTO_TE_HL_PROTO_TYPE_VARINT;
      } else if constexpr (std::is_same_v<T, double>) {
        return PERFETTO_TE_HL_PROTO_TYPE_DOUBLE;
      } else if constexpr (std::is_same_v<T, const char*>) {
        return PERFETTO_TE_HL_PROTO_TYPE_CSTR;
      } else {
        static_assert(always_false<T>::value, "Unsupported type");
        return 0;  // Never reached, just to satisfy return type
      }
    }();
  };

  ProtoField() {
    static_assert(!std::is_same_v<typename TypeMap::type, void>,
                  "Unsupported type for ProtoField");

    typename TypeMap::type arg;
    arg.header.type = TypeMap::enum_value;
    arg_ = std::move(arg);
  }

  void set_value(uint32_t id, T value) {
    if constexpr (std::is_same_v<T, int64_t>) {
      arg_.header.id = id;
      arg_.value = value;
    } else if constexpr (std::is_same_v<T, double>) {
      arg_.header.id = id;
      arg_.value = value;
    } else if constexpr (std::is_same_v<T, const char*>) {
      arg_.header.id = id;
      arg_.str = value;
    }
  }

  static void delete_field(ProtoField* field) { delete field; }

  const typename TypeMap::type* get() const { return &arg_; }

 private:
  DISALLOW_COPY_AND_ASSIGN(ProtoField);
  typename TypeMap::type arg_;
};

class ProtoFieldNested {
 public:
  ProtoFieldNested();

  void add_field(PerfettoTeHlProtoField* field);
  void set_id(uint32_t id);
  static void delete_field(ProtoFieldNested* field);

  const PerfettoTeHlProtoFieldNested* get() const;

 private:
  DISALLOW_COPY_AND_ASSIGN(ProtoFieldNested);
  PerfettoTeHlProtoFieldNested field_;
  // These PerfettoTeHlProtoField pointers are really pointers to all the other
  // types of protos: PerfettoTeHlProtoFieldVarInt,
  // PerfettoTeHlProtoFieldVarInt, PerfettoTeHlProtoFieldVarInt,
  // PerfettoTeHlProtoFieldNested. Those objects are individually managed by
  // Java.
  std::vector<PerfettoTeHlProtoField*> fields_;
};

class Proto {
 public:
  Proto();

  void add_field(PerfettoTeHlProtoField* field);
  void clear_fields();
  static void delete_proto(Proto* proto);

  const PerfettoTeHlExtraProtoFields* get() const;

 private:
  DISALLOW_COPY_AND_ASSIGN(Proto);
  PerfettoTeHlExtraProtoFields proto_;
  // These PerfettoTeHlProtoField pointers are really pointers to all the other
  // types of protos: PerfettoTeHlProtoFieldVarInt,
  // PerfettoTeHlProtoFieldVarInt, PerfettoTeHlProtoFieldVarInt,
  // PerfettoTeHlProtoFieldNested. Those objects are individually managed by
  // Java.
  std::vector<PerfettoTeHlProtoField*> fields_;
};

class Session {
 public:
  Session(bool is_backend_in_process, void* buf, size_t len);
  ~Session();
  Session(const Session&) = delete;
  Session& operator=(const Session&) = delete;

  bool FlushBlocking(uint32_t timeout_ms);
  void StopBlocking();
  std::vector<uint8_t> ReadBlocking();

  static void delete_session(Session* session);

  struct PerfettoTracingSessionImpl* session_ = nullptr;
};

/**
 * @brief Activates a trigger.
 * @param name The name of the trigger.
 * @param ttl_ms The time-to-live of the trigger in milliseconds.
 */
void activate_trigger(const char* name, uint32_t ttl_ms);
}  // namespace sdk_for_jni
}  // namespace perfetto

#endif  // SRC_ANDROID_SDK_PERFETTO_SDK_FOR_JNI_TRACING_SDK_H_
