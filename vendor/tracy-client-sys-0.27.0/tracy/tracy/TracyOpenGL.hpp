#ifndef __TRACYOPENGL_HPP__
#define __TRACYOPENGL_HPP__

#if !defined TRACY_ENABLE || defined __APPLE__

#define TracyGpuContext
#define TracyGpuContextName(x,y)
#define TracyGpuNamedZone(x,y,z)
#define TracyGpuNamedZoneC(x,y,z,w)
#define TracyGpuZone(x)
#define TracyGpuZoneC(x,y)
#define TracyGpuZoneTransient(x,y,z)
#define TracyGpuCollect

#define TracyGpuNamedZoneS(x,y,z,w)
#define TracyGpuNamedZoneCS(x,y,z,w,a)
#define TracyGpuZoneS(x,y)
#define TracyGpuZoneCS(x,y,z)
#define TracyGpuZoneTransientS(x,y,z,w)

namespace tracy
{
struct SourceLocationData;
class GpuCtxScope
{
public:
    GpuCtxScope( const SourceLocationData*, bool ) {}
    GpuCtxScope( const SourceLocationData*, int32_t, bool ) {}
};
}

#else

#include <atomic>
#include <assert.h>
#include <stdlib.h>

#include "Tracy.hpp"
#include "../client/TracyProfiler.hpp"
#include "../client/TracyCallstack.hpp"
#include "../common/TracyAlign.hpp"
#include "../common/TracyAlloc.hpp"

#if !defined GL_TIMESTAMP && defined GL_TIMESTAMP_EXT
#  define GL_TIMESTAMP GL_TIMESTAMP_EXT
#  define GL_QUERY_COUNTER_BITS GL_QUERY_COUNTER_BITS_EXT
#  define glGetQueryObjectiv glGetQueryObjectivEXT
#  define glGetQueryObjectui64v glGetQueryObjectui64vEXT
#  define glQueryCounter glQueryCounterEXT
#endif

#define TracyGpuContext tracy::GetGpuCtx().ptr = (tracy::GpuCtx*)tracy::tracy_malloc( sizeof( tracy::GpuCtx ) ); new(tracy::GetGpuCtx().ptr) tracy::GpuCtx;
#define TracyGpuContextName( name, size ) tracy::GetGpuCtx().ptr->Name( name, size );
#if defined TRACY_HAS_CALLSTACK && defined TRACY_CALLSTACK
#  define TracyGpuNamedZone( varname, name, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, 0 }; tracy::GpuCtxScope varname( &TracyConcat(__tracy_gpu_source_location,TracyLine), TRACY_CALLSTACK, active );
#  define TracyGpuNamedZoneC( varname, name, color, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, color }; tracy::GpuCtxScope varname( &TracyConcat(__tracy_gpu_source_location,TracyLine), TRACY_CALLSTACK, active );
#  define TracyGpuZone( name ) TracyGpuNamedZoneS( ___tracy_gpu_zone, name, TRACY_CALLSTACK, true )
#  define TracyGpuZoneC( name, color ) TracyGpuNamedZoneCS( ___tracy_gpu_zone, name, color, TRACY_CALLSTACK, true )
#  define TracyGpuZoneTransient( varname, name, active ) tracy::GpuCtxScope varname( TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), name, strlen( name ), TRACY_CALLSTACK, active );
#else
#  define TracyGpuNamedZone( varname, name, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, 0 }; tracy::GpuCtxScope varname( &TracyConcat(__tracy_gpu_source_location,TracyLine), active );
#  define TracyGpuNamedZoneC( varname, name, color, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, color }; tracy::GpuCtxScope varname( &TracyConcat(__tracy_gpu_source_location,TracyLine), active );
#  define TracyGpuZone( name ) TracyGpuNamedZone( ___tracy_gpu_zone, name, true )
#  define TracyGpuZoneC( name, color ) TracyGpuNamedZoneC( ___tracy_gpu_zone, name, color, true )
#  define TracyGpuZoneTransient( varname, name, active ) tracy::GpuCtxScope varname( TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), name, strlen( name ), active );
#endif
#define TracyGpuCollect tracy::GetGpuCtx().ptr->Collect();

