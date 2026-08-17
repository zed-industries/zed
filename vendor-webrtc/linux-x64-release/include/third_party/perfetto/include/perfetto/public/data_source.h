/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_PUBLIC_DATA_SOURCE_H_
#define INCLUDE_PERFETTO_PUBLIC_DATA_SOURCE_H_

#include <stdlib.h>
#include <string.h>

#include "perfetto/public/abi/atomic.h"
#include "perfetto/public/abi/data_source_abi.h"
#include "perfetto/public/abi/heap_buffer.h"
#include "perfetto/public/compiler.h"
#include "perfetto/public/pb_msg.h"
#include "perfetto/public/pb_utils.h"
#include "perfetto/public/protos/common/data_source_descriptor.pzc.h"
#include "perfetto/public/protos/trace/trace_packet.pzc.h"

// A data source type.
struct PerfettoDs {
  // Pointer to a (atomic) boolean, which is set to true if there is at
  // least one enabled instance of this data source type.
  PERFETTO_ATOMIC(bool) * enabled;
  struct PerfettoDsImpl* impl;
};

// Initializes a PerfettoDs struct.
#define PERFETTO_DS_INIT() {&perfetto_atomic_false, PERFETTO_NULL}

// All the callbacks are optional and can be NULL if not needed.
//
struct PerfettoDsParams {
  // Instance lifecycle callbacks.
  //
  // Can be called from any thread.
  PerfettoDsOnSetupCb on_setup_cb;
  PerfettoDsOnStartCb on_start_cb;
  PerfettoDsOnStopCb on_stop_cb;
  PerfettoDsOnDestroyCb on_destroy_cb;
  PerfettoDsOnFlushCb on_flush_cb;

  // These are called to create/delete custom thread-local instance state, which
  // can be accessed with PerfettoDsTracerImplGetCustomTls().
  //
  // Called from inside a trace point. Trace points inside these will be
  // ignored.
  PerfettoDsOnCreateCustomState on_create_tls_cb;
  PerfettoDsOnDeleteCustomState on_delete_tls_cb;

  // These are called to create/delete custom thread-local instance incremental
  // state. Incremental state may be cleared periodically by the tracing service
  // and can be accessed with PerfettoDsTracerImplGetIncrementalState().
  //
  // Called from inside a trace point. Trace points inside these will be
  // ignored.
  PerfettoDsOnCreateCustomState on_create_incr_cb;
  PerfettoDsOnDeleteCustomState on_delete_incr_cb;

  // Passed to all the callbacks as the `user_arg` param.
  void* user_arg;

  // How to behave when running out of shared memory buffer space.
  enum PerfettoDsBufferExhaustedPolicy buffer_exhausted_policy;

  // When true the data source is expected to ack the stop request through the
  // NotifyDataSourceStopped() IPC.
  bool will_notify_on_stop;
};

static inline struct PerfettoDsParams PerfettoDsParamsDefault(void) {
  struct PerfettoDsParams ret = {PERFETTO_NULL,
                                 PERFETTO_NULL,
                                 PERFETTO_NULL,
                                 PERFETTO_NULL,
                                 PERFETTO_NULL,
                                 PERFETTO_NULL,
                                 PERFETTO_NULL,
                                 PERFETTO_NULL,
                                 PERFETTO_NULL,
                                 PERFETTO_NULL,
                                 PERFETTO_DS_BUFFER_EXHAUSTED_POLICY_DROP,
                                 true};
  return ret;
}

// Registers the data source type `ds`, named `data_source_name` with the global
// perfetto producer.
static inline bool PerfettoDsRegister(struct PerfettoDs* ds,
                                      const char* data_source_name,
                                      struct PerfettoDsParams params) {
  struct PerfettoDsImpl* ds_impl;
  bool success;
  void* desc_buf;
  size_t desc_size;

  ds->enabled = &perfetto_atomic_false;
  ds->impl = PERFETTO_NULL;

  {
    struct PerfettoPbMsgWriter writer;
    struct PerfettoHeapBuffer* hb = PerfettoHeapBufferCreate(&writer.writer);
    struct perfetto_protos_DataSourceDescriptor desc;
    PerfettoPbMsgInit(&desc.msg, &writer);

    perfetto_protos_DataSourceDescriptor_set_cstr_name(&desc, data_source_name);
    perfetto_protos_DataSourceDescriptor_set_will_notify_on_stop(
        &desc, params.will_notify_on_stop);

    desc_size = PerfettoStreamWriterGetWrittenSize(&writer.writer);
    desc_buf = malloc(desc_size);
    PerfettoHeapBufferCopyInto(hb, &writer.writer, desc_buf, desc_size);
    PerfettoHeapBufferDestroy(hb, &writer.writer);
  }

  if (!desc_buf) {
    return false;
  }

  ds_impl = PerfettoDsImplCreate();
  if (params.on_setup_cb) {
    PerfettoDsSetOnSetupCallback(ds_impl, params.on_setup_cb);
  }
  if (params.on_start_cb) {
    PerfettoDsSetOnStartCallback(ds_impl, params.on_start_cb);
  }
  if (params.on_stop_cb) {
    PerfettoDsSetOnStopCallback(ds_impl, params.on_stop_cb);
  }
  if (params.on_destroy_cb) {
    PerfettoDsSetOnDestroyCallback(ds_impl, params.on_destroy_cb);
  }
  if (params.on_flush_cb) {
    PerfettoDsSetOnFlushCallback(ds_impl, params.on_flush_cb);
  }
  if (params.on_create_tls_cb) {
    PerfettoDsSetOnCreateTls(ds_impl, params.on_create_tls_cb);
  }
  if (params.on_delete_tls_cb) {
    PerfettoDsSetOnDeleteTls(ds_impl, params.on_delete_tls_cb);
  }
  if (params.on_create_incr_cb) {
    PerfettoDsSetOnCreateIncr(ds_impl, params.on_create_incr_cb);
  }
  if (params.on_delete_incr_cb) {
    PerfettoDsSetOnDeleteIncr(ds_impl, params.on_delete_incr_cb);
  }
  if (params.user_arg) {
    PerfettoDsSetCbUserArg(ds_impl, params.user_arg);
  }
  if (params.buffer_exhausted_policy !=
      PERFETTO_DS_BUFFER_EXHAUSTED_POLICY_DROP) {
    PerfettoDsSetBufferExhaustedPolicy(ds_impl, params.buffer_exhausted_policy);
  }

  success = PerfettoDsImplRegister(ds_impl, &ds->enabled, desc_buf, desc_size);
  free(desc_buf);
  if (!success) {
    return false;
  }
  ds->impl = ds_impl;
  return true;
}

