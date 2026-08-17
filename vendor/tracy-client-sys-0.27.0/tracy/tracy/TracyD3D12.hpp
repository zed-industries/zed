#ifndef __TRACYD3D12_HPP__
#define __TRACYD3D12_HPP__

#ifndef TRACY_ENABLE

#define TracyD3D12Context(device, queue) nullptr
#define TracyD3D12Destroy(ctx)
#define TracyD3D12ContextName(ctx, name, size)

#define TracyD3D12NewFrame(ctx)

#define TracyD3D12Zone(ctx, cmdList, name)
#define TracyD3D12ZoneC(ctx, cmdList, name, color)
#define TracyD3D12NamedZone(ctx, varname, cmdList, name, active)
#define TracyD3D12NamedZoneC(ctx, varname, cmdList, name, color, active)
#define TracyD3D12ZoneTransient(ctx, varname, cmdList, name, active)

#define TracyD3D12ZoneS(ctx, cmdList, name, depth)
#define TracyD3D12ZoneCS(ctx, cmdList, name, color, depth)
#define TracyD3D12NamedZoneS(ctx, varname, cmdList, name, depth, active)
#define TracyD3D12NamedZoneCS(ctx, varname, cmdList, name, color, depth, active)
#define TracyD3D12ZoneTransientS(ctx, varname, cmdList, name, depth, active)

#define TracyD3D12Collect(ctx)

namespace tracy
{
    class D3D12ZoneScope {};
}

using TracyD3D12Ctx = void*;

#else

#include "Tracy.hpp"
#include "../client/TracyProfiler.hpp"
#include "../client/TracyCallstack.hpp"

#include <cstdlib>
#include <cassert>
#include <d3d12.h>
#include <dxgi.h>
#include <queue>

#define TracyD3D12Panic(msg, ...) do { assert(false && "TracyD3D12: " msg); TracyMessageLC("TracyD3D12: " msg, tracy::Color::Red4); __VA_ARGS__; } while(false);

namespace tracy
{

    struct D3D12QueryPayload
    {
        uint32_t m_queryIdStart = 0;
        uint32_t m_queryCount = 0;
    };

    // Command queue context.
    class D3D12QueueCtx
    {
        friend class D3D12ZoneScope;

        ID3D12Device* m_device = nullptr;
        ID3D12CommandQueue* m_queue = nullptr;
        uint8_t m_contextId = 255;  // TODO: apparently, 255 means "invalid id"; is this documented somewhere?
        ID3D12QueryHeap* m_queryHeap = nullptr;
        ID3D12Resource* m_readbackBuffer = nullptr;

        // In-progress payload.
        uint32_t m_queryLimit = 0;
        std::atomic<uint32_t> m_queryCounter = 0;
        uint32_t m_previousQueryCounter = 0;

        uint32_t m_activePayload = 0;
        ID3D12Fence* m_payloadFence = nullptr;
        std::queue<D3D12QueryPayload> m_payloadQueue;

        UINT64 m_prevCalibrationTicksCPU = 0;

        void RecalibrateClocks()
        {
            UINT64 cpuTimestamp;
            UINT64 gpuTimestamp;
            if (FAILED(m_queue->GetClockCalibration(&gpuTimestamp, &cpuTimestamp)))
            {
                TracyD3D12Panic("failed to obtain queue clock calibration counters.", return);
            }

            int64_t cpuDeltaTicks = cpuTimestamp - m_prevCalibrationTicksCPU;
            if (cpuDeltaTicks > 0)
            {
                static const int64_t nanosecodsPerTick = int64_t(1000000000) / GetFrequencyQpc();
                int64_t cpuDeltaNS = cpuDeltaTicks * nanosecodsPerTick;
                // Save the device cpu timestamp, not the Tracy profiler timestamp:
                m_prevCalibrationTicksCPU = cpuTimestamp;

                cpuTimestamp = Profiler::GetTime();

                auto* item = Profiler::QueueSerial();
                MemWrite(&item->hdr.type, QueueType::GpuCalibration);
                MemWrite(&item->gpuCalibration.gpuTime, gpuTimestamp);
                MemWrite(&item->gpuCalibration.cpuTime, cpuTimestamp);
                MemWrite(&item->gpuCalibration.cpuDelta, cpuDeltaNS);
                MemWrite(&item->gpuCalibration.context, GetId());
                SubmitQueueItem(item);
            }
        }