#ifdef TRACY_HAS_CALLSTACK
#  define TracyGpuNamedZoneS( varname, name, depth, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, 0 }; tracy::GpuCtxScope varname( &TracyConcat(__tracy_gpu_source_location,TracyLine), depth, active );
#  define TracyGpuNamedZoneCS( varname, name, color, depth, active ) static constexpr tracy::SourceLocationData TracyConcat(__tracy_gpu_source_location,TracyLine) { name, TracyFunction,  TracyFile, (uint32_t)TracyLine, color }; tracy::GpuCtxScope varname( &TracyConcat(__tracy_gpu_source_location,TracyLine), depth, active );
#  define TracyGpuZoneS( name, depth ) TracyGpuNamedZoneS( ___tracy_gpu_zone, name, depth, true )
#  define TracyGpuZoneCS( name, color, depth ) TracyGpuNamedZoneCS( ___tracy_gpu_zone, name, color, depth, true )
#  define TracyGpuZoneTransientS( varname, name, depth, active ) tracy::GpuCtxScope varname( TracyLine, TracyFile, strlen( TracyFile ), TracyFunction, strlen( TracyFunction ), name, strlen( name ), depth, active );
#else
#  define TracyGpuNamedZoneS( varname, name, depth, active ) TracyGpuNamedZone( varname, name, active )
#  define TracyGpuNamedZoneCS( varname, name, color, depth, active ) TracyGpuNamedZoneC( varname, name, color, active )
#  define TracyGpuZoneS( name, depth ) TracyGpuZone( name )
#  define TracyGpuZoneCS( name, color, depth ) TracyGpuZoneC( name, color )
#  define TracyGpuZoneTransientS( varname, name, depth, active ) TracyGpuZoneTransient( varname, name, active )
#endif

namespace tracy
{

class GpuCtx
{
    friend class GpuCtxScope;

    enum { QueryCount = 64 * 1024 };

public:
    GpuCtx()
        : m_context( GetGpuCtxCounter().fetch_add( 1, std::memory_order_relaxed ) )
        , m_head( 0 )
        , m_tail( 0 )
    {
        assert( m_context != 255 );

        glGenQueries( QueryCount, m_query );

        int64_t tgpu;
        glGetInteger64v( GL_TIMESTAMP, &tgpu );
        int64_t tcpu = Profiler::GetTime();

        GLint bits;
        glGetQueryiv( GL_TIMESTAMP, GL_QUERY_COUNTER_BITS, &bits );

        const float period = 1.f;
        const auto thread = GetThreadHandle();
        TracyLfqPrepare( QueueType::GpuNewContext );
        MemWrite( &item->gpuNewContext.cpuTime, tcpu );
        MemWrite( &item->gpuNewContext.gpuTime, tgpu );
        MemWrite( &item->gpuNewContext.thread, thread );
        MemWrite( &item->gpuNewContext.period, period );
        MemWrite( &item->gpuNewContext.context, m_context );
        MemWrite( &item->gpuNewContext.flags, uint8_t( 0 ) );
        MemWrite( &item->gpuNewContext.type, GpuContextType::OpenGl );

#ifdef TRACY_ON_DEMAND
        GetProfiler().DeferItem( *item );
#endif

        TracyLfqCommit;
    }

    void Name( const char* name, uint16_t len )
    {
        auto ptr = (char*)tracy_malloc( len );
        memcpy( ptr, name, len );

        TracyLfqPrepare( QueueType::GpuContextName );
        MemWrite( &item->gpuContextNameFat.context, m_context );
        MemWrite( &item->gpuContextNameFat.ptr, (uint64_t)ptr );
        MemWrite( &item->gpuContextNameFat.size, len );
#ifdef TRACY_ON_DEMAND
        GetProfiler().DeferItem( *item );
#endif
        TracyLfqCommit;
    }

