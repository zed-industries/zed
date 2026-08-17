#ifndef __TRACYD3D11_HPP__
#define __TRACYD3D11_HPP__

#ifndef TRACY_ENABLE

#define TracyD3D11Context(device,queue) nullptr
#define TracyD3D11Destroy(ctx)
#define TracyD3D11ContextName(ctx, name, size)

#define TracyD3D11NewFrame(ctx)

#define TracyD3D11Zone(ctx, name)
#define TracyD3D11ZoneC(ctx, name, color)
#define TracyD3D11NamedZone(ctx, varname, name, active)
#define TracyD3D11NamedZoneC(ctx, varname, name, color, active)
#define TracyD3D11ZoneTransient(ctx, varname, name, active)

#define TracyD3D11ZoneS(ctx, name, depth)
#define TracyD3D11ZoneCS(ctx, name, color, depth)
#define TracyD3D11NamedZoneS(ctx, varname, name, depth, active)
#define TracyD3D11NamedZoneCS(ctx, varname, name, color, depth, active)
#define TracyD3D11ZoneTransientS(ctx, varname, name, depth, active)

#define TracyD3D11Collect(ctx)

namespace tracy
{
class D3D11ZoneScope {};
}

using TracyD3D11Ctx = void*;

#else

#include <atomic>
#include <assert.h>
#include <stdlib.h>

#include "Tracy.hpp"
#include "../client/TracyProfiler.hpp"
#include "../client/TracyCallstack.hpp"
#include "../common/TracyYield.hpp"

#include <d3d11.h>

#define TracyD3D11Panic(msg, ...) do { assert(false && "TracyD3D11: " msg); TracyMessageLC("TracyD3D11: " msg, tracy::Color::Red4); __VA_ARGS__; } while(false);

namespace tracy
{

class D3D11Ctx
{
    friend class D3D11ZoneScope;

    static constexpr uint32_t MaxQueries = 64 * 1024;

    enum CollectMode { POLL, BLOCK };

public:
    D3D11Ctx( ID3D11Device* device, ID3D11DeviceContext* devicectx )
    {
        // TODO: consider calling ID3D11Device::GetImmediateContext() instead of passing it as an argument
        m_device = device;
        device->AddRef();
        m_immediateDevCtx = devicectx;
        devicectx->AddRef();

        {
            D3D11_QUERY_DESC desc = { };
            desc.Query = D3D11_QUERY_TIMESTAMP_DISJOINT;
            if (FAILED(m_device->CreateQuery(&desc, &m_disjointQuery)))
            {
                TracyD3D11Panic("unable to create disjoint timestamp query.", return);
            }
        }

        for (ID3D11Query*& query : m_queries)
        {
            D3D11_QUERY_DESC desc = { };
            desc.Query = D3D11_QUERY_TIMESTAMP;
            if (FAILED(m_device->CreateQuery(&desc, &query)))
            {
                TracyD3D11Panic("unable to create timestamp query.", return);
            }
        }

        // Calibrate CPU and GPU timestamps
        int64_t tcpu = 0;
        int64_t tgpu = 0;
        for (int attempts = 0; attempts < 50; attempts++)
        {
            m_immediateDevCtx->Begin(m_disjointQuery);
            m_immediateDevCtx->End(m_queries[0]);
            m_immediateDevCtx->End(m_disjointQuery);

            int64_t tcpu0 = Profiler::GetTime();
            WaitForQuery(m_disjointQuery);
            // NOTE: one would expect that by waiting for the enclosing disjoint query to finish,
            // all timestamp queries within would also be readily available, but that does not
            // seem to be the case here... See https://github.com/wolfpld/tracy/issues/947
            WaitForQuery(m_queries[0]);
            int64_t tcpu1 = Profiler::GetTime();

            D3D11_QUERY_DATA_TIMESTAMP_DISJOINT disjoint = { };
            if (m_immediateDevCtx->GetData(m_disjointQuery, &disjoint, sizeof(disjoint), 0) != S_OK)
            {
                TracyMessageLC("TracyD3D11: unable to query GPU timestamp; retrying...", tracy::Color::Tomato);
                continue;
            }

            if (disjoint.Disjoint)
                continue;

            UINT64 timestamp = 0;
            if (m_immediateDevCtx->GetData(m_queries[0], &timestamp, sizeof(timestamp), 0) != S_OK)
                continue;   // this should never happen (we waited for the query to finish above)

            tcpu = tcpu0 + (tcpu1 - tcpu0) * 1 / 2;
            tgpu = timestamp * (1000000000 / disjoint.Frequency);
            break;
        }

        // ready to roll
        m_contextId = GetGpuCtxCounter().fetch_add(1);
        m_immediateDevCtx->Begin(m_disjointQuery);
        m_previousCheckpoint = m_nextCheckpoint = 0;

        auto* item = Profiler::QueueSerial();
        MemWrite( &item->hdr.type, QueueType::GpuNewContext );
        MemWrite( &item->gpuNewContext.cpuTime, tcpu );
        MemWrite( &item->gpuNewContext.gpuTime, tgpu );
        MemWrite( &item->gpuNewContext.thread, uint32_t(0) );   // #TODO: why not GetThreadHandle()?
        MemWrite( &item->gpuNewContext.period, 1.0f );
        MemWrite( &item->gpuNewContext.context, m_contextId);
        MemWrite( &item->gpuNewContext.flags, uint8_t(0) );
        MemWrite( &item->gpuNewContext.type, GpuContextType::Direct3D11 );

#ifdef TRACY_ON_DEMAND
        GetProfiler().DeferItem( *item );
#endif

        Profiler::QueueSerialFinish();
    }