        tracy_force_inline void SubmitQueueItem(tracy::QueueItem* item)
        {
#ifdef TRACY_ON_DEMAND
            GetProfiler().DeferItem(*item);
#endif
            Profiler::QueueSerialFinish();
        }

    public:
        D3D12QueueCtx(ID3D12Device* device, ID3D12CommandQueue* queue)
            : m_device(device)
            , m_queue(queue)
        {
            // Verify we support timestamp queries on this queue.

            if (queue->GetDesc().Type == D3D12_COMMAND_LIST_TYPE_COPY)
            {
                D3D12_FEATURE_DATA_D3D12_OPTIONS3 featureData{};

                HRESULT hr = device->CheckFeatureSupport(D3D12_FEATURE_D3D12_OPTIONS3, &featureData, sizeof(featureData));
                if (FAILED(hr) || (featureData.CopyQueueTimestampQueriesSupported == FALSE))
                {
                    TracyD3D12Panic("Platform does not support profiling of copy queues.", return);
                }
            }

            static constexpr uint32_t MaxQueries = 64 * 1024;  // Must be even, because queries are (begin, end) pairs
            m_queryLimit = MaxQueries;

            D3D12_QUERY_HEAP_DESC heapDesc{};
            heapDesc.Type = queue->GetDesc().Type == D3D12_COMMAND_LIST_TYPE_COPY ? D3D12_QUERY_HEAP_TYPE_COPY_QUEUE_TIMESTAMP : D3D12_QUERY_HEAP_TYPE_TIMESTAMP;
            heapDesc.Count = m_queryLimit;
            heapDesc.NodeMask = 0;  // #TODO: Support multiple adapters.

            while (FAILED(device->CreateQueryHeap(&heapDesc, IID_PPV_ARGS(&m_queryHeap))))
            {
                m_queryLimit /= 2;
                heapDesc.Count = m_queryLimit;
            }

            // Create a readback buffer, which will be used as a destination for the query data.

            D3D12_RESOURCE_DESC readbackBufferDesc{};
            readbackBufferDesc.Alignment = 0;
            readbackBufferDesc.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
            readbackBufferDesc.Width = m_queryLimit * sizeof(uint64_t);
            readbackBufferDesc.Height = 1;
            readbackBufferDesc.DepthOrArraySize = 1;
            readbackBufferDesc.Format = DXGI_FORMAT_UNKNOWN;
            readbackBufferDesc.Layout = D3D12_TEXTURE_LAYOUT_ROW_MAJOR;  // Buffers are always row major.
            readbackBufferDesc.MipLevels = 1;
            readbackBufferDesc.SampleDesc.Count = 1;
            readbackBufferDesc.SampleDesc.Quality = 0;
            readbackBufferDesc.Flags = D3D12_RESOURCE_FLAG_NONE;

            D3D12_HEAP_PROPERTIES readbackHeapProps{};
            readbackHeapProps.Type = D3D12_HEAP_TYPE_READBACK;
            readbackHeapProps.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_UNKNOWN;
            readbackHeapProps.MemoryPoolPreference = D3D12_MEMORY_POOL_UNKNOWN;
            readbackHeapProps.CreationNodeMask = 0;
            readbackHeapProps.VisibleNodeMask = 0;  // #TODO: Support multiple adapters.

            if (FAILED(device->CreateCommittedResource(&readbackHeapProps, D3D12_HEAP_FLAG_NONE, &readbackBufferDesc, D3D12_RESOURCE_STATE_COPY_DEST, nullptr, IID_PPV_ARGS(&m_readbackBuffer))))
            {
                TracyD3D12Panic("Failed to create query readback buffer.", return);
            }

            if (FAILED(device->CreateFence(0, D3D12_FENCE_FLAG_NONE, IID_PPV_ARGS(&m_payloadFence))))
            {
                TracyD3D12Panic("Failed to create payload fence.", return);
            }

            float period = [queue]()
            {
                uint64_t timestampFrequency;
                if (FAILED(queue->GetTimestampFrequency(&timestampFrequency)))
                {
                    return 0.0f;
                }
                return static_cast<float>( 1E+09 / static_cast<double>(timestampFrequency) );
            }();

            if (period == 0.0f)
            {
                TracyD3D12Panic("Failed to get timestamp frequency.", return);
            }

            uint64_t cpuTimestamp;
            uint64_t gpuTimestamp;
            if (FAILED(queue->GetClockCalibration(&gpuTimestamp, &cpuTimestamp)))
            {
                TracyD3D12Panic("Failed to get queue clock calibration.", return);
            }

            // Save the device cpu timestamp, not the profiler's timestamp.
            m_prevCalibrationTicksCPU = cpuTimestamp;

            cpuTimestamp = Profiler::GetTime();

            // all checked: ready to roll
            m_contextId = GetGpuCtxCounter().fetch_add(1);

            auto* item = Profiler::QueueSerial();
            MemWrite(&item->hdr.type, QueueType::GpuNewContext);
            MemWrite(&item->gpuNewContext.cpuTime, cpuTimestamp);
            MemWrite(&item->gpuNewContext.gpuTime, gpuTimestamp);
            MemWrite(&item->gpuNewContext.thread, decltype(item->gpuNewContext.thread)(0)); // #TODO: why 0 instead of GetThreadHandle()?
            MemWrite(&item->gpuNewContext.period, period);
            MemWrite(&item->gpuNewContext.context, GetId());
            MemWrite(&item->gpuNewContext.flags, GpuContextCalibration);
            MemWrite(&item->gpuNewContext.type, GpuContextType::Direct3D12);
            SubmitQueueItem(item);
        }