    void Collect()
    {
        ZoneScopedC( Color::Red4 );

        if( m_tail == m_head ) return;

#ifdef TRACY_ON_DEMAND
        if( !GetProfiler().IsConnected() )
        {
            m_head = m_tail = 0;
            return;
        }
#endif

        while( m_tail != m_head )
        {
            GLint available;
            glGetQueryObjectiv( m_query[m_tail], GL_QUERY_RESULT_AVAILABLE, &available );
            if( !available ) return;

            uint64_t time;
            glGetQueryObjectui64v( m_query[m_tail], GL_QUERY_RESULT, &time );

            TracyLfqPrepare( QueueType::GpuTime );
            MemWrite( &item->gpuTime.gpuTime, (int64_t)time );
            MemWrite( &item->gpuTime.queryId, (uint16_t)m_tail );
            MemWrite( &item->gpuTime.context, m_context );
            TracyLfqCommit;

            m_tail = ( m_tail + 1 ) % QueryCount;
        }
    }

private:
    tracy_force_inline unsigned int NextQueryId()
    {
        const auto id = m_head;
        m_head = ( m_head + 1 ) % QueryCount;
        assert( m_head != m_tail );
        return id;
    }

    tracy_force_inline unsigned int TranslateOpenGlQueryId( unsigned int id )
    {
        return m_query[id];
    }

    tracy_force_inline uint8_t GetId() const
    {
        return m_context;
    }

    unsigned int m_query[QueryCount];
    uint8_t m_context;

    unsigned int m_head;
    unsigned int m_tail;
};

class GpuCtxScope
{
public:
    tracy_force_inline GpuCtxScope( const SourceLocationData* srcloc, bool is_active )
#ifdef TRACY_ON_DEMAND
        : m_active( is_active && GetProfiler().IsConnected() )
#else
        : m_active( is_active )
#endif
    {
        if( !m_active ) return;

        const auto queryId = GetGpuCtx().ptr->NextQueryId();
        glQueryCounter( GetGpuCtx().ptr->TranslateOpenGlQueryId( queryId ), GL_TIMESTAMP );

        TracyLfqPrepare( QueueType::GpuZoneBegin );
        MemWrite( &item->gpuZoneBegin.cpuTime, Profiler::GetTime() );
        memset( &item->gpuZoneBegin.thread, 0, sizeof( item->gpuZoneBegin.thread ) );
        MemWrite( &item->gpuZoneBegin.queryId, uint16_t( queryId ) );
        MemWrite( &item->gpuZoneBegin.context, GetGpuCtx().ptr->GetId() );
        MemWrite( &item->gpuZoneBegin.srcloc, (uint64_t)srcloc );
        TracyLfqCommit;
    }

    tracy_force_inline GpuCtxScope( const SourceLocationData* srcloc, int32_t depth, bool is_active )
#ifdef TRACY_ON_DEMAND
        : m_active( is_active && GetProfiler().IsConnected() )
#else
        : m_active( is_active )
#endif
    {
        if( !m_active ) return;

        const auto queryId = GetGpuCtx().ptr->NextQueryId();
        glQueryCounter( GetGpuCtx().ptr->TranslateOpenGlQueryId( queryId ), GL_TIMESTAMP );

#ifdef TRACY_FIBERS
        TracyLfqPrepare( QueueType::GpuZoneBegin );
        memset( &item->gpuZoneBegin.thread, 0, sizeof( item->gpuZoneBegin.thread ) );
#else
        GetProfiler().SendCallstack( depth );
        TracyLfqPrepare( QueueType::GpuZoneBeginCallstack );
        MemWrite( &item->gpuZoneBegin.thread, GetThreadHandle() );
#endif
        MemWrite( &item->gpuZoneBegin.cpuTime, Profiler::GetTime() );
        MemWrite( &item->gpuZoneBegin.queryId, uint16_t( queryId ) );
        MemWrite( &item->gpuZoneBegin.context, GetGpuCtx().ptr->GetId() );
        MemWrite( &item->gpuZoneBegin.srcloc, (uint64_t)srcloc );
        TracyLfqCommit;
    }

