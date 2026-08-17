#ifndef __TRACYOPENCL_HPP__
#define __TRACYOPENCL_HPP__

#if !defined TRACY_ENABLE

#define TracyCLContext(c, x) nullptr
#define TracyCLDestroy(c)
#define TracyCLContextName(c, x, y)

#define TracyCLNamedZone(c, x, y, z)
#define TracyCLNamedZoneC(c, x, y, z, w)
#define TracyCLZone(c, x)
#define TracyCLZoneC(c, x, y)
#define TracyCLZoneTransient(c,x,y,z)

#define TracyCLNamedZoneS(c, x, y, z, w)
#define TracyCLNamedZoneCS(c, x, y, z, w, v)
#define TracyCLZoneS(c, x, y)
#define TracyCLZoneCS(c, x, y, z)
#define TracyCLZoneTransientS(c,x,y,z,w)

#define TracyCLNamedZoneSetEvent(x, e)
#define TracyCLZoneSetEvent(e)

#define TracyCLCollect(c)

namespace tracy
{
    class OpenCLCtxScope {};
}

using TracyCLCtx = void*;

#else

#include <CL/cl.h>

#include <atomic>
#include <cassert>
#include <sstream>

#include "Tracy.hpp"
#include "../client/TracyCallstack.hpp"
#include "../client/TracyProfiler.hpp"
#include "../common/TracyAlloc.hpp"

#define TRACY_CL_TO_STRING_INDIRECT(T) #T
#define TRACY_CL_TO_STRING(T) TRACY_CL_TO_STRING_INDIRECT(T)
#define TRACY_CL_ASSERT(p) if(!(p)) {                                                         \
    TracyMessageL( "TRACY_CL_ASSERT failed on " TracyFile ":" TRACY_CL_TO_STRING(TracyLine) );  \
    assert(false && "TRACY_CL_ASSERT failed");                                                \
}
#define TRACY_CL_CHECK_ERROR(err) if(err != CL_SUCCESS) {                    \
    std::ostringstream oss;                                                  \
    oss << "TRACY_CL_CHECK_ERROR failed on " << TracyFile << ":" << TracyLine  \
        << ": error code " << err;                                           \
    auto msg = oss.str();                                                    \
    TracyMessage(msg.data(), msg.size());                                    \
    assert(false && "TRACY_CL_CHECK_ERROR failed");                          \
}

namespace tracy {

    enum class EventPhase : uint8_t
    {
        Begin,
        End
    };

    struct EventInfo
    {
        cl_event event;
        EventPhase phase;
    };

    class OpenCLCtx
    {
    public:
        enum { QueryCount = 64 * 1024 };

        OpenCLCtx(cl_context context, cl_device_id device)
            : m_contextId(GetGpuCtxCounter().fetch_add(1, std::memory_order_relaxed))
            , m_head(0)
            , m_tail(0)
        {
            int64_t tcpu, tgpu;
            TRACY_CL_ASSERT(m_contextId != 255);

            cl_int err = CL_SUCCESS;
            cl_command_queue queue = clCreateCommandQueue(context, device, CL_QUEUE_PROFILING_ENABLE, &err);
            TRACY_CL_CHECK_ERROR(err)
            uint32_t dummyValue = 42;
            cl_mem dummyBuffer = clCreateBuffer(context, CL_MEM_WRITE_ONLY, sizeof(uint32_t), nullptr, &err);
            TRACY_CL_CHECK_ERROR(err)
            cl_event writeBufferEvent;
            TRACY_CL_CHECK_ERROR(clEnqueueWriteBuffer(queue, dummyBuffer, CL_FALSE, 0, sizeof(uint32_t), &dummyValue, 0, nullptr, &writeBufferEvent));
            TRACY_CL_CHECK_ERROR(clWaitForEvents(1, &writeBufferEvent));

            tcpu = Profiler::GetTime();

            cl_int eventStatus;
            TRACY_CL_CHECK_ERROR(clGetEventInfo(writeBufferEvent, CL_EVENT_COMMAND_EXECUTION_STATUS, sizeof(cl_int), &eventStatus, nullptr));
            TRACY_CL_ASSERT(eventStatus == CL_COMPLETE);
            TRACY_CL_CHECK_ERROR(clGetEventProfilingInfo(writeBufferEvent, CL_PROFILING_COMMAND_END, sizeof(cl_ulong), &tgpu, nullptr));
            TRACY_CL_CHECK_ERROR(clReleaseEvent(writeBufferEvent));
            TRACY_CL_CHECK_ERROR(clReleaseMemObject(dummyBuffer));
            TRACY_CL_CHECK_ERROR(clReleaseCommandQueue(queue));

            auto item = Profiler::QueueSerial();
            MemWrite(&item->hdr.type, QueueType::GpuNewContext);
            MemWrite(&item->gpuNewContext.cpuTime, tcpu);
            MemWrite(&item->gpuNewContext.gpuTime, tgpu);
            memset(&item->gpuNewContext.thread, 0, sizeof(item->gpuNewContext.thread));
            MemWrite(&item->gpuNewContext.period, 1.0f);
            MemWrite(&item->gpuNewContext.type, GpuContextType::OpenCL);
            MemWrite(&item->gpuNewContext.context, (uint8_t) m_contextId);
            MemWrite(&item->gpuNewContext.flags, (uint8_t)0);
#ifdef TRACY_ON_DEMAND
            GetProfiler().DeferItem(*item);
#endif
            Profiler::QueueSerialFinish();
        }