    ~D3D11Ctx()
    {
        // collect all pending timestamps before destroying everything
        do
        {
            Collect(BLOCK);
        } while (m_previousCheckpoint != m_queryCounter);

        for (ID3D11Query* query : m_queries)
        {
            query->Release();
        }
        m_immediateDevCtx->End(m_disjointQuery);
        m_disjointQuery->Release();
        m_immediateDevCtx->Release();
        m_device->Release();
    }

    void Name( const char* name, uint16_t len )
    {
        auto ptr = (char*)tracy_malloc( len );
        memcpy( ptr, name, len );

        auto item = Profiler::QueueSerial();
        MemWrite( &item->hdr.type, QueueType::GpuContextName );
        MemWrite( &item->gpuContextNameFat.context, m_contextId );
        MemWrite( &item->gpuContextNameFat.ptr, (uint64_t)ptr );
        MemWrite( &item->gpuContextNameFat.size, len );
#ifdef TRACY_ON_DEMAND
        GetProfiler().DeferItem( *item );
#endif
        Profiler::QueueSerialFinish();
    }

    void Collect(CollectMode mode = POLL)
    {
        ZoneScopedC( Color::Red4 );

#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() )
        {
            m_previousCheckpoint = m_nextCheckpoint = m_queryCounter;
            return;
        }
#endif

        if (m_previousCheckpoint == m_nextCheckpoint)
        {
            uintptr_t nextCheckpoint = m_queryCounter;
            if (nextCheckpoint == m_nextCheckpoint)
            {
                return;
            }
            m_nextCheckpoint = nextCheckpoint;
            m_immediateDevCtx->End(m_disjointQuery);
        }

        if (mode == CollectMode::BLOCK)
        {
            WaitForQuery(m_disjointQuery);
        }

        D3D11_QUERY_DATA_TIMESTAMP_DISJOINT disjoint = { };
        if (m_immediateDevCtx->GetData(m_disjointQuery, &disjoint, sizeof(disjoint), D3D11_ASYNC_GETDATA_DONOTFLUSH) != S_OK)
        {
            return;
        }

        if (disjoint.Disjoint == TRUE)
        {
            m_previousCheckpoint = m_nextCheckpoint;
            TracyD3D11Panic("disjoint timestamps detected; dropping.");
            return;
        }

        auto begin = m_previousCheckpoint;
        auto end = m_nextCheckpoint;
        for (auto i = begin; i != end; ++i)
        {
            uint32_t k = RingIndex(i);
            UINT64 timestamp = 0;
            if (m_immediateDevCtx->GetData(m_queries[k], &timestamp, sizeof(timestamp), 0) != S_OK)
            {
                TracyD3D11Panic("timestamp expected to be ready, but it was not!");
                break;
            }
            timestamp *= (1000000000ull / disjoint.Frequency);
            auto* item = Profiler::QueueSerial();
            MemWrite(&item->hdr.type, QueueType::GpuTime);
            MemWrite(&item->gpuTime.gpuTime, static_cast<int64_t>(timestamp));
            MemWrite(&item->gpuTime.queryId, static_cast<uint16_t>(k));
            MemWrite(&item->gpuTime.context, m_contextId);
            Profiler::QueueSerialFinish();
        }

        // disjoint timestamp queries should only be invoked once per frame or less
        // https://learn.microsoft.com/en-us/windows/win32/api/d3d11/ne-d3d11-d3d11_query
        m_immediateDevCtx->Begin(m_disjointQuery);
        m_previousCheckpoint = m_nextCheckpoint;
    }

private:
    tracy_force_inline uint32_t RingIndex(uintptr_t index)
    {
        index %= MaxQueries;
        return static_cast<uint32_t>(index);
    }