        ~D3D12QueueCtx()
        {
            ZoneScopedC(Color::Red4);
            // collect all pending timestamps
            while (m_payloadFence->GetCompletedValue() != m_activePayload)
                /* busy-wait ... */;
            Collect();
            m_payloadFence->Release();
            m_readbackBuffer->Release();
            m_queryHeap->Release();
        }


        void NewFrame()
        {
            uint32_t queryCounter = m_queryCounter.exchange(0);
            m_payloadQueue.emplace(D3D12QueryPayload{ m_previousQueryCounter, queryCounter });
            m_previousQueryCounter += queryCounter;

            if (m_previousQueryCounter >= m_queryLimit)
            {
                m_previousQueryCounter -= m_queryLimit;
            }

            m_queue->Signal(m_payloadFence, ++m_activePayload);
        }

        void Name( const char* name, uint16_t len )
        {
            auto ptr = (char*)tracy_malloc( len );
            memcpy( ptr, name, len );

            auto item = Profiler::QueueSerial();
            MemWrite( &item->hdr.type, QueueType::GpuContextName );
            MemWrite( &item->gpuContextNameFat.context, GetId());
            MemWrite( &item->gpuContextNameFat.ptr, (uint64_t)ptr );
            MemWrite( &item->gpuContextNameFat.size, len );
            SubmitQueueItem(item);
        }

        void Collect()
        {
            ZoneScopedC(Color::Red4);

#ifdef TRACY_ON_DEMAND
            if (!GetProfiler().IsConnected())
            {
                m_queryCounter = 0;

                return;
            }
#endif

            // Find out what payloads are available.
            const auto newestReadyPayload = m_payloadFence->GetCompletedValue();
            const auto payloadCount = m_payloadQueue.size() - (m_activePayload - newestReadyPayload);

            if (!payloadCount)
            {
                return;  // No payloads are available yet, exit out.
            }

            D3D12_RANGE mapRange{ 0, m_queryLimit * sizeof(uint64_t) };

            // Map the readback buffer so we can fetch the query data from the GPU.
            void* readbackBufferMapping = nullptr;

            if (FAILED(m_readbackBuffer->Map(0, &mapRange, &readbackBufferMapping)))
            {
                TracyD3D12Panic("Failed to map readback buffer.", return);
            }

            auto* timestampData = static_cast<uint64_t*>(readbackBufferMapping);

            for (uint32_t i = 0; i < payloadCount; ++i)
            {
                const auto& payload = m_payloadQueue.front();

                for (uint32_t j = 0; j < payload.m_queryCount; ++j)
                {
                    const auto counter = (payload.m_queryIdStart + j) % m_queryLimit;
                    const auto timestamp = timestampData[counter];
                    const auto queryId = counter;

                    auto* item = Profiler::QueueSerial();
                    MemWrite(&item->hdr.type, QueueType::GpuTime);
                    MemWrite(&item->gpuTime.gpuTime, timestamp);
                    MemWrite(&item->gpuTime.queryId, static_cast<uint16_t>(queryId));
                    MemWrite(&item->gpuTime.context, GetId());

                    Profiler::QueueSerialFinish();
                }

                m_payloadQueue.pop();
            }

            m_readbackBuffer->Unmap(0, nullptr);

            // Recalibrate to account for drift.
            RecalibrateClocks();
        }