        void Name( const char* name, uint16_t len )
        {
            auto ptr = (char*)tracy_malloc( len );
            memcpy( ptr, name, len );

            auto item = Profiler::QueueSerial();
            MemWrite( &item->hdr.type, QueueType::GpuContextName );
            MemWrite( &item->gpuContextNameFat.context, (uint8_t)m_contextId );
            MemWrite( &item->gpuContextNameFat.ptr, (uint64_t)ptr );
            MemWrite( &item->gpuContextNameFat.size, len );
#ifdef TRACY_ON_DEMAND
            GetProfiler().DeferItem( *item );
#endif
            Profiler::QueueSerialFinish();
        }

        void Collect()
        {
            ZoneScopedC(Color::Red4);

            if (m_tail == m_head) return;

#ifdef TRACY_ON_DEMAND
            if (!GetProfiler().IsConnected())
            {
                m_head = m_tail = 0;
            }
#endif

            for (; m_tail != m_head; m_tail = (m_tail + 1) % QueryCount)
            {
                EventInfo eventInfo = GetQuery(m_tail);
                cl_int eventStatus;
                cl_int err = clGetEventInfo(eventInfo.event, CL_EVENT_COMMAND_EXECUTION_STATUS, sizeof(cl_int), &eventStatus, nullptr);
                if (err != CL_SUCCESS)
                {
                    std::ostringstream oss;
                    oss << "clGetEventInfo falied with error code " << err << ", on event " << eventInfo.event << ", skipping...";
                    auto msg = oss.str();
                    TracyMessage(msg.data(), msg.size());
                    if (eventInfo.event == nullptr) {
                        TracyMessageL("A TracyCLZone must be paird with a TracyCLZoneSetEvent, check your code!");
                    }
                    assert(false && "clGetEventInfo failed, maybe a TracyCLZone is not paired with TracyCLZoneSetEvent");
                    continue;
                }
                if (eventStatus != CL_COMPLETE) return;

                cl_int eventInfoQuery = (eventInfo.phase == EventPhase::Begin)
                    ? CL_PROFILING_COMMAND_START
                    : CL_PROFILING_COMMAND_END;

                cl_ulong eventTimeStamp = 0;
                err = clGetEventProfilingInfo(eventInfo.event, eventInfoQuery, sizeof(cl_ulong), &eventTimeStamp, nullptr);
                if (err == CL_PROFILING_INFO_NOT_AVAILABLE)
                {
                    TracyMessageL("command queue is not created with CL_QUEUE_PROFILING_ENABLE flag, check your code!");
                    assert(false && "command queue is not created with CL_QUEUE_PROFILING_ENABLE flag");
                }
                else
                    TRACY_CL_CHECK_ERROR(err);

                TRACY_CL_ASSERT(eventTimeStamp != 0);

                auto item = Profiler::QueueSerial();
                MemWrite(&item->hdr.type, QueueType::GpuTime);
                MemWrite(&item->gpuTime.gpuTime, (int64_t)eventTimeStamp);
                MemWrite(&item->gpuTime.queryId, (uint16_t)m_tail);
                MemWrite(&item->gpuTime.context, m_contextId);
                Profiler::QueueSerialFinish();

                if (eventInfo.phase == EventPhase::End)
                {
                    // Done with the event, so release it
                    TRACY_CL_CHECK_ERROR(clReleaseEvent(eventInfo.event));
                }
            }
        }

