/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_PUBLIC_TRACK_EVENT_H_
#define INCLUDE_PERFETTO_PUBLIC_TRACK_EVENT_H_

#include <stdint.h>
#include <stdlib.h>

#include "perfetto/public/abi/heap_buffer.h"
#include "perfetto/public/abi/track_event_abi.h"     // IWYU pragma: export
#include "perfetto/public/abi/track_event_hl_abi.h"  // IWYU pragma: export
#include "perfetto/public/abi/track_event_ll_abi.h"  // IWYU pragma: export
#include "perfetto/public/compiler.h"
#include "perfetto/public/data_source.h"
#include "perfetto/public/fnv1a.h"
#include "perfetto/public/pb_msg.h"
#include "perfetto/public/protos/trace/interned_data/interned_data.pzc.h"
#include "perfetto/public/protos/trace/trace_packet.pzc.h"
#include "perfetto/public/protos/trace/track_event/counter_descriptor.pzc.h"
#include "perfetto/public/protos/trace/track_event/track_descriptor.pzc.h"
#include "perfetto/public/protos/trace/track_event/track_event.pzc.h"
#include "perfetto/public/thread_utils.h"

// A registered category.
struct PerfettoTeCategory {
  PERFETTO_ATOMIC(bool) * enabled;
  struct PerfettoTeCategoryImpl* impl;
  struct PerfettoTeCategoryDescriptor desc;
  uint64_t cat_iid;
};

// Registers the category `cat`. `cat->desc` must be filled before calling this.
// The rest of the structure is filled by the function.
static inline void PerfettoTeCategoryRegister(struct PerfettoTeCategory* cat) {
  cat->impl = PerfettoTeCategoryImplCreate(&cat->desc);
  cat->enabled = PerfettoTeCategoryImplGetEnabled(cat->impl);
  cat->cat_iid = PerfettoTeCategoryImplGetIid(cat->impl);
}

// Calls PerfettoTeCategoryRegister() on multiple categories.
static inline void PerfettoTeRegisterCategories(
    struct PerfettoTeCategory* cats[],
    size_t size) {
  for (size_t i = 0; i < size; i++) {
    PerfettoTeCategoryRegister(cats[i]);
  }
}

// Registers `cb` to be called every time a data source instance with `reg_cat`
// enabled is created or destroyed. `user_arg` will be passed unaltered to `cb`.
//
// `cb` can be NULL to disable the callback.
static inline void PerfettoTeCategorySetCallback(
    struct PerfettoTeCategory* reg_cat,
    PerfettoTeCategoryImplCallback cb,
    void* user_arg) {
  PerfettoTeCategoryImplSetCallback(reg_cat->impl, cb, user_arg);
}

// Unregisters the category `cat`.
//
// WARNING: The category cannot be used for tracing anymore after this.
// Executing PERFETTO_TE() on an unregistered category will cause a null pointer
// dereference.
static inline void PerfettoTeCategoryUnregister(
    struct PerfettoTeCategory* cat) {
  PerfettoTeCategoryImplDestroy(cat->impl);
  cat->impl = PERFETTO_NULL;
  cat->enabled = &perfetto_atomic_false;
  cat->cat_iid = 0;
}

// Calls PerfettoTeCategoryUnregister() on multiple categories.
//
// WARNING: The categories cannot be used for tracing anymore after this.
// Executing PERFETTO_TE() on unregistered categories will cause a null pointer
// dereference.
static inline void PerfettoTeUnregisterCategories(
    struct PerfettoTeCategory* cats[],
    size_t size) {
  for (size_t i = 0; i < size; i++) {
    PerfettoTeCategoryUnregister(cats[i]);
  }
}

// A track. Must be registered before it can be used in trace events.
struct PerfettoTeRegisteredTrack {
  struct PerfettoTeRegisteredTrackImpl impl;
};

// Returns the track uuid for the current process.
static inline uint64_t PerfettoTeProcessTrackUuid(void) {
  return perfetto_te_process_track_uuid;
}