    tracy_force_inline uint32_t RingCount(uintptr_t begin, uintptr_t end)
    {
        // wrap-around safe: all unsigned
        uintptr_t count = end - begin;
        return static_cast<uint32_t>(count);
    }

    tracy_force_inline uint32_t NextQueryId()
    {
        auto id = m_queryCounter++;
        if (RingCount(m_previousCheckpoint, id) >= MaxQueries)
        {
            TracyD3D11Panic("too many pending timestamp queries.");
            // #TODO: return some sentinel value; ideally a "hidden" query index
        }
        return RingIndex(id);
    }

    tracy_force_inline ID3D11Query* GetQueryObjectFromId(uint32_t id)
    {
        return m_queries[id];
    }

    tracy_force_inline void WaitForQuery(ID3D11Query* query)
    {
        m_immediateDevCtx->Flush();
        while (m_immediateDevCtx->GetData(query, nullptr, 0, 0) != S_OK)
            YieldThread();  // busy-wait :-( attempt to reduce power usage with _mm_pause() & friends...
    }

    tracy_force_inline uint8_t GetContextId() const
    {
        return m_contextId;
    }

    ID3D11Device* m_device = nullptr;
    ID3D11DeviceContext* m_immediateDevCtx = nullptr;

    ID3D11Query* m_queries[MaxQueries];
    ID3D11Query* m_disjointQuery = nullptr;

    uint8_t m_contextId = 255;  // NOTE: apparently, 255 means invalid id; is this documented anywhere?

    uintptr_t m_queryCounter = 0;

    uintptr_t m_previousCheckpoint = 0;
    uintptr_t m_nextCheckpoint = 0;
};

class D3D11ZoneScope
{
public:
    tracy_force_inline D3D11ZoneScope( D3D11Ctx* ctx, const SourceLocationData* srcloc, bool active )
        : D3D11ZoneScope(ctx, active)
    {
        if( !m_active ) return;

        auto* item = Profiler::QueueSerial();
        WriteQueueItem(item, QueueType::GpuZoneBeginSerial, reinterpret_cast<uint64_t>(srcloc));
    }

    tracy_force_inline D3D11ZoneScope( D3D11Ctx* ctx, const SourceLocationData* srcloc, int32_t depth, bool active )
        : D3D11ZoneScope(ctx, active)
    {
        if( !m_active ) return;

        if( depth > 0 && has_callstack() )
        {
            auto* item = Profiler::QueueSerialCallstack(Callstack(depth));
            WriteQueueItem(item, QueueType::GpuZoneBeginCallstackSerial, reinterpret_cast<uint64_t>(srcloc));
        }
        else
        {
            auto* item = Profiler::QueueSerial();
            WriteQueueItem(item, QueueType::GpuZoneBeginSerial, reinterpret_cast<uint64_t>(srcloc));
        }
    }

    tracy_force_inline D3D11ZoneScope(D3D11Ctx* ctx, uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, bool active)
        : D3D11ZoneScope(ctx, active)
    {
        if( !m_active ) return;

        const auto sourceLocation = Profiler::AllocSourceLocation(line, source, sourceSz, function, functionSz, name, nameSz);

        auto* item = Profiler::QueueSerial();
        WriteQueueItem(item, QueueType::GpuZoneBeginAllocSrcLocSerial, sourceLocation);
    }