        tracy_force_inline uint8_t GetId() const
        {
            return m_contextId;
        }

        tracy_force_inline unsigned int NextQueryId(EventInfo eventInfo)
        {
            const auto id = m_head;
            m_head = (m_head + 1) % QueryCount;
            TRACY_CL_ASSERT(m_head != m_tail);
            m_query[id] = eventInfo;
            return id;
        }

        tracy_force_inline EventInfo& GetQuery(unsigned int id)
        {
            TRACY_CL_ASSERT(id < QueryCount);
            return m_query[id];
        }

    private:

        unsigned int m_contextId;

        EventInfo m_query[QueryCount];
        unsigned int m_head; // index at which a new event should be inserted
        unsigned int m_tail; // oldest event

    };

    class OpenCLCtxScope {
    public:
        tracy_force_inline OpenCLCtxScope(OpenCLCtx* ctx, const SourceLocationData* srcLoc, bool is_active)
#ifdef TRACY_ON_DEMAND
            : m_active(is_active&& GetProfiler().IsConnected())
#else
            : m_active(is_active)
#endif
            , m_ctx(ctx)
            , m_event(nullptr)
        {
            if (!m_active) return;

            m_beginQueryId = ctx->NextQueryId(EventInfo{ nullptr, EventPhase::Begin });

            auto item = Profiler::QueueSerial();
            MemWrite(&item->hdr.type, QueueType::GpuZoneBeginSerial);
            MemWrite(&item->gpuZoneBegin.cpuTime, Profiler::GetTime());
            MemWrite(&item->gpuZoneBegin.srcloc, (uint64_t)srcLoc);
            MemWrite(&item->gpuZoneBegin.thread, GetThreadHandle());
            MemWrite(&item->gpuZoneBegin.queryId, (uint16_t)m_beginQueryId);
            MemWrite(&item->gpuZoneBegin.context, ctx->GetId());
            Profiler::QueueSerialFinish();
        }

        tracy_force_inline OpenCLCtxScope(OpenCLCtx* ctx, const SourceLocationData* srcLoc, int32_t depth, bool is_active)
#ifdef TRACY_ON_DEMAND
            : m_active(is_active&& GetProfiler().IsConnected())
#else
            : m_active(is_active)
#endif
            , m_ctx(ctx)
            , m_event(nullptr)
        {
            if (!m_active) return;

            m_beginQueryId = ctx->NextQueryId(EventInfo{ nullptr, EventPhase::Begin });

            GetProfiler().SendCallstack(depth);

            auto item = Profiler::QueueSerial();
            MemWrite(&item->hdr.type, QueueType::GpuZoneBeginCallstackSerial);
            MemWrite(&item->gpuZoneBegin.cpuTime, Profiler::GetTime());
            MemWrite(&item->gpuZoneBegin.srcloc, (uint64_t)srcLoc);
            MemWrite(&item->gpuZoneBegin.thread, GetThreadHandle());
            MemWrite(&item->gpuZoneBegin.queryId, (uint16_t)m_beginQueryId);
            MemWrite(&item->gpuZoneBegin.context, ctx->GetId());
            Profiler::QueueSerialFinish();
        }