// Returns the track uuid for the current thread.
static inline uint64_t PerfettoTeThreadTrackUuid(void) {
  return perfetto_te_process_track_uuid ^
         PERFETTO_STATIC_CAST(uint64_t, PerfettoGetThreadId());
}

// Returns the root track uuid.
static inline uint64_t PerfettoTeGlobalTrackUuid(void) {
  return 0;
}

// Computes the track uuid for a counter track named `name` whose parent track
// has `parent_uuid`.
static inline uint64_t PerfettoTeCounterTrackUuid(const char* name,
                                                  uint64_t parent_uuid) {
  const uint64_t kCounterMagic = 0xb1a4a67d7970839eul;
  uint64_t uuid = kCounterMagic;
  uuid ^= parent_uuid;
  uuid ^= PerfettoFnv1a(name, strlen(name));
  return uuid;
}

// Computes the track uuid for a track named `name` with unique `id` whose
// parent track has `parent_uuid`.
static inline uint64_t PerfettoTeNamedTrackUuid(const char* name,
                                                uint64_t id,
                                                uint64_t parent_uuid) {
  uint64_t uuid = parent_uuid;
  uuid ^= PerfettoFnv1a(name, strlen(name));
  uuid ^= id;
  return uuid;
}

// Serializes the descriptor for a counter track named `name` with
// `parent_uuid`. `track_uuid` must be the return value of
// PerfettoTeCounterTrackUuid().
static inline void PerfettoTeCounterTrackFillDesc(
    struct perfetto_protos_TrackDescriptor* desc,
    const char* name,
    uint64_t parent_track_uuid,
    uint64_t track_uuid) {
  perfetto_protos_TrackDescriptor_set_uuid(desc, track_uuid);
  if (parent_track_uuid) {
    perfetto_protos_TrackDescriptor_set_parent_uuid(desc, parent_track_uuid);
  }
  perfetto_protos_TrackDescriptor_set_cstr_name(desc, name);
  {
    struct perfetto_protos_CounterDescriptor counter;
    perfetto_protos_TrackDescriptor_begin_counter(desc, &counter);
    perfetto_protos_TrackDescriptor_end_counter(desc, &counter);
  }
}

// Serializes the descriptor for a track named `name` with unique `id` and
// `parent_uuid`. `track_uuid` must be the return value of
// PerfettoTeNamedTrackUuid().
static inline void PerfettoTeNamedTrackFillDesc(
    struct perfetto_protos_TrackDescriptor* desc,
    const char* track_name,
    uint64_t id,
    uint64_t parent_track_uuid,
    uint64_t track_uuid) {
  (void)id;
  perfetto_protos_TrackDescriptor_set_uuid(desc, track_uuid);
  if (parent_track_uuid) {
    perfetto_protos_TrackDescriptor_set_parent_uuid(desc, parent_track_uuid);
  }
  perfetto_protos_TrackDescriptor_set_cstr_name(desc, track_name);
}

// Registers a track named `name` with unique `id` and `parent_uuid` into
// `track`.
static inline void PerfettoTeNamedTrackRegister(
    struct PerfettoTeRegisteredTrack* track,
    const char* name,
    uint64_t id,
    uint64_t parent_track_uuid) {
  uint64_t uuid;
  // Build the TrackDescriptor protobuf message.
  struct PerfettoPbMsgWriter writer;
  struct PerfettoHeapBuffer* hb = PerfettoHeapBufferCreate(&writer.writer);
  struct perfetto_protos_TrackDescriptor desc;
  PerfettoPbMsgInit(&desc.msg, &writer);

  uuid = PerfettoTeNamedTrackUuid(name, id, parent_track_uuid);

  PerfettoTeNamedTrackFillDesc(&desc, name, id, parent_track_uuid, uuid);

  track->impl.descriptor_size =
      PerfettoStreamWriterGetWrittenSize(&writer.writer);
  track->impl.descriptor = malloc(track->impl.descriptor_size);
  track->impl.uuid = uuid;
  PerfettoHeapBufferCopyInto(hb, &writer.writer, track->impl.descriptor,
                             track->impl.descriptor_size);
  PerfettoHeapBufferDestroy(hb, &writer.writer);
}