    private:
        tracy_force_inline uint32_t NextQueryId()
        {
            uint32_t queryCounter = m_queryCounter.fetch_add(2);
            if (queryCounter >= m_queryLimit)
            {
                TracyD3D12Panic("Submitted too many GPU queries! Consider increasing MaxQueries.");
                // #TODO: consider returning an invalid id or sentinel value here
            }

            const uint32_t id = (m_previousQueryCounter + queryCounter) % m_queryLimit;

            return id;
        }

        tracy_force_inline uint8_t GetId() const
        {
            return m_contextId;
        }
    };

    class D3D12ZoneScope
    {
        const bool m_active;
        D3D12QueueCtx* m_ctx = nullptr;
        ID3D12GraphicsCommandList* m_cmdList = nullptr;
        uint32_t m_queryId = 0;  // Used for tracking in nested zones.

        tracy_force_inline void WriteQueueItem(QueueItem* item, QueueType type, uint64_t srcLocation)
        {
            MemWrite(&item->hdr.type, type);
            MemWrite(&item->gpuZoneBegin.cpuTime, Profiler::GetTime());
            MemWrite(&item->gpuZoneBegin.srcloc, srcLocation);
            MemWrite(&item->gpuZoneBegin.thread, GetThreadHandle());
            MemWrite(&item->gpuZoneBegin.queryId, static_cast<uint16_t>(m_queryId));
            MemWrite(&item->gpuZoneBegin.context, m_ctx->GetId());
            Profiler::QueueSerialFinish();
        }

        tracy_force_inline D3D12ZoneScope(D3D12QueueCtx* ctx, ID3D12GraphicsCommandList* cmdList, bool active)
#ifdef TRACY_ON_DEMAND
            : m_active(active&& GetProfiler().IsConnected())
#else
            : m_active(active)
#endif
        {
            if (!m_active) return;

            m_ctx = ctx;
            m_cmdList = cmdList;

            m_queryId = m_ctx->NextQueryId();
            m_cmdList->EndQuery(m_ctx->m_queryHeap, D3D12_QUERY_TYPE_TIMESTAMP, m_queryId);
        }

    public:
        tracy_force_inline D3D12ZoneScope(D3D12QueueCtx* ctx, ID3D12GraphicsCommandList* cmdList, const SourceLocationData* srcLocation, bool active)
            : D3D12ZoneScope(ctx, cmdList, active)
        {
            if (!m_active) return;

            auto* item = Profiler::QueueSerial();
            WriteQueueItem(item, QueueType::GpuZoneBeginSerial, reinterpret_cast<uint64_t>(srcLocation));
        }

        tracy_force_inline D3D12ZoneScope(D3D12QueueCtx* ctx, ID3D12GraphicsCommandList* cmdList, const SourceLocationData* srcLocation, int32_t depth, bool active)
            : D3D12ZoneScope(ctx, cmdList, active)
        {
            if (!m_active) return;

            auto* item = Profiler::QueueSerialCallstack(Callstack(depth));
            WriteQueueItem(item, QueueType::GpuZoneBeginCallstackSerial, reinterpret_cast<uint64_t>(srcLocation));
        }