        tracy_force_inline OpenCLCtxScope(OpenCLCtx* ctx, uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, bool is_active)
#ifdef TRACY_ON_DEMAND
            : m_active(is_active && GetProfiler().IsConnected())
#else
            : m_active(is_active)
#endif
            , m_ctx(ctx)
            , m_event(nullptr)
        {
            if (!m_active) return;

            m_beginQueryId = ctx->NextQueryId(EventInfo{ nullptr, EventPhase::Begin });

            const auto srcloc = Profiler::AllocSourceLocation( line, source, sourceSz, function, functionSz, name, nameSz );
            auto item = Profiler::QueueSerial();
            MemWrite( &item->hdr.type, QueueType::GpuZoneBeginAllocSrcLocSerial );
            MemWrite(&item->gpuZoneBegin.cpuTime, Profiler::GetTime());
            MemWrite(&item->gpuZoneBegin.srcloc, srcloc);
            MemWrite(&item->gpuZoneBegin.thread, GetThreadHandle());
            MemWrite(&item->gpuZoneBegin.queryId, (uint16_t)m_beginQueryId);
            MemWrite(&item->gpuZoneBegin.context, ctx->GetId());
            Profiler::QueueSerialFinish();
        }

        tracy_force_inline OpenCLCtxScope(OpenCLCtx* ctx, uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, int32_t depth, bool is_active)
#ifdef TRACY_ON_DEMAND
            : m_active(is_active && GetProfiler().IsConnected())
#else
            : m_active(is_active)
#endif
            , m_ctx(ctx)
            , m_event(nullptr)
        {
            if (!m_active) return;

            m_beginQueryId = ctx->NextQueryId(EventInfo{ nullptr, EventPhase::Begin });

            const auto srcloc = Profiler::AllocSourceLocation( line, source, sourceSz, function, functionSz, name, nameSz );
            auto item = Profiler::QueueSerialCallstack( Callstack( depth ) );
            MemWrite(&item->hdr.type, QueueType::GpuZoneBeginAllocSrcLocCallstackSerial);
            MemWrite(&item->gpuZoneBegin.cpuTime, Profiler::GetTime());
            MemWrite(&item->gpuZoneBegin.srcloc, srcloc);
            MemWrite(&item->gpuZoneBegin.thread, GetThreadHandle());
            MemWrite(&item->gpuZoneBegin.queryId, (uint16_t)m_beginQueryId);
            MemWrite(&item->gpuZoneBegin.context, ctx->GetId());
            Profiler::QueueSerialFinish();
        }

        tracy_force_inline void SetEvent(cl_event event)
        {
            if (!m_active) return;
            m_event = event;
            TRACY_CL_CHECK_ERROR(clRetainEvent(m_event));
            m_ctx->GetQuery(m_beginQueryId).event = m_event;
        }

        tracy_force_inline ~OpenCLCtxScope()
        {
            if (!m_active) return;
            const auto queryId = m_ctx->NextQueryId(EventInfo{ m_event, EventPhase::End });

            auto item = Profiler::QueueSerial();
            MemWrite(&item->hdr.type, QueueType::GpuZoneEndSerial);
            MemWrite(&item->gpuZoneEnd.cpuTime, Profiler::GetTime());
            MemWrite(&item->gpuZoneEnd.thread, GetThreadHandle());
            MemWrite(&item->gpuZoneEnd.queryId, (uint16_t)queryId);
            MemWrite(&item->gpuZoneEnd.context, m_ctx->GetId());
            Profiler::QueueSerialFinish();
        }

        const bool m_active;
        OpenCLCtx* m_ctx;
        cl_event m_event;
        unsigned int m_beginQueryId;
    };

    static inline OpenCLCtx* CreateCLContext(cl_context context, cl_device_id device)
    {
        auto ctx = (OpenCLCtx*)tracy_malloc(sizeof(OpenCLCtx));
        new (ctx) OpenCLCtx(context, device);
        return ctx;
    }

    static inline void DestroyCLContext(OpenCLCtx* ctx)
    {
        ctx->~OpenCLCtx();
        tracy_free(ctx);
    }

}  // namespace tracy

using TracyCLCtx = tracy::OpenCLCtx*;