// Registers a counter track named `name` with and `parent_uuid` into `track`.
static inline void PerfettoTeCounterTrackRegister(
    struct PerfettoTeRegisteredTrack* track,
    const char* name,
    uint64_t parent_track_uuid) {
  uint64_t uuid;
  struct PerfettoPbMsgWriter writer;
  struct PerfettoHeapBuffer* hb = PerfettoHeapBufferCreate(&writer.writer);
  struct perfetto_protos_TrackDescriptor desc;
  PerfettoPbMsgInit(&desc.msg, &writer);

  uuid = PerfettoTeCounterTrackUuid(name, parent_track_uuid);

  PerfettoTeCounterTrackFillDesc(&desc, name, parent_track_uuid, uuid);

  track->impl.descriptor_size =
      PerfettoStreamWriterGetWrittenSize(&writer.writer);
  track->impl.descriptor = malloc(track->impl.descriptor_size);
  track->impl.uuid = uuid;
  PerfettoHeapBufferCopyInto(hb, &writer.writer, track->impl.descriptor,
                             track->impl.descriptor_size);
  PerfettoHeapBufferDestroy(hb, &writer.writer);
}

// Unregisters the previously registered track `track`.
static inline void PerfettoTeRegisteredTrackUnregister(
    struct PerfettoTeRegisteredTrack* track) {
  free(track->impl.descriptor);
  track->impl.descriptor = PERFETTO_NULL;
  track->impl.descriptor_size = 0;
}

// Identifies a flow: a link between two events.
struct PerfettoTeFlow {
  uint64_t id;
};

// Returns a flow that's scoped to this process. It can be used to link events
// inside this process.
static inline struct PerfettoTeFlow PerfettoTeProcessScopedFlow(uint64_t id) {
  struct PerfettoTeFlow ret;
  ret.id = id ^ perfetto_te_process_track_uuid;
  return ret;
}

// Returns a global flow. It can be used to link events between different
// processes.
static inline struct PerfettoTeFlow PerfettoTeGlobalFlow(uint64_t id) {
  struct PerfettoTeFlow ret;
  ret.id = id;
  return ret;
}

// Returns a static-category-like object used when dynamic categories are passed
// as extra parameters.
static inline struct PerfettoTeCategory PerfettoTeRegisteredDynamicCategory(
    void) {
  struct PerfettoTeCategory ret = {
      perfetto_te_any_categories_enabled,
      perfetto_te_any_categories,
      {PERFETTO_NULL, PERFETTO_NULL, PERFETTO_NULL, 0},
      0};
  return ret;
}

// Iterator for all the active instances (on this thread) of a data source type.
struct PerfettoTeLlIterator {
  struct PerfettoTeLlImplIterator impl;
};

static inline struct PerfettoTeLlIterator PerfettoTeLlBeginSlowPath(
    struct PerfettoTeCategory* cat,
    struct PerfettoTeTimestamp ts) {
  struct PerfettoTeLlIterator ret;
  ret.impl = PerfettoTeLlImplBegin(cat->impl, ts);
  return ret;
}

static inline void PerfettoTeLlNext(struct PerfettoTeCategory* cat,
                                    struct PerfettoTeTimestamp ts,
                                    struct PerfettoTeLlIterator* iterator) {
  PerfettoTeLlImplNext(cat->impl, ts, &iterator->impl);
}

static inline void PerfettoTeLlBreak(struct PerfettoTeCategory* cat,
                                     struct PerfettoTeLlIterator* iterator) {
  if (iterator->impl.ds.tracer) {
    PerfettoTeLlImplBreak(cat->impl, &iterator->impl);
  }
}

// Checks if the category descriptor `dyn_cat` is enabled in the current active
// instance pointed by `iterator`.
static inline bool PerfettoTeLlDynCatEnabled(
    struct PerfettoTeLlIterator* iterator,
    const struct PerfettoTeCategoryDescriptor* dyn_cat) {
  return PerfettoTeLlImplDynCatEnabled(iterator->impl.ds.tracer,
                                       iterator->impl.ds.inst_id, dyn_cat);
}