// Iterator for all the active instances (on this thread) of a data source type.
struct PerfettoDsTracerIterator {
  struct PerfettoDsImplTracerIterator impl;
};

static inline struct PerfettoDsTracerIterator PerfettoDsTraceIterateBegin(
    struct PerfettoDs* ds) {
  struct PerfettoDsTracerIterator ret;
  PERFETTO_ATOMIC(bool)* enabled = ds->enabled;
  if (PERFETTO_LIKELY(!PERFETTO_ATOMIC_LOAD_EXPLICIT(
          enabled, PERFETTO_MEMORY_ORDER_RELAXED))) {
    // Tracing fast path: bail out immediately if the enabled flag is false.
    ret.impl.tracer = PERFETTO_NULL;
  } else {
    // Else, make an ABI call to start iteration over the data source type
    // active instances.
    ret.impl = PerfettoDsImplTraceIterateBegin(ds->impl);
  }
  return ret;
}

static inline void PerfettoDsTraceIterateNext(
    struct PerfettoDs* ds,
    struct PerfettoDsTracerIterator* iterator) {
  PerfettoDsImplTraceIterateNext(ds->impl, &iterator->impl);
}

static inline void PerfettoDsTraceIterateBreak(
    struct PerfettoDs* ds,
    struct PerfettoDsTracerIterator* iterator) {
  if (iterator->impl.tracer) {
    PerfettoDsImplTraceIterateBreak(ds->impl, &iterator->impl);
  }
}

// For loop over the active instances of a data source type.
//
// `NAME` is the data source type (struct PerfettoDs).
//
// A local variable called `ITERATOR` will be instantiated. It can be used to
// perform tracing on each instance.
//
// N.B. The iteration MUST NOT be interrupted early with `break`.
// PERFETTO_DS_TRACE_BREAK should be used instead.
#define PERFETTO_DS_TRACE(NAME, ITERATOR)         \
  for (struct PerfettoDsTracerIterator ITERATOR = \
           PerfettoDsTraceIterateBegin(&(NAME));  \
       (ITERATOR).impl.tracer != NULL;            \
       PerfettoDsTraceIterateNext(&(NAME), &(ITERATOR)))

// Used to break the iteration in a PERFETTO_DS_TRACE loop.
#define PERFETTO_DS_TRACE_BREAK(NAME, ITERATOR)      \
  PerfettoDsTraceIterateBreak(&(NAME), &(ITERATOR)); \
  break

static inline void* PerfettoDsGetCustomTls(
    struct PerfettoDs* ds,
    struct PerfettoDsTracerIterator* iterator) {
  return PerfettoDsImplGetCustomTls(ds->impl, iterator->impl.tracer,
                                    iterator->impl.inst_id);
}

static inline void* PerfettoDsGetIncrementalState(
    struct PerfettoDs* ds,
    struct PerfettoDsTracerIterator* iterator) {
  return PerfettoDsImplGetIncrementalState(ds->impl, iterator->impl.tracer,
                                           iterator->impl.inst_id);
}

// Used to write a TracePacket on a data source instance. Stores the writer and
// the TracePacket message.
struct PerfettoDsRootTracePacket {
  struct PerfettoPbMsgWriter writer;
  struct perfetto_protos_TracePacket msg;
};

// Initializes `root` to write a new packet to the data source instance pointed
// by `iterator`.
static inline void PerfettoDsTracerPacketBegin(
    struct PerfettoDsTracerIterator* iterator,
    struct PerfettoDsRootTracePacket* root) {
  root->writer.writer = PerfettoDsTracerImplPacketBegin(iterator->impl.tracer);
  PerfettoPbMsgInit(&root->msg.msg, &root->writer);
}

// Finishes writing the packet pointed by `root` on the data source instance
// pointer by `iterator`.
static inline void PerfettoDsTracerPacketEnd(
    struct PerfettoDsTracerIterator* iterator,
    struct PerfettoDsRootTracePacket* root) {
  PerfettoPbMsgFinalize(&root->msg.msg);
  PerfettoDsTracerImplPacketEnd(iterator->impl.tracer, &root->writer.writer);
}

static inline void PerfettoDsTracerFlush(
    struct PerfettoDsTracerIterator* iterator,
    PerfettoDsTracerOnFlushCb cb,
    void* ctx) {
  PerfettoDsTracerImplFlush(iterator->impl.tracer, cb, ctx);
}

#endif  // INCLUDE_PERFETTO_PUBLIC_DATA_SOURCE_H_