    tracy_force_inline D3D11ZoneScope(D3D11Ctx* ctx, uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, int32_t depth, bool active)
        : D3D11ZoneScope(ctx, active)
    {
        if( !m_active ) return;

        const auto sourceLocation = Profiler::AllocSourceLocation(line, source, sourceSz, function, functionSz, name, nameSz);

        if ( depth > 0 && has_callstack() )
        {
            auto* item = Profiler::QueueSerialCallstack(Callstack(depth));
            WriteQueueItem(item, QueueType::GpuZoneBeginAllocSrcLocCallstackSerial, sourceLocation);
        }
        else
        {
            auto* item = Profiler::QueueSerial();
            WriteQueueItem(item, QueueType::GpuZoneBeginAllocSrcLocSerial, sourceLocation);
        }
    }

    tracy_force_inline ~D3D11ZoneScope()
    {
        if( !m_active ) return;

        const auto queryId = m_ctx->NextQueryId();
        m_ctx->m_immediateDevCtx->End(m_ctx->GetQueryObjectFromId(queryId));

        auto* item = Profiler::QueueSerial();
        MemWrite( &item->hdr.type, QueueType::GpuZoneEndSerial );
        MemWrite( &item->gpuZoneEnd.cpuTime, Profiler::GetTime() );
        MemWrite( &item->gpuZoneEnd.thread, GetThreadHandle() );
        MemWrite( &item->gpuZoneEnd.queryId, uint16_t( queryId ) );
        MemWrite( &item->gpuZoneEnd.context, m_ctx->GetContextId() );
        Profiler::QueueSerialFinish();
    }

private:
    tracy_force_inline D3D11ZoneScope( D3D11Ctx* ctx, bool active )
#ifdef TRACY_ON_DEMAND
        : m_active( active && GetProfiler().IsConnected() )
#else
        : m_active( active )
#endif
    {
        if( !m_active ) return;
        m_ctx = ctx;
    }

    void WriteQueueItem(tracy::QueueItem* item, tracy::QueueType queueItemType, uint64_t sourceLocation)
    {
        const auto queryId = m_ctx->NextQueryId();
        m_ctx->m_immediateDevCtx->End(m_ctx->GetQueryObjectFromId(queryId));

        MemWrite( &item->hdr.type, queueItemType);
        MemWrite( &item->gpuZoneBegin.cpuTime, Profiler::GetTime() );
        MemWrite( &item->gpuZoneBegin.srcloc, sourceLocation );
        MemWrite( &item->gpuZoneBegin.thread, GetThreadHandle() );
        MemWrite( &item->gpuZoneBegin.queryId, uint16_t( queryId ) );
        MemWrite( &item->gpuZoneBegin.context, m_ctx->GetContextId() );
        Profiler::QueueSerialFinish();
    }

    const bool m_active;

    D3D11Ctx* m_ctx;
};

static inline D3D11Ctx* CreateD3D11Context( ID3D11Device* device, ID3D11DeviceContext* devicectx )
{
    auto ctx = (D3D11Ctx*)tracy_malloc( sizeof( D3D11Ctx ) );
    new(ctx) D3D11Ctx( device, devicectx );
    return ctx;
}

static inline void DestroyD3D11Context( D3D11Ctx* ctx )
{
    ctx->~D3D11Ctx();
    tracy_free( ctx );
}
}

#undef TracyD3D11Panic

using TracyD3D11Ctx = tracy::D3D11Ctx*;

#define TracyD3D11Context( device, devicectx ) tracy::CreateD3D11Context( device, devicectx );
#define TracyD3D11Destroy(ctx) tracy::DestroyD3D11Context(ctx);
#define TracyD3D11ContextName(ctx, name, size) ctx->Name(name, size);

#define TracyD3D11UnnamedZone ___tracy_gpu_d3d11_zone
#define TracyD3D11SrcLocSymbol TracyConcat(__tracy_gpu_d3d11_source_location,TracyLine)
#define TracyD3D11SrcLocObject(name, color) static constexpr tracy::SourceLocationData TracyD3D11SrcLocSymbol { name, TracyFunction, TracyFile, (uint32_t)TracyLine, color };