        tracy_force_inline D3D12ZoneScope(D3D12QueueCtx* ctx, uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, ID3D12GraphicsCommandList* cmdList, bool active)
            : D3D12ZoneScope(ctx, cmdList, active)
        {
            if (!m_active) return;

            const auto sourceLocation = Profiler::AllocSourceLocation(line, source, sourceSz, function, functionSz, name, nameSz);

            auto* item = Profiler::QueueSerial();
            WriteQueueItem(item, QueueType::GpuZoneBeginAllocSrcLocSerial, sourceLocation);
        }

        tracy_force_inline D3D12ZoneScope(D3D12QueueCtx* ctx, uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, ID3D12GraphicsCommandList* cmdList, int32_t depth, bool active)
            : D3D12ZoneScope(ctx, cmdList, active)
        {
            if (!m_active) return;

            const auto sourceLocation = Profiler::AllocSourceLocation(line, source, sourceSz, function, functionSz, name, nameSz);

            auto* item = Profiler::QueueSerialCallstack(Callstack(depth));
            WriteQueueItem(item, QueueType::GpuZoneBeginAllocSrcLocCallstackSerial, sourceLocation);
        }

        tracy_force_inline ~D3D12ZoneScope()
        {
            if (!m_active) return;

            const auto queryId = m_queryId + 1;  // Our end query slot is immediately after the begin slot.
            m_cmdList->EndQuery(m_ctx->m_queryHeap, D3D12_QUERY_TYPE_TIMESTAMP, queryId);

            auto* item = Profiler::QueueSerial();
            MemWrite(&item->hdr.type, QueueType::GpuZoneEndSerial);
            MemWrite(&item->gpuZoneEnd.cpuTime, Profiler::GetTime());
            MemWrite(&item->gpuZoneEnd.thread, GetThreadHandle());
            MemWrite(&item->gpuZoneEnd.queryId, static_cast<uint16_t>(queryId));
            MemWrite(&item->gpuZoneEnd.context, m_ctx->GetId());
            Profiler::QueueSerialFinish();

            m_cmdList->ResolveQueryData(m_ctx->m_queryHeap, D3D12_QUERY_TYPE_TIMESTAMP, m_queryId, 2, m_ctx->m_readbackBuffer, m_queryId * sizeof(uint64_t));
        }
    };

    static inline D3D12QueueCtx* CreateD3D12Context(ID3D12Device* device, ID3D12CommandQueue* queue)
    {
        auto* ctx = static_cast<D3D12QueueCtx*>(tracy_malloc(sizeof(D3D12QueueCtx)));
        new (ctx) D3D12QueueCtx{ device, queue };

        return ctx;
    }

    static inline void DestroyD3D12Context(D3D12QueueCtx* ctx)
    {
        ctx->~D3D12QueueCtx();
        tracy_free(ctx);
    }

}

#undef TracyD3D12Panic

using TracyD3D12Ctx = tracy::D3D12QueueCtx*;

#define TracyD3D12Context(device, queue) tracy::CreateD3D12Context(device, queue);
#define TracyD3D12Destroy(ctx) tracy::DestroyD3D12Context(ctx);
#define TracyD3D12ContextName(ctx, name, size) ctx->Name(name, size);

#define TracyD3D12NewFrame(ctx) ctx->NewFrame();

#define TracyD3D12UnnamedZone ___tracy_gpu_d3d12_zone
#define TracyD3D12SrcLocSymbol TracyConcat(__tracy_d3d12_source_location,TracyLine)
#define TracyD3D12SrcLocObject(name, color) static constexpr tracy::SourceLocationData TracyD3D12SrcLocSymbol { name, TracyFunction, TracyFile, (uint32_t)TracyLine, color };