// Initializes `root` to write a new packet to the data source instance pointed
// by `iterator`.
static inline void PerfettoTeLlPacketBegin(
    struct PerfettoTeLlIterator* iterator,
    struct PerfettoDsRootTracePacket* root) {
  root->writer.writer =
      PerfettoDsTracerImplPacketBegin(iterator->impl.ds.tracer);
  PerfettoPbMsgInit(&root->msg.msg, &root->writer);
}

// Finishes writing the packet pointed by `root` on the data source instance
// pointer by `iterator`.
static inline void PerfettoTeLlPacketEnd(
    struct PerfettoTeLlIterator* iterator,
    struct PerfettoDsRootTracePacket* root) {
  PerfettoPbMsgFinalize(&root->msg.msg);
  PerfettoDsTracerImplPacketEnd(iterator->impl.ds.tracer, &root->writer.writer);
}

static inline void PerfettoTeLlFlushPacket(
    struct PerfettoTeLlIterator* iterator) {
  PerfettoDsTracerImplFlush(iterator->impl.ds.tracer, PERFETTO_NULL,
                            PERFETTO_NULL);
}

// Returns true if the track event incremental state has already seen in the
// past a track with `uuid` as track UUID.
static inline bool PerfettoTeLlTrackSeen(struct PerfettoTeLlImplIncr* incr,
                                         uint64_t uuid) {
  return PerfettoTeLlImplTrackSeen(incr, uuid);
}

// Interning:
//
// it's possible to avoid repeating the same data over and over in a trace by
// using "interning".
//
// `type` is a field id in the `perfetto.protos.InternedData` protobuf message.
// `data` and `data_size` point to the raw data that is potentially repeated.
// The function returns an integer (the iid) that can be used instead of
// serializing the data directly in the packet. `*seen` is set to false if this
// is the first time the library observed this data for this specific type
// (therefore it allocated a new iid).
static inline uint64_t PerfettoTeLlIntern(struct PerfettoTeLlImplIncr* incr,
                                          int32_t type,
                                          const void* data,
                                          size_t data_size,
                                          bool* seen) {
  return PerfettoTeLlImplIntern(incr, type, data, data_size, seen);
}

// Used to lazily start, only if required, a nested InternedData submessage for
// a TracePacket `tp`. `incr` is the incremental state ABI pointer received from
// PerfettoTeLlIterator.
struct PerfettoTeLlInternContext {
  struct PerfettoTeLlImplIncr* incr;
  struct perfetto_protos_TracePacket* tp;
  struct perfetto_protos_InternedData interned;
  // true if the nested `interned` submessage has been started, false otherwise.
  bool started;
};

static inline void PerfettoTeLlInternContextInit(
    struct PerfettoTeLlInternContext* ctx,
    struct PerfettoTeLlImplIncr* incr,
    struct perfetto_protos_TracePacket* tp) {
  ctx->incr = incr;
  ctx->tp = tp;
  ctx->started = false;
}

static inline void PerfettoTeLlInternContextStartIfNeeded(
    struct PerfettoTeLlInternContext* ctx) {
  if (!ctx->started) {
    ctx->started = true;
    perfetto_protos_TracePacket_begin_interned_data(ctx->tp, &ctx->interned);
  }
}

static inline void PerfettoTeLlInternContextDestroy(
    struct PerfettoTeLlInternContext* ctx) {
  if (ctx->started) {
    perfetto_protos_TracePacket_end_interned_data(ctx->tp, &ctx->interned);
  }
}

static inline void PerfettoTeLlInternRegisteredCat(
    struct PerfettoTeLlInternContext* ctx,
    struct PerfettoTeCategory* reg_cat) {
  uint64_t iid = reg_cat->cat_iid;
  bool seen;

  if (!iid) {
    return;
  }
  PerfettoTeLlIntern(ctx->incr,
                     perfetto_protos_InternedData_event_categories_field_number,
                     &iid, sizeof(iid), &seen);
  if (!seen) {
    struct perfetto_protos_EventCategory event_category;
    PerfettoTeLlInternContextStartIfNeeded(ctx);

    perfetto_protos_InternedData_begin_event_categories(&ctx->interned,
                                                        &event_category);
    perfetto_protos_EventCategory_set_iid(&event_category, iid);
    perfetto_protos_EventCategory_set_cstr_name(&event_category,
                                                reg_cat->desc.name);
    perfetto_protos_InternedData_end_event_categories(&ctx->interned,
                                                      &event_category);
  }
}