#if defined TRACY_HAS_CALLSTACK && defined TRACY_CALLSTACK
#  define TracyD3D11Zone( ctx, name ) TracyD3D11NamedZoneS( ctx, TracyD3D11UnnamedZone, name, TRACY_CALLSTACK, true )
#  define TracyD3D11ZoneC( ctx, name, color ) TracyD3D11NamedZoneCS( ctx, TracyD3D11UnnamedZone, name, color, TRACY_CALLSTACK, true )
#  define TracyD3D11NamedZone( ctx, varname, name, active ) TracyD3D11SrcLocObject(name, 0); tracy::D3D11ZoneScope varname( ctx, &TracyD3D11SrcLocSymbol, TRACY_CALLSTACK, active );
#  define TracyD3D11NamedZoneC( ctx, varname, name, color, active ) TracyD3D11SrcLocObject(name, color); tracy::D3D11ZoneScope varname( ctx, &TracyD3D11SrcLocSymbol, TRACY_CALLSTACK, active );
#  define TracyD3D11ZoneTransient(ctx, varname, name, active) TracyD3D11ZoneTransientS(ctx, varname, cmdList, name, TRACY_CALLSTACK, active)
#else
#  define TracyD3D11Zone( ctx, name ) TracyD3D11NamedZone( ctx, TracyD3D11UnnamedZone, name, true )
#  define TracyD3D11ZoneC( ctx, name, color ) TracyD3D11NamedZoneC( ctx, TracyD3D11UnnamedZone, name, color, true )
#  define TracyD3D11NamedZone( ctx, varname, name, active ) TracyD3D11SrcLocObject(name, 0); tracy::D3D11ZoneScope varname( ctx, &TracyD3D11SrcLocSymbol, active );
#  define TracyD3D11NamedZoneC( ctx, varname, name, color, active ) TracyD3D11SrcLocObject(name, color); tracy::D3D11ZoneScope varname( ctx, &TracyD3D11SrcLocSymbol, active );
#  define TracyD3D11ZoneTransient(ctx, varname, name, active) tracy::D3D11ZoneScope varname{ ctx, TracyLine, TracyFile, strlen(TracyFile), TracyFunction, strlen(TracyFunction), name, strlen(name), active };
#endif

#ifdef TRACY_HAS_CALLSTACK
#  define TracyD3D11ZoneS( ctx, name, depth ) TracyD3D11NamedZoneS( ctx, TracyD3D11UnnamedZone, name, depth, true )
#  define TracyD3D11ZoneCS( ctx, name, color, depth ) TracyD3D11NamedZoneCS( ctx, TracyD3D11UnnamedZone, name, color, depth, true )
#  define TracyD3D11NamedZoneS( ctx, varname, name, depth, active ) TracyD3D11SrcLocObject(name, 0); tracy::D3D11ZoneScope varname( ctx, &TracyD3D11SrcLocSymbol, depth, active );
#  define TracyD3D11NamedZoneCS( ctx, varname, name, color, depth, active ) TracyD3D11SrcLocObject(name, color); tracy::D3D11ZoneScope varname( ctx, &TracyD3D11SrcLocSymbol, depth, active );
#  define TracyD3D11ZoneTransientS(ctx, varname, name, depth, active) tracy::D3D11ZoneScope varname{ ctx, TracyLine, TracyFile, strlen(TracyFile), TracyFunction, strlen(TracyFunction), name, strlen(name), depth, active };
#else
#  define TracyD3D11ZoneS( ctx, name, depth, active ) TracyD3D11Zone( ctx, name )
#  define TracyD3D11ZoneCS( ctx, name, color, depth, active ) TracyD3D11ZoneC( name, color )
#  define TracyD3D11NamedZoneS( ctx, varname, name, depth, active ) TracyD3D11NamedZone( ctx, varname, name, active )
#  define TracyD3D11NamedZoneCS( ctx, varname, name, color, depth, active ) TracyD3D11NamedZoneC( ctx, varname, name, color, active )
#  define TracyD3D11ZoneTransientS(ctx, varname, name, depth, active) TracyD3D11ZoneTransient(ctx, varname, name, active)
#endif

#define TracyD3D11Collect( ctx ) ctx->Collect();

#endif

#endif