#if defined TRACY_HAS_CALLSTACK && defined TRACY_CALLSTACK
#  define TracyD3D12Zone(ctx, cmdList, name) TracyD3D12NamedZoneS(ctx, TracyD3D12UnnamedZone, cmdList, name, TRACY_CALLSTACK, true)
#  define TracyD3D12ZoneC(ctx, cmdList, name, color) TracyD3D12NamedZoneCS(ctx, TracyD3D12UnnamedZone, cmdList, name, color, TRACY_CALLSTACK, true)
#  define TracyD3D12NamedZone(ctx, varname, cmdList, name, active) TracyD3D12SrcLocObject(name, 0); tracy::D3D12ZoneScope varname{ ctx, cmdList, &TracyD3D12SrcLocSymbol, TRACY_CALLSTACK, active };
#  define TracyD3D12NamedZoneC(ctx, varname, cmdList, name, color, active) TracyD3D12SrcLocObject(name, color); tracy::D3D12ZoneScope varname{ ctx, cmdList, &TracyD3D12SrcLocSymbol, TRACY_CALLSTACK, active };
#  define TracyD3D12ZoneTransient(ctx, varname, cmdList, name, active) TracyD3D12ZoneTransientS(ctx, varname, cmdList, name, TRACY_CALLSTACK, active)
#else
#  define TracyD3D12Zone(ctx, cmdList, name) TracyD3D12NamedZone(ctx, TracyD3D12UnnamedZone, cmdList, name, true)
#  define TracyD3D12ZoneC(ctx, cmdList, name, color) TracyD3D12NamedZoneC(ctx, TracyD3D12UnnamedZone, cmdList, name, color, true)
#  define TracyD3D12NamedZone(ctx, varname, cmdList, name, active) TracyD3D12SrcLocObject(name, 0); tracy::D3D12ZoneScope varname{ ctx, cmdList, &TracyD3D12SrcLocSymbol, active };
#  define TracyD3D12NamedZoneC(ctx, varname, cmdList, name, color, active) TracyD3D12SrcLocObject(name, color); tracy::D3D12ZoneScope varname{ ctx, cmdList, &TracyD3D12SrcLocSymbol, active };
#  define TracyD3D12ZoneTransient(ctx, varname, cmdList, name, active) tracy::D3D12ZoneScope varname{ ctx, TracyLine, TracyFile, strlen(TracyFile), TracyFunction, strlen(TracyFunction), name, strlen(name), cmdList, active };
#endif

#ifdef TRACY_HAS_CALLSTACK
#  define TracyD3D12ZoneS(ctx, cmdList, name, depth) TracyD3D12NamedZoneS(ctx, TracyD3D12UnnamedZone, cmdList, name, depth, true)
#  define TracyD3D12ZoneCS(ctx, cmdList, name, color, depth) TracyD3D12NamedZoneCS(ctx, TracyD3D12UnnamedZone, cmdList, name, color, depth, true)
#  define TracyD3D12NamedZoneS(ctx, varname, cmdList, name, depth, active) TracyD3D12SrcLocObject(name, 0); tracy::D3D12ZoneScope varname{ ctx, cmdList, &TracyD3D12SrcLocSymbol, depth, active };
#  define TracyD3D12NamedZoneCS(ctx, varname, cmdList, name, color, depth, active) TracyD3D12SrcLocObject(name, color); tracy::D3D12ZoneScope varname{ ctx, cmdList, &TracyD3D12SrcLocSymbol, depth, active };
#  define TracyD3D12ZoneTransientS(ctx, varname, cmdList, name, depth, active) tracy::D3D12ZoneScope varname{ ctx, TracyLine, TracyFile, strlen(TracyFile), TracyFunction, strlen(TracyFunction), name, strlen(name), cmdList, depth, active };
#else
#  define TracyD3D12ZoneS(ctx, cmdList, name, depth) TracyD3D12Zone(ctx, cmdList, name)
#  define TracyD3D12ZoneCS(ctx, cmdList, name, color, depth) TracyD3D12Zone(ctx, cmdList, name, color)
#  define TracyD3D12NamedZoneS(ctx, varname, cmdList, name, depth, active) TracyD3D12NamedZone(ctx, varname, cmdList, name, active)
#  define TracyD3D12NamedZoneCS(ctx, varname, cmdList, name, color, depth, active) TracyD3D12NamedZoneC(ctx, varname, cmdList, name, color, active)
#  define TracyD3D12ZoneTransientS(ctx, varname, cmdList, name, depth, active) TracyD3D12ZoneTransient(ctx, varname, cmdList, name, active)
#endif

#define TracyD3D12Collect(ctx) ctx->Collect();

#endif

#endif