static inline void PerfettoTeLlWriteRegisteredCat(
    struct perfetto_protos_TrackEvent* te,
    struct PerfettoTeCategory* reg_cat) {
  if (reg_cat->cat_iid) {
    perfetto_protos_TrackEvent_set_category_iids(te, reg_cat->cat_iid);
  } else if (reg_cat->desc.name) {
    perfetto_protos_TrackEvent_set_cstr_categories(te, reg_cat->desc.name);
  }
}

static inline void PerfettoTeLlWriteDynamicCat(
    struct perfetto_protos_TrackEvent* te,
    struct PerfettoTeCategoryDescriptor* dyn_cat,
    int32_t type) {
  if (dyn_cat && type != PERFETTO_TE_TYPE_SLICE_END &&
      type != PERFETTO_TE_TYPE_COUNTER) {
    perfetto_protos_TrackEvent_set_cstr_categories(te, dyn_cat->name);
  }
}

static inline uint64_t PerfettoTeLlInternEventName(
    struct PerfettoTeLlInternContext* ctx,
    const char* name) {
  uint64_t iid = 0;
  if (name) {
    bool seen;
    iid = PerfettoTeLlIntern(
        ctx->incr, perfetto_protos_InternedData_event_names_field_number, name,
        strlen(name), &seen);
    if (!seen) {
      struct perfetto_protos_EventName event_name;
      PerfettoTeLlInternContextStartIfNeeded(ctx);
      perfetto_protos_InternedData_begin_event_names(&ctx->interned,
                                                     &event_name);
      perfetto_protos_EventName_set_iid(&event_name, iid);
      perfetto_protos_EventName_set_cstr_name(&event_name, name);
      perfetto_protos_InternedData_end_event_names(&ctx->interned, &event_name);
    }
  }
  return iid;
}

static inline void PerfettoTeLlWriteEventName(
    struct perfetto_protos_TrackEvent* te,
    const char* name) {
  if (name) {
    perfetto_protos_TrackEvent_set_cstr_name(te, name);
  }
}

static inline void PerfettoTeLlWriteInternedEventName(
    struct perfetto_protos_TrackEvent* te,
    uint64_t iid) {
  if (iid != 0) {
    perfetto_protos_TrackEvent_set_name_iid(te, iid);
  }
}

static inline void PerfettoTeLlWriteTimestamp(
    struct perfetto_protos_TracePacket* tp,
    const struct PerfettoTeTimestamp* ts) {
  uint32_t clock_id = ts->clock_id;
  perfetto_protos_TracePacket_set_timestamp(tp, ts->value);
  perfetto_protos_TracePacket_set_timestamp_clock_id(tp, clock_id);
}

static inline uint64_t PerfettoTeLlInternDbgArgName(
    struct PerfettoTeLlInternContext* ctx,
    const char* name) {
  uint64_t iid = 0;
  if (name) {
    bool seen;
    iid = PerfettoTeLlIntern(
        ctx->incr,
        perfetto_protos_InternedData_debug_annotation_names_field_number, name,
        strlen(name), &seen);
    if (!seen) {
      struct perfetto_protos_EventName event_name;
      PerfettoTeLlInternContextStartIfNeeded(ctx);
      perfetto_protos_InternedData_begin_event_names(&ctx->interned,
                                                     &event_name);
      perfetto_protos_EventName_set_iid(&event_name, iid);
      perfetto_protos_EventName_set_cstr_name(&event_name, name);
      perfetto_protos_InternedData_end_event_names(&ctx->interned, &event_name);
    }
  }
  return iid;
}

#endif  // INCLUDE_PERFETTO_PUBLIC_TRACK_EVENT_H_