    tracy_force_inline GpuCtxScope( uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, bool is_active )
#ifdef TRACY_ON_DEMAND
        : m_active( is_active && GetProfiler().IsConnected() )
#else
        : m_active( is_active )
#endif
    {
        if( !m_active ) return;

        const auto queryId = GetGpuCtx().ptr->NextQueryId();
        glQueryCounter( GetGpuCtx().ptr->TranslateOpenGlQueryId( queryId ), GL_TIMESTAMP );

        TracyLfqPrepare( QueueType::GpuZoneBeginAllocSrcLoc );
        const auto srcloc = Profiler::AllocSourceLocation( line, source, sourceSz, function, functionSz, name, nameSz );
        MemWrite( &item->gpuZoneBegin.cpuTime, Profiler::GetTime() );
        memset( &item->gpuZoneBegin.thread, 0, sizeof( item->gpuZoneBegin.thread ) );
        MemWrite( &item->gpuZoneBegin.queryId, uint16_t( queryId ) );
        MemWrite( &item->gpuZoneBegin.context, GetGpuCtx().ptr->GetId() );
        MemWrite( &item->gpuZoneBegin.srcloc, (uint64_t)srcloc );
        TracyLfqCommit;
    }

    tracy_force_inline GpuCtxScope( uint32_t line, const char* source, size_t sourceSz, const char* function, size_t functionSz, const char* name, size_t nameSz, int32_t depth, bool is_active )
#ifdef TRACY_ON_DEMAND
        : m_active( is_active && GetProfiler().IsConnected() )
#else
        : m_active( is_active )
#endif
    {
        if( !m_active ) return;

        const auto queryId = GetGpuCtx().ptr->NextQueryId();
        glQueryCounter( GetGpuCtx().ptr->TranslateOpenGlQueryId( queryId ), GL_TIMESTAMP );

#ifdef TRACY_FIBERS
        TracyLfqPrepare( QueueType::GpuZoneBeginAllocSrcLoc );
        memset( &item->gpuZoneBegin.thread, 0, sizeof( item->gpuZoneBegin.thread ) );
#else
        GetProfiler().SendCallstack( depth );
        TracyLfqPrepare( QueueType::GpuZoneBeginAllocSrcLocCallstack );
        MemWrite( &item->gpuZoneBegin.thread, GetThreadHandle() );
#endif
        const auto srcloc = Profiler::AllocSourceLocation( line, source, sourceSz, function, functionSz, name, nameSz );
        MemWrite( &item->gpuZoneBegin.cpuTime, Profiler::GetTime() );
        MemWrite( &item->gpuZoneBegin.queryId, uint16_t( queryId ) );
        MemWrite( &item->gpuZoneBegin.context, GetGpuCtx().ptr->GetId() );
        MemWrite( &item->gpuZoneBegin.srcloc, (uint64_t)srcloc );
        TracyLfqCommit;
    }

    tracy_force_inline ~GpuCtxScope()
    {
        if( !m_active ) return;

        const auto queryId = GetGpuCtx().ptr->NextQueryId();
        glQueryCounter( GetGpuCtx().ptr->TranslateOpenGlQueryId( queryId ), GL_TIMESTAMP );

        TracyLfqPrepare( QueueType::GpuZoneEnd );
        MemWrite( &item->gpuZoneEnd.cpuTime, Profiler::GetTime() );
        memset( &item->gpuZoneEnd.thread, 0, sizeof( item->gpuZoneEnd.thread ) );
        MemWrite( &item->gpuZoneEnd.queryId, uint16_t( queryId ) );
        MemWrite( &item->gpuZoneEnd.context, GetGpuCtx().ptr->GetId() );
        TracyLfqCommit;
    }

private:
    const bool m_active;
};

}

#endif

#endif