#define TracyCLContext(ctx, device) tracy::CreateCLContext(ctx, device);
#define TracyCLDestroy(ctx) tracy::DestroyCLContext(ctx);
#define TracyCLContextName(ctx, name, size) ctx->Name(name, size);
#if defined TRACY_HAS_CALLSTACK && defined TRACY_CALLSTACK
#  define TracyCLNamedZone(ctx, varname, name, active) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction, TracyFile, (uint32_t)TracyLine, 0 }; tracy::OpenCLCtxScope varname(ctx, &TracyConcat(__tracy_gpu_source_location,TracyLine), TRACY_CALLSTACK, active );
#  define TracyCLNamedZoneC(ctx, varname, name, color, active) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction, TracyFile, (uint32_t)TracyLine, color }; tracy::OpenCLCtxScope varname(ctx, &TracyConcat(__tracy_gpu_source_location,TracyLine), TRACY_CALLSTACK, active );
#  define TracyCLZone(ctx, name) TracyCLNamedZoneS(ctx, __tracy_gpu_zone, name, TRACY_CALLSTACK, true)
#  define TracyCLZoneC(ctx, name, color) TracyCLNamedZoneCS(ctx, __tracy_gpu_zone, name, color, TRACY_CALLSTACK, true)
#  define TracyCLZoneTransient( ctx, varname, name, active ) tracy::OpenCLCtxScope varname( ctx, TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), name, strlen( name ), TRACY_CALLSTACK, active );
#else
#  define TracyCLNamedZone(ctx, varname, name, active) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine){ name, TracyFunction, TracyFile, (uint32_t)TracyLine, 0 }; tracy::OpenCLCtxScope varname(ctx, &TracyConcat(__tracy_gpu_source_location,TracyLine), active);
#  define TracyCLNamedZoneC(ctx, varname, name, color, active) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine){ name, TracyFunction, TracyFile, (uint32_t)TracyLine, color }; tracy::OpenCLCtxScope varname(ctx, &TracyConcat(__tracy_gpu_source_location,TracyLine), active);
#  define TracyCLZone(ctx, name) TracyCLNamedZone(ctx, __tracy_gpu_zone, name, true)
#  define TracyCLZoneC(ctx, name, color) TracyCLNamedZoneC(ctx, __tracy_gpu_zone, name, color, true )
#  define TracyCLZoneTransient( ctx, varname, name, active ) tracy::OpenCLCtxScope varname( ctx, TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), name, strlen( name ), active );
#endif

#ifdef TRACY_HAS_CALLSTACK
#  define TracyCLNamedZoneS(ctx, varname, name, depth, active) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine){ name, TracyFunction, TracyFile, (uint32_t)TracyLine, 0 }; tracy::OpenCLCtxScope varname(ctx, &TracyConcat(__tracy_gpu_source_location,TracyLine), depth, active);
#  define TracyCLNamedZoneCS(ctx, varname, name, color, depth, active) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine){ name, TracyFunction, TracyFile, (uint32_t)TracyLine, color }; tracy::OpenCLCtxScope varname(ctx, &TracyConcat(__tracy_gpu_source_location,TracyLine), depth, active);
#  define TracyCLZoneS(ctx, name, depth) TracyCLNamedZoneS(ctx, __tracy_gpu_zone, name, depth, true)
#  define TracyCLZoneCS(ctx, name, color, depth) TracyCLNamedZoneCS(ctx, __tracy_gpu_zone, name, color, depth, true)
#  define TracyCLZoneTransientS( ctx, varname, name, depth, active ) tracy::OpenCLCtxScope varname( ctx, TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), name, strlen( name ), depth, active );
#else
#  define TracyCLNamedZoneS(ctx, varname, name, depth, active) TracyCLNamedZone(ctx, varname, name, active)
#  define TracyCLNamedZoneCS(ctx, varname, name, color, depth, active) TracyCLNamedZoneC(ctx, varname, name, color, active)
#  define TracyCLZoneS(ctx, name, depth) TracyCLZone(ctx, name)
#  define TracyCLZoneCS(ctx, name, color, depth) TracyCLZoneC(ctx, name, color)
#  define TracyCLZoneTransientS( ctx, varname, name, depth, active ) TracyCLZoneTransient( ctx, varname, name, active )
#endif

#define TracyCLNamedZoneSetEvent(varname, event) varname.SetEvent(event)
#define TracyCLZoneSetEvent(event) __tracy_gpu_zone.SetEvent(event)

#define TracyCLCollect(ctx) ctx->Collect()

#endif

#endif
